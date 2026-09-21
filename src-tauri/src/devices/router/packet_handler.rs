use crate::events::{EventKind, TableView};
use crate::net::arp::{Arp, ArpOperations};
use crate::net::ethernet::{Ethernet, EtherTypes};
use crate::net::icmp::{self, Icmp};
use crate::net::ipv4::{IpNextHeaderProtocol, Ipv4};
use crate::utils::addrs::ip_addr::Ipv4Addr;
use crate::utils::addrs::mac_addr::MacAddr;
use crate::devices::router::{Router, PendingForward};


impl Router {
    pub(crate) fn arp_table_view(&self) -> TableView {
        let mut rows = Vec::new();
        for (ip, mac, age) in self.arp_table.entries() {
            rows.push(vec![ip.to_string(), mac.to_string(), format!("{}s", age.as_secs())]);
        }
        TableView {
            name: "ARP Table".into(),
            headers: vec!["IP".into(), "MAC".into(), "Age".into()],
            rows,
        }
    }

    pub(crate) fn routing_table_view(&self) -> TableView {
        let mut rows = Vec::new();
        for route in self.routing_table.entries() {
            let prefix_len = route.mask.count_ones();
            rows.push(vec![
                format!("{}/{}", Ipv4Addr::from_u32(route.network), prefix_len),
                route.next_hop.to_string(),
                route.iface_name.clone(),
            ]);
        }
        TableView {
            name: "Routing Table".into(),
            headers: vec!["Network".into(), "Next Hop".into(), "Interface".into()],
            rows,
        }
    }

    pub(crate) async fn interface_ip(&self, iface_id: usize) -> Option<Ipv4Addr> {
        let ifaces = self.ifaces.lock().await;
        let iface = ifaces.get(iface_id)?;
        Some(Ipv4Addr::from_u32(iface.addr))
    }

    pub(crate) async fn interface_mac(&self, iface_id: usize) -> Option<MacAddr> {
        let ifaces = self.ifaces.lock().await;
        let iface = ifaces.get(iface_id)?;
        Some(MacAddr::from_bytes(&iface.mac).unwrap())
    }

    pub(crate) async fn is_local_ip(&self, ip: Ipv4Addr) -> bool {
        let ifaces = self.ifaces.lock().await;
        let found = ifaces.iter().any(|iface| iface.addr == u32::from(ip));
        found
    }

    pub(crate) async fn handle_packet(&mut self, in_idx: usize, packet: Vec<u8>) {
        let frame = match Ethernet::from_bytes(&packet) {
            Ok(f) => f,
            Err(_) => return,
        };

        match frame.ethertype {
            EtherTypes::Arp => self.handle_arp(in_idx, frame).await,
            EtherTypes::Ipv4 => self.handle_ipv4(in_idx, frame).await,
            _ => {}
        }
    }

    async fn handle_arp(&mut self, in_idx: usize, frame: Ethernet) {
        let arp = match Arp::from_bytes(&frame.payload) {
            Ok(a) => a,
            Err(_) => return,
        };

        self.arp_table.learn(arp.sender_ip, arp.sender_mac);
        self.events.step(
            &self.name,
            EventKind::ArpTableUpdate { ip: arp.sender_ip, mac: arp.sender_mac },
            vec![self.arp_table_view()],
        ).await;

        match arp.operation {
            ArpOperations::Request => {
                let my_mac = {
                    let ifaces = self.ifaces.lock().await;
                    let iface = match ifaces.get(in_idx) {
                        Some(i) => i,
                        None => return,
                    };
                    if iface.addr != u32::from(arp.target_ip) {
                        return;
                    }
                    MacAddr::from_bytes(&iface.mac).unwrap()
                };

                self.events.step(
                    &self.name,
                    EventKind::ArpReplySent { target_ip: arp.sender_ip, mac: my_mac },
                    vec![self.arp_table_view()],
                ).await;

                let reply = Arp::new(
                    ArpOperations::Reply,
                    my_mac, arp.target_ip,
                    arp.sender_mac, arp.sender_ip,
                );
                let frame = Ethernet::new(
                    arp.sender_mac, my_mac,
                    EtherTypes::Arp, reply.to_bytes(),
                );
                let ifaces = self.ifaces.lock().await;
                ifaces.send_on(in_idx, frame.to_bytes()).await;
            }
            ArpOperations::Reply => {
                self.events.step(
                    &self.name,
                    EventKind::ArpReplyReceived { sender_ip: arp.sender_ip, sender_mac: arp.sender_mac },
                    vec![self.arp_table_view()],
                ).await;
                self.flush_pending(arp.sender_ip).await;
            }
            _ => {}
        }
    }

