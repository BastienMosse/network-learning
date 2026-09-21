use crate::devices::pc::PC;
use crate::net::ethernet::Ethernet;
use crate::net::icmp::{Icmp, ECHO_REQUEST};
use crate::net::ipv4::{IpNextHeaderProtocol, Ipv4};
use crate::utils::addrs::ip_addr::Ipv4Addr;

use tokio::sync::oneshot;


pub enum PcCommand {
    Ping {
        target: Ipv4Addr,
        reply: oneshot::Sender<bool>,
    },
    ListPackets {
        reply: oneshot::Sender<Vec<Vec<u8>>>,
    },
    SendFrame {
        iface_id: usize,
        packet: Vec<u8>,
        reply: oneshot::Sender<()>,
    },
    Exec {
        input: String,
        reply: oneshot::Sender<String>,
    },
}


impl PC {
    pub async fn handle_command(&mut self, command: PcCommand) {
        match command {
            PcCommand::Ping { target, reply } => {
                let success = self.handle_ping(target).await;
                let _ = reply.send(success);
            }
            PcCommand::ListPackets { reply } => {
                let packets = self.packets.lock().await;
                let _ = reply.send(packets.clone());
            }
            PcCommand::SendFrame { iface_id, packet, reply } => {
                self.handle_send_frame(iface_id, packet).await;
                let _ = reply.send(());
            }
            PcCommand::Exec { input, reply } => {
                let output = self.handle_exec(&input).await;
                let _ = reply.send(output);
            }
        }
    }

    async fn handle_send_frame(&self, iface_id: usize, packet: Vec<u8>) {
        let frame = match Ethernet::from_bytes(&packet) {
            Ok(frame) => frame,
            Err(_) => return,
        };

        if !frame.verify_fcs() {
            return;
        }

        let ifaces = self.ifaces.lock().await;
        ifaces.send_on(iface_id, packet).await;
    }

    async fn handle_ping(&mut self, target: Ipv4Addr) -> bool {
        let iface_id = {
            let ifaces = self.ifaces.lock().await;
            let id = ifaces.ids().next();
            match id {
                Some(id) => id,
                None => return false,
            }
        };

        let icmp = Icmp::new(ECHO_REQUEST, 0, Vec::new());

        let ipv4 = {
            let source = match self.interface_ip(iface_id).await {
                Some(ip) => ip,
                None => return false,
            };
            Ipv4::new(source, target, IpNextHeaderProtocol(1), icmp.to_bytes())
        };

        self.send_ipv4(iface_id, ipv4).await;
        true
    }

    // ============================================================
    // CLI text commands
    // ============================================================

    async fn handle_exec(&mut self, input: &str) -> String {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return String::new();
        }

        match parts[0] {
            "ping" => self.cmd_ping(&parts).await,
            "arp" => self.cmd_arp(&parts).await,
            "ifconfig" | "ip" => self.cmd_ifconfig().await,
            "help" => self.cmd_help(),
            _ => format!("unknown command: {}", parts[0]),
        }
    }

    async fn cmd_ping(&mut self, args: &[&str]) -> String {
        if args.len() < 2 {
            return "usage: ping <ip>".into();
        }

        let target: Ipv4Addr = match args[1].parse() {
            Ok(ip) => ip,
            Err(_) => return format!("invalid IP: {}", args[1]),
        };

        let sent = self.handle_ping(target).await;
        if sent {
            format!("PING {} - request sent", target)
        } else {
            format!("PING {} - failed (no interface)", target)
        }
    }

    async fn cmd_arp(&self, args: &[&str]) -> String {
        if args.len() >= 2 && args[1] == "-d" {
            return "arp -d not supported yet".into();
        }

        let mut out = String::from("ARP Table:\n");
        let mut count = 0;
        for (ip, mac, age) in self.arp_table.entries() {
            out.push_str(&format!(
                "  {} -> {} ({}s ago)\n", ip, mac, age.as_secs()
            ));
            count += 1;
        }
        if count == 0 {
            out.push_str("  (empty)\n");
        }
        out
    }

    async fn cmd_ifconfig(&self) -> String {
        let ifaces = self.ifaces.lock().await;
        let mut out = String::new();

        for (id, iface) in ifaces.iter_with_id() {
            let ip = Ipv4Addr::from_u32(iface.addr);
            let mac = crate::utils::addrs::mac_addr::MacAddr::from_bytes(&iface.mac)
                .unwrap_or(crate::utils::addrs::mac_addr::MacAddr::zero());
            out.push_str(&format!(
                "{} (id={}):\n  IP:   {}/{}\n  MAC:  {}\n  MTU:  {}\n\n",
                iface.name, id, ip, iface.mask, mac, iface.mtu,
            ));
        }

        if out.is_empty() {
            out.push_str("no interfaces configured\n");
        }
        out
    }

    fn cmd_help(&self) -> String {
        "Available commands:\n  \
         ping <ip>    - Send ICMP echo request\n  \
         arp          - Show ARP table\n  \
         ifconfig     - Show interfaces\n  \
         help         - Show this help\n"
            .into()
    }
}
