use crate::utils::interfaces::Interfaces;
use crate::utils::socket::Socket;

use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;

#[allow(async_fn_in_trait)]
pub trait Device: Send + Sync + 'static {
    type Command: Send + 'static;

    fn name(&self) -> &str;

    fn ifaces(&self) -> &Arc<Mutex<Interfaces>>;

    async fn on_packet(
        &mut self,
        in_idx: usize,
        packet: Vec<u8>,
    );

    async fn on_command(
        &mut self,
        cmd: Self::Command,
    );

    // ============================================================
    // API commune pour manipuler les interfaces
    // ============================================================

    async fn add_interface(
        &self,
        name: &str,
        addr: &str,
        mask: u8,
        mtu: u16,
    ) -> (usize, [u8; 6]);

    async fn get_iface_id(
        &self,
        name: &str,
    ) -> Option<usize>;

    async fn set_socket(
        &self,
        id: usize,
        soc: Socket,
    );

    // ============================================================
    // Boucle principale
    // ============================================================

    async fn run(
        &mut self,
        commands: &mut mpsc::Receiver<Self::Command>,
    ) {
        {
            let ifaces = self.ifaces().lock().await;
            ifaces.listen(self.name());
        }

        loop {
            tokio::select! {
                received = async {
                    let mut ifaces = self.ifaces().lock().await;
                    ifaces.recv().await
                } => {
                    if let Some((idx, packet)) = received {
                        self.on_packet(idx, packet).await;
                    }
                }

                command = commands.recv() => {
                    match command {
                        Some(command) => {
                            self.on_command(command).await;
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }
    }
}