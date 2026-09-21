use crate::events::{EventKind, TableView};
use crate::net::ethernet::{Ethernet, EtherTypes};
use crate::net::ipv4::{IpNextHeaderProtocol, Ipv4};
use crate::net::arp::{Arp, ArpOperations};
use crate::devices::pc::{PC, PendingIpv4};
use crate::net::icmp::Icmp;
use crate::utils::addrs::ip_addr::Ipv4Addr;
use crate::utils::addrs::mac_addr::MacAddr;


impl PC {
    pub async fn handle_packet(&mut self, in_idx: usize, packet: Vec<u8>) {
        let frame = match Ethernet::from_bytes(&packet) {
            Ok(frame) => frame,
            Err(_) => return,
        };

        self.packets.lock().await.push(packet);

        match frame.ethertype {
            EtherTypes::Arp => self.handle_arp(in_idx, frame).await,
            EtherTypes::Ipv4 => self.handle_ipv4(in_idx, frame).await,
            _ => {}
        }
    }

    // ============================================================
    // Helpers
    // ============================================================

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

    pub(crate) async fn interface_mac_for_ip(&self, ip: Ipv4Addr) -> Option<MacAddr> {
        let ifaces = self.ifaces.lock().await;
        for iface in ifaces.iter() {
            if iface.addr == u32::from(ip) {
                return Some(MacAddr::from_bytes(&iface.mac).unwrap());
            }
        }
        None
    }

    pub(crate) async fn is_local_ip(&self, ip: Ipv4Addr) -> bool {
        let ifaces = self.ifaces.lock().await;
        let found = ifaces.iter().any(|iface| iface.addr == u32::from(ip));
        found
    }

    pub(crate) async fn send_raw(&self, iface_id: usize, data: Vec<u8>) {
        let ifaces = self.ifaces.lock().await;
        ifaces.send_on(iface_id, data).await;
    }

    pub(crate) async fn send_ethernet(&self, iface_id: usize, frame: Ethernet) {
        self.send_raw(iface_id, frame.to_bytes()).await;
    }

    pub(crate) fn arp_table_view(&self) -> TableView {
        let mut rows = Vec::new();
        for (ip, mac, age) in self.arp_table.entries() {
            rows.push(vec![
                ip.to_string(),
                mac.to_string(),
                format!("{}s", age.as_secs()),
            ]);
        }
        TableView {
            name: "ARP Table".into(),
            headers: vec!["IP".into(), "MAC".into(), "Age".into()],
            rows,
        }
    }

    // ============================================================
    // Send IPv4 (with ARP resolution)
    // ============================================================

    pub(crate) async fn send_ipv4(&mut self, iface_id: usize, packet: Ipv4) {
        let dst_ip = packet.destination;
        let found = self.arp_table.lookup(&dst_ip);

        self.events.step(
            &self.name,
            EventKind::ArpLookup { ip: dst_ip, found: found.is_some() },
            vec![self.arp_table_view()],
        ).await;

        if let Some(mac) = found {
            let src_mac = self.interface_mac(iface_id).await.unwrap_or(MacAddr::zero());
            let frame = Ethernet::new(mac, src_mac, EtherTypes::Ipv4, packet.to_bytes());
            self.send_ethernet(iface_id, frame).await;
        } else {
            let src_ip = self.interface_ip(iface_id).await.unwrap_or(Ipv4Addr::new(0, 0, 0, 0));
            let src_mac = self.interface_mac(iface_id).await.unwrap_or(MacAddr::zero());

            self.pending_ipv4
                .entry(dst_ip)
                .or_default()
                .push(PendingIpv4 { iface_id, packet });

            let arp_request = Arp::new(
                ArpOperations::Request,
                src_mac, src_ip,
                MacAddr::zero(), dst_ip,
            );

            self.events.step(
                &self.name,
                EventKind::ArpRequestSent { target_ip: dst_ip },
                vec![self.arp_table_view()],
            ).await;

            let frame = Ethernet::new(
                MacAddr::broadcast(), src_mac,
                EtherTypes::Arp, arp_request.to_bytes(),
            );
            self.send_ethernet(iface_id, frame).await;
        }
    }

    // ============================================================
    // ARP
    // ============================================================

    async fn handle_arp(&mut self, in_idx: usize, frame: Ethernet) {
        let arp = match Arp::from_bytes(&frame.payload) {
            Ok(arp) => arp,
            Err(_) => return,
        };

        self.arp_table.learn(arp.sender_ip, arp.sender_mac);

        match arp.operation {
            ArpOperations::Request => self.handle_arp_request(in_idx, arp).await,
            ArpOperations::Reply => self.handle_arp_reply(arp).await,
            _ => {}
        }
    }

