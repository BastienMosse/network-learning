use crate::utils::socket::Socket;

use futures::future::{select_all, FutureExt};
use rand::Rng;
use std::collections::HashMap;


pub struct Iface {
    pub name: String,
    pub description: String,
    pub addr: u32,
    pub mask: u8,
    pub mtu: u16,
    pub mac: [u8; 6],
    pub soc: Option<Socket>,
}


pub struct Interfaces {
    ifaces: HashMap<usize, Iface>,
    next_id: usize,
}


impl Interfaces {
    pub fn new() -> Self {
        Self {
            ifaces: HashMap::new(),
            next_id: 0,
        }
    }


    pub fn add(
        &mut self,
        name: impl Into<String>,
        addr: u32,
        mask: u8,
        mtu: u16,
    ) -> (usize, [u8; 6]) {
        let id = self.next_id;
        self.next_id += 1;

        let mac = self.generate_unique_mac();

        self.ifaces.insert(
            id,
            Iface {
                name: name.into(),
                description: String::new(),
                addr,
                mask,
                mtu,
                mac,
                soc: None,
            },
        );

        (id, mac)
    }


    pub fn update(
        &mut self,
        id: usize,
        name: Option<&str>,
        addr: Option<u32>,
        mask: Option<u8>,
        mtu: Option<u16>,
    ) -> bool {
        if let Some(iface) = self.ifaces.get_mut(&id) {
            if let Some(n) = name { iface.name = n.to_string(); }
            if let Some(a) = addr { iface.addr = a; }
            if let Some(m) = mask { iface.mask = m; }
            if let Some(mt) = mtu { iface.mtu = mt; }
            true
        } else {
            false
        }
    }

    pub fn remove(
        &mut self,
        id: usize,
    ) -> Option<Iface> {
        self.ifaces.remove(&id)
    }


    pub fn ids(
        &self,
    ) -> impl Iterator<Item = usize> + '_ {
        self.ifaces.keys().copied()
    }


    pub fn get(&self, id: usize) -> Option<&Iface> {
        self.ifaces.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Iface> {
        self.ifaces.values()
    }

    pub fn iter_with_id(&self) -> impl Iterator<Item = (usize, &Iface)> {
        self.ifaces.iter().map(|(&id, iface)| (id, iface))
    }

    pub fn len(&self) -> usize {
        self.ifaces.len()
    }

    fn generate_unique_mac(&self) -> [u8; 6] {
        let mut rng = rand::thread_rng();
        loop {
            let mut mac = [0u8; 6];
            rng.fill(&mut mac[..]);
            mac[0] = 0x02; // locally administered, unicast
            let exists = self.ifaces.values().any(|i| i.mac == mac);
            if !exists {
                return mac;
            }
        }
    }

    pub fn id_by_name(
        &self,
        name: &str,
    ) -> Option<usize> {
        self.ifaces
            .iter()
            .find(|(_, iface)| iface.name == name)
            .map(|(id, _)| *id)
    }


    pub fn set_socket(
        &mut self,
        id: usize,
        socket: Socket,
    ) {
        if let Some(iface) = self.ifaces.get_mut(&id) {
            iface.soc = Some(socket);
        }
    }


    pub async fn send_on(
        &self,
        id: usize,
        packet: Vec<u8>,
    ) {
        if let Some(iface) = self.ifaces.get(&id) {
            if let Some(socket) = &iface.soc {
                socket.send(packet).await;
            }
        }
    }


    pub fn listen(
        &self,
        device_name: &str,
    ) {
        for iface in self.ifaces.values() {
            println!(
                "{device_name} listening on {}",
                iface.name
            );
        }
    }


    // ============================================================
    // Attend simultanément sur toutes les interfaces.
    // ============================================================

    pub async fn recv(
        &mut self,
    ) -> Option<(usize, Vec<u8>)> {
        loop {
            let futures = self
                .ifaces
                .iter_mut()
                .filter_map(|(id, iface)| {
                    iface.soc.as_mut().map(|socket| {
                        let id = *id;

                        async move {
                            let packet = socket.recv().await;
                            (id, packet)
                        }
                        .boxed()
                    })
                })
                .collect::<Vec<_>>();

            if futures.is_empty() {
                return None;
            }

            let ((id, packet), _, _) =
                select_all(futures).await;

            match packet {
                Some(packet) => {
                    return Some((id, packet));
                }

                None => {
                    // Un socket est fermé.
                    // On reconstruit les futures pour
                    // continuer à écouter les autres.
                    continue;
                }
            }
        }
    }
}


// ================================================================
// ======================= Link interfaces ========================
// ================================================================

use crate::devices::device::Device;

pub async fn link_interfaces(
    a: &impl Device,
    a_iface: &str,
    b: &impl Device,
    b_iface: &str,
) {
    let a_id = a
        .get_iface_id(a_iface)
        .await
        .expect("interface inconnue côté A");

    let b_id = b
        .get_iface_id(b_iface)
        .await
        .expect("interface inconnue côté B");

    let (soc_a, soc_b) = Socket::pair(8);

    a.set_socket(a_id, soc_a).await;
    b.set_socket(b_id, soc_b).await;
}