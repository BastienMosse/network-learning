use crate::events::EventKind;
use crate::devices::hub::Hub;


impl Hub {
    pub(crate) async fn handle_packet(&mut self, in_idx: usize, packet: Vec<u8>) {
        let ifaces = self.ifaces.lock().await;
        let ids: Vec<usize> = ifaces.ids().filter(|&id| id != in_idx).collect();
        let count = ids.len();

        self.events.emit(&self.name, EventKind::HubFlooded { port_count: count });

        for id in ids {
            ifaces.send_on(id, packet.clone()).await;
        }
    }
}
