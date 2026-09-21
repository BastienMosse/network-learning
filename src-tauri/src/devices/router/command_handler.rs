use crate::devices::router::Router;
use crate::utils::addrs::ip_addr::Ipv4Addr;
use crate::utils::addrs::mac_addr::MacAddr;
use tokio::sync::oneshot;


pub enum RouterCommand {
    AddRoute {
        network: Ipv4Addr,
        prefix_len: u8,
        next_hop: Ipv4Addr,
        iface_name: String,
        reply: oneshot::Sender<bool>,
    },
    Exec {
        input: String,
        reply: oneshot::Sender<String>,
    },
}


impl Router {
    pub(crate) async fn handle_command(&mut self, cmd: RouterCommand) {
        match cmd {
            RouterCommand::AddRoute { network, prefix_len, next_hop, iface_name, reply } => {
                let iface_id = {
                    let ifaces = self.ifaces.lock().await;
                    ifaces.id_by_name(&iface_name)
                };
                match iface_id {
                    Some(id) => {
                        self.routing_table.add(network, prefix_len, next_hop, id, &iface_name);
                        let _ = reply.send(true);
                    }
                    None => { let _ = reply.send(false); }
                }
            }
            RouterCommand::Exec { input, reply } => {
                let output = self.handle_exec(&input).await;
                let _ = reply.send(output);
            }
        }
    }

    async fn handle_exec(&self, input: &str) -> String {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return String::new();
        }

        match parts[0] {
            "route" => self.cmd_route(),
            "arp" => self.cmd_arp(),
            "ifconfig" | "ip" => self.cmd_ifconfig().await,
            "help" => self.cmd_help(),
            _ => format!("unknown command: {}", parts[0]),
        }
    }

    fn cmd_route(&self) -> String {
        let mut out = String::from("Routing Table:\n");
        let mut count = 0;
        for route in self.routing_table.entries() {
            let prefix_len = route.mask.count_ones();
            out.push_str(&format!(
                "  {}/{} via {} dev {}\n",
                Ipv4Addr::from_u32(route.network), prefix_len, route.next_hop, route.iface_name,
            ));
            count += 1;
        }
        if count == 0 {
            out.push_str("  (empty)\n");
        }
        out
    }

    fn cmd_arp(&self) -> String {
        let mut out = String::from("ARP Table:\n");
        let mut count = 0;
        for (ip, mac, age) in self.arp_table.entries() {
            out.push_str(&format!("  {} -> {} ({}s ago)\n", ip, mac, age.as_secs()));
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
            let mac = MacAddr::from_bytes(&iface.mac).unwrap_or(MacAddr::zero());
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
         route        - Show routing table\n  \
         arp          - Show ARP table\n  \
         ifconfig     - Show interfaces\n  \
         help         - Show this help\n"
            .into()
    }
}
