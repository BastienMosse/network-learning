
use tokio::sync::mpsc;

pub struct Socket {
    tx: mpsc::Sender<Vec<u8>>,
    rx: mpsc::Receiver<Vec<u8>>,
}

impl Socket {
    pub fn pair(buffer: usize) -> (Socket, Socket) {
        let (tx_a, rx_b) = mpsc::channel(buffer);
        let (tx_b, rx_a) = mpsc::channel(buffer);

        (
            Socket {
                tx: tx_a,
                rx: rx_a,
            },
            Socket {
                tx: tx_b,
                rx: rx_b,
            },
        )
    }

    pub async fn send(&self, packet: Vec<u8>) {
        let _ = self.tx.send(packet).await;
    }

    pub async fn recv(&mut self) -> Option<Vec<u8>> {
        self.rx.recv().await
    }
}