    async fn handle_arp_request(&mut self, in_idx: usize, arp: Arp) {
        self.events.step(
            &self.name,
            EventKind::ArpRequestReceived { sender_ip: arp.sender_ip, sender_mac: arp.sender_mac },
            vec![self.arp_table_view()],
        ).await;

        let source_mac = match self.interface_mac_for_ip(arp.target_ip).await {
            Some(mac) => mac,
            None => return,
        };

        self.events.step(
            &self.name,
            EventKind::ArpReplySent { target_ip: arp.sender_ip, mac: source_mac },
            vec![self.arp_table_view()],
        ).await;

        let reply = Arp::new(
            ArpOperations::Reply,
            source_mac, arp.target_ip,
            arp.sender_mac, arp.sender_ip,
        );

        let frame = Ethernet::new(
            arp.sender_mac, source_mac,
            EtherTypes::Arp, reply.to_bytes(),
        );
        self.send_ethernet(in_idx, frame).await;
    }

    async fn handle_arp_reply(&mut self, arp: Arp) {
        self.arp_table.learn(arp.sender_ip, arp.sender_mac);

        self.events.step(
            &self.name,
            EventKind::ArpReplyReceived { sender_ip: arp.sender_ip, sender_mac: arp.sender_mac },
            vec![self.arp_table_view()],
        ).await;

        self.flush_pending(arp.sender_ip).await;
    }

    async fn flush_pending(&mut self, ip: Ipv4Addr) {
        let pending = match self.pending_ipv4.remove(&ip) {
            Some(p) => p,
            None => return,
        };

        let mac = match self.arp_table.lookup(&ip) {
            Some(m) => m,
            None => return,
        };

        for p in pending {
            let src_mac = self.interface_mac(p.iface_id).await.unwrap_or(MacAddr::zero());
            let frame = Ethernet::new(mac, src_mac, EtherTypes::Ipv4, p.packet.to_bytes());
            self.send_ethernet(p.iface_id, frame).await;
        }
    }

    // ============================================================
    // IPv4
    // ============================================================

    async fn handle_ipv4(&mut self, in_idx: usize, frame: Ethernet) {
        let packet = match Ipv4::from_bytes(&frame.payload) {
            Ok(packet) => packet,
            Err(_) => return,
        };

        if !packet.verify_checksum() {
            return;
        }

        if !self.is_local_ip(packet.destination).await {
            return;
        }

        match packet.next_level_protocol {
            IpNextHeaderProtocol(1) => {
                self.handle_icmp(in_idx, frame, packet).await;
            }
            _ => {}
        }
    }

    // ============================================================
    // ICMP
    // ============================================================

    async fn handle_icmp(&mut self, in_idx: usize, frame: Ethernet, packet: Ipv4) {
        let icmp = match Icmp::from_bytes(&packet.payload) {
            Ok(icmp) => icmp,
            Err(_) => return,
        };

        if !icmp.verify_checksum() {
            return;
        }

        if icmp.is_echo_request() {
            self.handle_icmp_echo_request(in_idx, &frame, &packet, &icmp).await;
        }

        if icmp.is_echo_reply() {
            self.handle_icmp_echo_reply(&packet, &icmp).await;
        }
    }

    async fn handle_icmp_echo_request(
        &mut self,
        in_idx: usize,
        frame: &Ethernet,
        packet: &Ipv4,
        request: &Icmp,
    ) {
        self.events.step(
            &self.name,
            EventKind::IcmpEchoRequest { src: packet.source, dst: packet.destination },
            vec![self.arp_table_view()],
        ).await;

        let reply = Icmp::new(
            crate::net::icmp::ECHO_REPLY, 0,
            request.payload.clone(),
        );

        let ipv4 = Ipv4::new(
            packet.destination, packet.source,
            IpNextHeaderProtocol(1), reply.to_bytes(),
        );

        let ethernet = Ethernet::new(
            frame.source, frame.destination,
            EtherTypes::Ipv4, ipv4.to_bytes(),
        );

        self.events.step(
            &self.name,
            EventKind::IcmpEchoReply { src: packet.destination, dst: packet.source },
            vec![self.arp_table_view()],
        ).await;

        self.send_ethernet(in_idx, ethernet).await;
    }

    async fn handle_icmp_echo_reply(&mut self, packet: &Ipv4, _icmp: &Icmp) {
        self.events.step(
            &self.name,
            EventKind::IcmpEchoReply { src: packet.source, dst: packet.destination },
            vec![self.arp_table_view()],
        ).await;
    }
}
