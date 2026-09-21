use crate::events::EventBus;
use crate::utils::addrs::ip_addr::Ipv4Addr;
use crate::utils::interfaces::Interfaces;
use crate::utils::tables::mac_table::MacTable;
use crate::devices::device::Device;
use crate::utils::socket::Socket;

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use super::command_handler::BridgeCommand;


pub struct Bridge {
    pub(crate) name: String,
    pub(crate) ifaces: Arc<Mutex<Interfaces>>,
    pub(crate) mac_table: MacTable,
    pub(crate) events: EventBus,
    commands: Option<mpsc::Receiver<BridgeCommand>>,
    command_tx: mpsc::Sender<BridgeCommand>,
}


impl Bridge {
    pub fn new(name: impl Into<String>) -> Self {
        let (command_tx, commands) = mpsc::channel(8);
        Self {
            name: name.into(),
            ifaces: Arc::new(Mutex::new(Interfaces::new())),
            mac_table: MacTable::new(),
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

    pub async fn exec(&self, input: &str) -> String {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx.send(BridgeCommand::Exec { input: input.to_string(), reply: tx }).await;
        rx.await.unwrap_or_else(|_| "device not running".into())
    }

    pub fn start(&mut self) -> tokio::task::JoinHandle<()> {
        let mut commands = self.commands.take().expect("Bridge already started");
        let ifaces = Arc::clone(&self.ifaces);
        let name = self.name.clone();
        let mac_table = std::mem::take(&mut self.mac_table);
        let events = self.events.clone();

        tokio::spawn(async move {
            let mut bridge = Bridge {
                name,
                ifaces,
                mac_table,
                events,
                commands: None,
                command_tx: { let (tx, _) = mpsc::channel(1); tx },
            };
            bridge.run(&mut commands).await;
        })
    }
}


impl Device for Bridge {
    type Command = BridgeCommand;

    fn name(&self) -> &str { &self.name }
    fn ifaces(&self) -> &Arc<Mutex<Interfaces>> { &self.ifaces }

    async fn on_packet(&mut self, in_idx: usize, packet: Vec<u8>) {
        self.handle_packet(in_idx, packet).await;
    }

    async fn on_command(&mut self, command: BridgeCommand) {
        self.handle_command(command).await;
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