    async fn handle_ipv4(&mut self, in_idx: usize, frame: Ethernet) {
        let mut packet = match Ipv4::from_bytes(&frame.payload) {
            Ok(p) => p,
            Err(_) => return,
        };

        if !packet.verify_checksum() {
            return;
        }

        if self.is_local_ip(packet.destination).await {
            if packet.next_level_protocol == IpNextHeaderProtocol(1) {
                self.handle_icmp(in_idx, frame, packet).await;
            }
            return;
        }

        if packet.ttl <= 1 {
            self.events.step(
                &self.name,
                EventKind::TtlExpired { src: packet.source, dst: packet.destination },
                vec![self.routing_table_view()],
            ).await;
            self.send_icmp_error(in_idx, &frame, &packet, icmp::TIME_EXCEEDED, icmp::TTL_EXCEEDED).await;
            return;
        }

        packet.ttl -= 1;
        packet.recompute_checksum();

        let route = match self.routing_table.lookup(packet.destination) {
            Some(r) => r.clone(),
            None => {
                self.events.step(
                    &self.name,
                    EventKind::RouteNotFound { dst: packet.destination },
                    vec![self.routing_table_view()],
                ).await;
                return;
            }
        };

        self.events.step(
            &self.name,
            EventKind::RouteFound {
                dst: packet.destination,
                next_hop: route.next_hop,
                iface: route.iface_name.clone(),
            },
            vec![self.routing_table_view(), self.arp_table_view()],
        ).await;

        self.forward(route.iface_id, route.next_hop, packet).await;
    }

    async fn handle_icmp(&mut self, in_idx: usize, frame: Ethernet, packet: Ipv4) {
        let icmp_pkt = match Icmp::from_bytes(&packet.payload) {
            Ok(i) => i,
            Err(_) => return,
        };

        if !icmp_pkt.verify_checksum() {
            return;
        }

        if icmp_pkt.is_echo_request() {
            self.events.step(
                &self.name,
                EventKind::IcmpEchoRequest { src: packet.source, dst: packet.destination },
                vec![self.arp_table_view()],
            ).await;

            let reply = Icmp::new(icmp::ECHO_REPLY, 0, icmp_pkt.payload);
            let ipv4 = Ipv4::new(
                packet.destination, packet.source,
                IpNextHeaderProtocol(1), reply.to_bytes(),
            );
            let eth = Ethernet::new(
                frame.source, frame.destination,
                EtherTypes::Ipv4, ipv4.to_bytes(),
            );

            self.events.step(
                &self.name,
                EventKind::IcmpEchoReply { src: packet.destination, dst: packet.source },
                vec![self.arp_table_view()],
            ).await;

            let ifaces = self.ifaces.lock().await;
            ifaces.send_on(in_idx, eth.to_bytes()).await;
        }
    }

    async fn send_icmp_error(&self, in_idx: usize, frame: &Ethernet, packet: &Ipv4, icmp_type: u8, code: u8) {
        let mut error_payload = Vec::new();
        error_payload.extend_from_slice(&[0u8; 4]);
        let orig = packet.to_bytes();
        let copy_len = (packet.header_length as usize * 4 + 8).min(orig.len());
        error_payload.extend_from_slice(&orig[..copy_len]);

        let icmp_pkt = Icmp::new(icmp_type, code, error_payload);
        let src_ip = self.interface_ip(in_idx).await.unwrap_or(Ipv4Addr::new(0, 0, 0, 0));
        let ipv4 = Ipv4::new(src_ip, packet.source, IpNextHeaderProtocol(1), icmp_pkt.to_bytes());
        let eth = Ethernet::new(frame.source, frame.destination, EtherTypes::Ipv4, ipv4.to_bytes());

        let ifaces = self.ifaces.lock().await;
        ifaces.send_on(in_idx, eth.to_bytes()).await;
    }

    async fn forward(&mut self, iface_id: usize, next_hop: Ipv4Addr, packet: Ipv4) {
        let found = self.arp_table.lookup(&next_hop);
        self.events.step(
            &self.name,
            EventKind::ArpLookup { ip: next_hop, found: found.is_some() },
            vec![self.arp_table_view()],
        ).await;

        if let Some(dst_mac) = found {
            let src_mac = self.interface_mac(iface_id).await.unwrap_or(MacAddr::zero());
            let frame = Ethernet::new(dst_mac, src_mac, EtherTypes::Ipv4, packet.to_bytes());
            let ifaces = self.ifaces.lock().await;
            ifaces.send_on(iface_id, frame.to_bytes()).await;
        } else {
            let src_ip = self.interface_ip(iface_id).await.unwrap_or(Ipv4Addr::new(0, 0, 0, 0));
            let src_mac = self.interface_mac(iface_id).await.unwrap_or(MacAddr::zero());

            self.pending_forward
                .entry(next_hop)
                .or_default()
                .push(PendingForward { iface_id, packet });

            self.events.step(
                &self.name,
                EventKind::ArpRequestSent { target_ip: next_hop },
                vec![self.arp_table_view()],
            ).await;

            let arp_req = Arp::new(
                ArpOperations::Request,
                src_mac, src_ip,
                MacAddr::zero(), next_hop,
            );
            let frame = Ethernet::new(
                MacAddr::broadcast(), src_mac,
                EtherTypes::Arp, arp_req.to_bytes(),
            );
            let ifaces = self.ifaces.lock().await;
            ifaces.send_on(iface_id, frame.to_bytes()).await;
        }
    }

    async fn flush_pending(&mut self, ip: Ipv4Addr) {
        let pending = match self.pending_forward.remove(&ip) {
            Some(p) => p,
            None => return,
        };
        let dst_mac = match self.arp_table.lookup(&ip) {
            Some(m) => m,
            None => return,
        };
        for p in pending {
            let src_mac = self.interface_mac(p.iface_id).await.unwrap_or(MacAddr::zero());
            let frame = Ethernet::new(dst_mac, src_mac, EtherTypes::Ipv4, p.packet.to_bytes());
            let ifaces = self.ifaces.lock().await;
            ifaces.send_on(p.iface_id, frame.to_bytes()).await;
        }
    }
}
