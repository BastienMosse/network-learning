use crate::utils::addrs::ip_addr::Ipv4Addr;
use crate::utils::interfaces::Interfaces;
use crate::net::ethernet::Ethernet;
use crate::devices::device::Device;
use crate::utils::socket::Socket;

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{
    mpsc,
    oneshot,
    Mutex,
};


pub enum BridgeCommand {
    ShowMacTable {
        reply: oneshot::Sender<Vec<([u8; 6], usize)>>,
    },
}


pub struct Bridge {
    name: String,
    ifaces: Arc<Mutex<Interfaces>>,
    mac_table: Arc<Mutex<HashMap<[u8; 6], usize>>>,
    commands: Option<mpsc::Receiver<BridgeCommand>>,
    command_tx: mpsc::Sender<BridgeCommand>,
}


impl Bridge {
    pub fn new(
        name: impl Into<String>,
    ) -> Self {
        let (command_tx, commands) = mpsc::channel(8);
        Self {
            name: name.into(),
            ifaces: Arc::new(Mutex::new(Interfaces::new())),
            mac_table: Arc::new(Mutex::new(HashMap::new())),
            commands: Some(commands),
            command_tx,
        }
    }

    pub fn start(&mut self) -> tokio::task::JoinHandle<()> {
        let mut commands = self
            .commands
            .take()
            .expect("Bridge déjà démarré");

        let ifaces = Arc::clone(&self.ifaces);
        let mac_table = Arc::clone(&self.mac_table);
        let name = self.name.clone();

        tokio::spawn(async move {
            let mut bridge = Bridge {
                name,
                ifaces,
                mac_table,
                commands: None,
                command_tx: {
                    let (tx, _rx) = mpsc::channel(1);
                    tx
                },
            };

            bridge.run(&mut commands).await;
        })
    }


    // ============================================================
    // Commands
    // ============================================================

    pub async fn show_mac_table(
        &self,
    ) -> Vec<([u8; 6], usize)> {
        let (reply_tx, reply_rx) =
            oneshot::channel();

        self.command_tx
            .send(
                BridgeCommand::ShowMacTable {
                    reply: reply_tx,
                }
            )
            .await
            .expect("Bridge arrêté");

        reply_rx
            .await
            .unwrap_or_default()
    }
}


impl Device for Bridge {
    type Command = BridgeCommand;


    fn name(&self) -> &str {
        &self.name
    }

    fn ifaces(&self) -> &Arc<Mutex<Interfaces>> {
        &self.ifaces
    }


    async fn on_packet(
        &mut self,
        in_idx: usize,
        packet: Ethernet,
    ) {
        let dest_mac = packet.destination.to_bytes();
        let src_mac = packet.source.to_bytes();

        // ========================================================
        // Apprentissage
        // ========================================================

        {
            let mut table =
                self.mac_table
                    .lock()
                    .await;

            table.insert(
                src_mac,
                in_idx,
            );
        }


        // ========================================================
        // Recherche de la destination
        // ========================================================

        let out_idx = {
            let table =
                self.mac_table
                    .lock()
                    .await;

            table
                .get(&dest_mac)
                .copied()
        };


        let ifaces =
            self.ifaces
                .lock()
                .await;


        match out_idx {
            Some(out_idx) if out_idx != in_idx =>
            {
                ifaces.send_on(
                        out_idx,
                        packet.to_bytes(),
                    )
                    .await;
            }
            Some(_) => {}
            None => {
                let ids = ifaces.ids().collect::<Vec<_>>();
                for id in ids {
                    if id != in_idx {
                        ifaces
                            .send_on(
                                id,
                                packet.clone().to_bytes(),
                            )
                            .await;
                    }
                }
            }
        }
    }


    async fn on_command(
        &mut self,
        command: BridgeCommand,
    ) {
        match command {

            BridgeCommand::ShowMacTable {
                reply,
            } => {
                let table =
                    self.mac_table
                        .lock()
                        .await;

                let table =
                    table
                        .iter()
                        .map(|(mac, idx)| {
                            (*mac, *idx)
                        })
                        .collect();

                let _ =
                    reply.send(table);
            }
        }
    }


    async fn add_interface(
        &self,
        name: &str,
        addr: &str,
        mask: u8,
        mtu: u16,
    ) -> usize {
        let ip: Ipv4Addr = addr
            .parse()
            .expect("adresse IPv4 invalide");

        let mut ifaces =
            self.ifaces
                .lock()
                .await;

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
            self.ifaces
                .lock()
                .await;

        ifaces.id_by_name(name)
    }


    async fn set_socket(
        &self,
        id: usize,
        socket: Socket,
    ) {
        let mut ifaces =
            self.ifaces
                .lock()
                .await;

        ifaces.set_socket(
            id,
            socket,
        );
    }
}