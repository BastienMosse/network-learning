use crate::devices::device::Device;
use crate::events::EventBus;
use crate::net::ipv4::Ipv4;
use crate::utils::addrs::ip_addr::Ipv4Addr;
use crate::utils::interfaces::Interfaces;
use crate::utils::socket::Socket;
use crate::utils::tables::arp_table::ArpTable;

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use super::command_handler::PcCommand;


pub struct PendingIpv4 {
    pub iface_id: usize,
    pub packet: Ipv4,
}


pub struct PC {
    pub(crate) name: String,
    pub(crate) ifaces: Arc<Mutex<Interfaces>>,
    pub(crate) packets: Arc<Mutex<Vec<Vec<u8>>>>,
    pub(crate) arp_table: ArpTable,
    pub(crate) pending_ipv4: HashMap<Ipv4Addr, Vec<PendingIpv4>>,
    pub(crate) events: EventBus,
    commands: Option<mpsc::Receiver<PcCommand>>,
    command_tx: mpsc::Sender<PcCommand>,
}


impl PC {
    pub fn new(name: impl Into<String>) -> Self {
        let (command_tx, commands) = mpsc::channel(8);

        Self {
            name: name.into(),
            ifaces: Arc::new(Mutex::new(Interfaces::new())),
            packets: Arc::new(Mutex::new(Vec::new())),
            arp_table: ArpTable::new(),
            pending_ipv4: HashMap::new(),
            events: EventBus::default(),
            commands: Some(commands),
            command_tx,
        }
    }

    pub fn with_event_bus(mut self, bus: EventBus) -> Self {
        self.events = bus;
        self
    }

    pub fn start(&mut self) -> tokio::task::JoinHandle<()> {
        let mut commands = self.commands.take().expect("PC already started");
        let ifaces = Arc::clone(&self.ifaces);
        let packets = Arc::clone(&self.packets);
        let name = self.name.clone();
        let arp_table = std::mem::take(&mut self.arp_table);
        let pending_ipv4 = std::mem::take(&mut self.pending_ipv4);
        let events = self.events.clone();

        tokio::spawn(async move {
            let mut pc = PC {
                name,
                ifaces,
                packets,
                arp_table,
                pending_ipv4,
                events,
                commands: None,
                command_tx: { let (tx, _) = mpsc::channel(1); tx },
            };
            pc.run(&mut commands).await;
        })
    }

    pub fn prepare_restart(&mut self) {
        let (tx, rx) = mpsc::channel(8);
        self.command_tx = tx;
        self.commands = Some(rx);
    }

    pub fn command_sender(&self) -> mpsc::Sender<PcCommand> {
        self.command_tx.clone()
    }

    pub async fn send_frame(&self, iface_id: usize, frame: Vec<u8>) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx
            .send(PcCommand::SendFrame { iface_id, packet: frame, reply: tx })
            .await;
        let _ = rx.await;
    }

    pub async fn list_packets(&self) -> Vec<Vec<u8>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx
            .send(PcCommand::ListPackets { reply: tx })
            .await;
        rx.await.unwrap_or_default()
    }

    pub async fn ping(&self, target: Ipv4Addr) -> bool {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx
            .send(PcCommand::Ping { target, reply: tx })
            .await;
        rx.await.unwrap_or(false)
    }

    pub async fn exec(&self, input: &str) -> String {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _ = self.command_tx
            .send(PcCommand::Exec { input: input.to_string(), reply: tx })
            .await;
        rx.await.unwrap_or_else(|_| "device not running".into())
    }
}


impl Device for PC {
    type Command = PcCommand;


    fn name(&self) -> &str {
        &self.name
    }


    fn ifaces(
        &self,
    ) -> &Arc<Mutex<Interfaces>> {
        &self.ifaces
    }


    async fn add_interface(
        &self,
        name: &str,
        addr: &str,
        mask: u8,
        mtu: u16,
    ) -> (usize, [u8; 6]) {
        let ip: Ipv4Addr =
            addr
                .parse()
                .expect("adresse IPv4 invalide");

        let mut ifaces =
            self.ifaces.lock().await;

        ifaces.add(
            name,
            u32::from(ip),
            mask,
            mtu,
        )
    }


    async fn get_iface_id(
        &self,
        name: &str,
    ) -> Option<usize> {
        let ifaces =
            self.ifaces.lock().await;

        ifaces.id_by_name(name)
    }


    async fn set_socket(
        &self,
        id: usize,
        socket: Socket,
    ) {
        let mut ifaces =
            self.ifaces.lock().await;

        ifaces.set_socket(
            id,
            socket,
        );
    }


    async fn on_packet(&mut self, in_idx: usize, packet: Vec<u8>) {
        self.handle_packet(in_idx, packet).await;
    }


    async fn on_command(&mut self, command: PcCommand) {
        self.handle_command(command).await;
    }
}