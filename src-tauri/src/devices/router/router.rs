use crate::events::EventBus;
use crate::utils::addrs::ip_addr::Ipv4Addr;
use crate::utils::interfaces::Interfaces;
use crate::utils::tables::arp_table::ArpTable;
use crate::utils::tables::routing_table::RoutingTable;
use crate::devices::device::Device;
use crate::utils::socket::Socket;
use crate::net::ipv4::Ipv4;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};

use super::command_handler::RouterCommand;


pub struct PendingForward {
    pub iface_id: usize,
    pub packet: Ipv4,
}


pub struct Router {
    pub(crate) name: String,
    pub(crate) ifaces: Arc<Mutex<Interfaces>>,
    pub(crate) arp_table: ArpTable,
    pub(crate) routing_table: RoutingTable,
    pub(crate) pending_forward: HashMap<Ipv4Addr, Vec<PendingForward>>,
    pub(crate) events: EventBus,
    commands: Option<mpsc::Receiver<RouterCommand>>,
    command_tx: mpsc::Sender<RouterCommand>,
}


impl Router {
    pub fn new(name: impl Into<String>) -> Self {
        let (command_tx, commands) = mpsc::channel(8);
        Self {
            name: name.into(),
            ifaces: Arc::new(Mutex::new(Interfaces::new())),
            arp_table: ArpTable::new(),
            routing_table: RoutingTable::new(),
            pending_forward: HashMap::new(),
            events: EventBus::default(),
            commands: Some(commands),
            command_tx,
        }
    }

    pub fn with_event_bus(mut self, bus: EventBus) -> Self {
        self.events = bus;
        self
    }

    pub fn prepare_restart(&mut self) {
        let (tx, rx) = mpsc::channel(8);
        self.command_tx = tx;
        self.commands = Some(rx);
    }

    pub fn start(&mut self) -> tokio::task::JoinHandle<()> {
        let mut commands = self.commands.take().expect("Router already started");
        let ifaces = Arc::clone(&self.ifaces);
        let name = self.name.clone();
        let arp_table = std::mem::take(&mut self.arp_table);
        let routing_table = std::mem::take(&mut self.routing_table);
        let pending_forward = std::mem::take(&mut self.pending_forward);
        let events = self.events.clone();

        tokio::spawn(async move {
            let mut router = Router {
                name,
                ifaces,
                arp_table,
                routing_table,
                pending_forward,
                events,
                commands: None,
                command_tx: { let (tx, _) = mpsc::channel(1); tx },
            };
            router.run(&mut commands).await;
        })
    }

    pub async fn add_route(&self, network: Ipv4Addr, prefix_len: u8, next_hop: Ipv4Addr, iface_name: &str) -> bool {
        let (tx, rx) = oneshot::channel();
        let _ = self.command_tx.send(RouterCommand::AddRoute {
            network, prefix_len, next_hop,
            iface_name: iface_name.to_string(),
            reply: tx,
        }).await;
        rx.await.unwrap_or(false)
    }

    pub async fn exec(&self, input: &str) -> String {
        let (tx, rx) = oneshot::channel();
        let _ = self.command_tx.send(RouterCommand::Exec {
            input: input.to_string(), reply: tx,
        }).await;
        rx.await.unwrap_or_else(|_| "device not running".into())
    }
}


impl Device for Router {
    type Command = RouterCommand;

    fn name(&self) -> &str { &self.name }
    fn ifaces(&self) -> &Arc<Mutex<Interfaces>> { &self.ifaces }

    async fn on_packet(&mut self, in_idx: usize, packet: Vec<u8>) {
        self.handle_packet(in_idx, packet).await;
    }

    async fn on_command(&mut self, cmd: RouterCommand) {
        self.handle_command(cmd).await;
    }

    async fn add_interface(&self, name: &str, addr: &str, mask: u8, mtu: u16) -> (usize, [u8; 6]) {
        let ip: Ipv4Addr = addr.parse().expect("invalid IPv4 address");
        let mut ifaces = self.ifaces.lock().await;
        ifaces.add(name, u32::from(ip), mask, mtu)
    }

    async fn get_iface_id(&self, name: &str) -> Option<usize> {
        self.ifaces.lock().await.id_by_name(name)
    }

    async fn set_socket(&self, id: usize, socket: Socket) {
        self.ifaces.lock().await.set_socket(id, socket);
    }
}
