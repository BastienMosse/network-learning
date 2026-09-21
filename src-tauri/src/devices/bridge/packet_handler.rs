use crate::events::{EventKind, TableView};
use crate::net::ethernet::Ethernet;
use crate::devices::bridge::Bridge;


impl Bridge {
    pub(crate) fn mac_table_view(&self) -> TableView {
        let mut rows = Vec::new();
        for (mac, port) in self.mac_table.entries() {
            rows.push(vec![mac.to_string(), format!("port {port}")]);
        }
        TableView {
            name: "MAC Table".into(),
            headers: vec!["MAC".into(), "Port".into()],
            rows,
        }
    }

    pub(crate) async fn handle_packet(&mut self, in_idx: usize, packet: Vec<u8>) {
        let frame = match Ethernet::from_bytes(&packet) {
            Ok(f) => f,
            Err(_) => return,
        };

        self.mac_table.learn(frame.source, in_idx);
        self.events.step(
            &self.name,
            EventKind::MacTableLearn { mac: frame.source, port: in_idx },
            vec![self.mac_table_view()],
        ).await;

        let dst = frame.destination;

        if dst.is_broadcast() {
            let ifaces = self.ifaces.lock().await;
            let ids: Vec<usize> = ifaces.ids().filter(|&id| id != in_idx).collect();
            let count = ids.len();
            self.events.step(
                &self.name,
                EventKind::FrameFlooded { port_count: count },
                vec![self.mac_table_view()],
            ).await;
            for id in ids {
                ifaces.send_on(id, packet.clone()).await;
            }
            return;
        }

        let port = self.mac_table.lookup(&dst);
        self.events.step(
            &self.name,
            EventKind::MacTableLookup { mac: dst, found: port.is_some() },
            vec![self.mac_table_view()],
        ).await;

        let ifaces = self.ifaces.lock().await;
        match port {
            Some(out_idx) if out_idx != in_idx => {
                self.events.step(
                    &self.name,
                    EventKind::FrameForwarded { out_port: out_idx },
                    vec![self.mac_table_view()],
                ).await;
                ifaces.send_on(out_idx, packet).await;
            }
            Some(_) => {}
            None => {
                let ids: Vec<usize> = ifaces.ids().filter(|&id| id != in_idx).collect();
                let count = ids.len();
                self.events.step(
                    &self.name,
                    EventKind::FrameFlooded { port_count: count },
                    vec![self.mac_table_view()],
                ).await;
                for id in ids {
                    ifaces.send_on(id, packet.clone()).await;
                }
            }
        }
    }
}
