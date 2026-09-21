use crate::devices::hub::Hub;
use crate::utils::addrs::ip_addr::Ipv4Addr;
use crate::utils::addrs::mac_addr::MacAddr;
use tokio::sync::oneshot;


pub enum HubCommand {
    Exec {
        input: String,
        reply: oneshot::Sender<String>,
    },
}


impl Hub {
    pub(crate) async fn handle_command(&mut self, cmd: HubCommand) {
        match cmd {
            HubCommand::Exec { input, reply } => {
                let output = self.handle_exec(&input).await;
                let _ = reply.send(output);
            }
        }
    }

    async fn handle_exec(&self, input: &str) -> String {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { return String::new(); }
        match parts[0] {
            "ifconfig" | "ip" => self.cmd_ifconfig().await,
            "help" => self.cmd_help(),
            _ => format!("unknown command: {}", parts[0]),
        }
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
         ifconfig     - Show interfaces\n  \
         help         - Show this help\n"
            .into()
    }
}
