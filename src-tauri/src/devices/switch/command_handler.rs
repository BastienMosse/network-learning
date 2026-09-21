use crate::devices::switch::Switch;
use crate::utils::addrs::ip_addr::Ipv4Addr;
use crate::utils::addrs::mac_addr::MacAddr;
use tokio::sync::oneshot;


pub enum SwitchCommand {
    ShowMacTable {
        reply: oneshot::Sender<Vec<([u8; 6], usize)>>,
    },
    Exec {
        input: String,
        reply: oneshot::Sender<String>,
    },
}


impl Switch {
    pub(crate) async fn handle_command(&mut self, cmd: SwitchCommand) {
        match cmd {
            SwitchCommand::ShowMacTable { reply } => {
                let entries = self.mac_table.entries()
                    .map(|(mac, iface_id)| (mac.to_bytes(), iface_id))
                    .collect();
                let _ = reply.send(entries);
            }
            SwitchCommand::Exec { input, reply } => {
                let output = self.handle_exec(&input).await;
                let _ = reply.send(output);
            }
        }
    }

    async fn handle_exec(&self, input: &str) -> String {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { return String::new(); }
        match parts[0] {
            "mac" => self.cmd_mac(),
            "ifconfig" | "ip" => self.cmd_ifconfig().await,
            "help" => self.cmd_help(),
            _ => format!("unknown command: {}", parts[0]),
        }
    }

    fn cmd_mac(&self) -> String {
        let mut out = String::from("MAC Table:\n");
        let mut count = 0;
        for (mac, port) in self.mac_table.entries() {
            out.push_str(&format!("  {} -> port {}\n", mac, port));
            count += 1;
        }
        if count == 0 { out.push_str("  (empty)\n"); }
        out
    }

    async fn cmd_ifconfig(&self) -> String {
        let ifaces = self.ifaces.lock().await;
        let mut out = String::new();
        for (id, iface) in ifaces.iter_with_id() {
            let mac = MacAddr::from_bytes(&iface.mac)
                .unwrap_or(MacAddr::zero());
            let ip = Ipv4Addr::from_u32(iface.addr);
            out.push_str(&format!(
                "{} (port {}):\n  MAC:  {}\n  IP:   {}/{}\n  MTU:  {}\n\n",
                iface.name, id, mac, ip, iface.mask, iface.mtu,
            ));
        }
        if out.is_empty() { out.push_str("no interfaces configured\n"); }
        out
    }

    fn cmd_help(&self) -> String {
        "Available commands:\n  \
         mac          - Show MAC address table\n  \
         ifconfig     - Show interfaces\n  \
         help         - Show this help\n"
            .into()
    }
}
