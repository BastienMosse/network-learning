use crate::utils::addrs::ip_addr::Ipv4Addr;
use crate::utils::interfaces::Interfaces;
use crate::net::ethernet::Ethernet;
use crate::devices::device::Device;
use crate::utils::socket::Socket;

use std::sync::Arc;
use tokio::sync::{
    mpsc,
    oneshot,
    Mutex,
};


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
}

pub struct PC {
    name: String,
    ifaces: Arc<Mutex<Interfaces>>,
    packets: Arc<Mutex<Vec<Vec<u8>>>>,
    commands: Option<mpsc::Receiver<PcCommand>>,
    command_tx: mpsc::Sender<PcCommand>,
}


impl PC {
    pub fn new(
        name: impl Into<String>,
    ) -> Self {
        let (command_tx, commands) =
            mpsc::channel(8);

        Self {
            name: name.into(),
            ifaces: Arc::new(Mutex::new(Interfaces::new())),
            packets: Arc::new(Mutex::new(Vec::new())),
            commands: Some(commands),
            command_tx,
        }
    }

    pub fn start(&mut self) -> tokio::task::JoinHandle<()> {
        let mut commands = self
            .commands
            .take()
            .expect("PC déjà démarré");

        let ifaces = Arc::clone(&self.ifaces);
        let packets = Arc::clone(&self.packets);
        let name = self.name.clone();

        tokio::spawn(async move {
            let mut pc = PC {
                name,
                ifaces,
                packets,
                commands: None,
                command_tx: {
                    let (tx, _rx) = mpsc::channel(1);
                    tx
                },
            };

            pc.run(&mut commands).await;
        })
    }


    // ============================================================
    // Commands
    // ============================================================

    pub async fn send_frame(
        &self,
        iface_id: usize,
        packet: Vec<u8>,
    ) {
        let (reply_tx, reply_rx) =
            oneshot::channel();

        self.command_tx
            .send(
                PcCommand::SendFrame {
                    iface_id,
                    packet,
                    reply: reply_tx,
                }
            )
            .await
            .expect("PC arrêté");

        let _ = reply_rx.await;
    }

    pub async fn list_packets(
        &self,
    ) -> Vec<Vec<u8>> {
        let (reply_tx, reply_rx) =
            oneshot::channel();

        self.command_tx.send(PcCommand::ListPackets { reply: reply_tx })
            .await
            .expect("PC arrêté");

        reply_rx
            .await
            .unwrap_or_default()
    }

}


impl Device for PC {
    type Command = PcCommand;


    fn name(&self) -> &str {
        &self.name
    }

    fn ifaces(&self) -> &Arc<Mutex<Interfaces>> {
        &self.ifaces
    }

    async fn on_packet(
        &mut self,
        in_idx: usize,
        frame: Ethernet,
    ) {
        println!("[{}] paquet reçu sur iface {} : {:?}", self.name, in_idx, frame);
        let packet = frame.to_bytes();

        self.packets
            .lock()
            .await
            .push(packet);
    }

    async fn on_command(&mut self, command: PcCommand) {
        match command {
            PcCommand::Ping { target, reply } => {
                println!("[{}] ping vers {:?} (simulé)", self.name, target);
                let _ = reply.send(true);
            }

            PcCommand::ListPackets { reply } => {
                let packets = self.packets.lock().await;
                let _ = reply.send(packets.clone());
            }


            PcCommand::SendFrame { iface_id, packet, reply } => {
                let frame = match Ethernet::from_bytes(&packet) {
                    Ok(frame) => frame,

                    Err(_) => {
                        let _ = reply.send(());
                        return;
                    }
                };

                if !frame.verify_fcs() {
                    let _ = reply.send(());
                    return;
                }

                let ifaces = self.ifaces.lock().await;
                ifaces.send_on(iface_id, packet).await;
                let _ = reply.send(());
            }
        }
    }

    async fn add_interface(&self, name: &str, addr: &str, mask: u8, mtu: u16) -> usize {
        let ip: Ipv4Addr = addr.parse().expect("adresse IPv4 invalide");
        let mut ifaces = self.ifaces.lock().await;
        ifaces.add(name, u32::from(ip), mask, mtu)
    }

    async fn get_iface_id(&self, name: &str) -> Option<usize> {
        let ifaces = self.ifaces.lock().await;
        ifaces.id_by_name(name)
    }

    async fn set_socket( &self, id: usize, socket: Socket) {
        let mut ifaces = self.ifaces.lock().await;
        ifaces.set_socket(id, socket);
    }
}