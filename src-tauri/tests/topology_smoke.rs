use app::devices::{bridge::Bridge, device::Device, pc::PC};
use app::net::ethernet::{EtherType, Ethernet};
use app::utils::addrs::mac_addr::MacAddr;
use app::utils::interfaces::link_interfaces;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn demo() {
        let mut pc1 = PC::new("PC1");
        pc1.add_interface("eth0", "10.0.1.2", 24, 1500).await;

        let mut pc2 = PC::new("PC2");
        pc2.add_interface("eth0", "10.0.2.2", 24, 1500).await;

        let mut bridge = Bridge::new("Bridge");
        bridge.add_interface("eth0", "10.0.1.1", 24, 1500).await;
        bridge.add_interface("eth1", "10.0.2.1", 24, 1500).await;

        link_interfaces(&pc1, "eth0", &bridge, "eth0").await;
        link_interfaces(&pc2, "eth0", &bridge, "eth1").await;

        let t_pc1 = pc1.start();
        let t_pc2 = pc2.start();
        let t_bridge = bridge.start();

        // PC1 émet une trame vers une MAC inconnue -> le bridge doit flooder vers PC2
        let ethernet = Ethernet::new(
            "cc:cc:cc:cc:cc:cc".parse().unwrap(),
            "dd:dd:dd:dd:dd:dd".parse().unwrap(),
            EtherType(0x0800),
            vec![42, 10, 42, 4],
        );

        let frame = ethernet.to_bytes();

        let pc1_eth0 = pc1.get_iface_id("eth0").await.unwrap();
        pc1.send_frame(pc1_eth0, frame.clone()).await;
        tokio::time::sleep(Duration::from_millis(50)).await;

        // PC2 doit avoir reçu la trame
        assert_eq!(pc2.list_packets().await, vec![frame]);

        t_pc1.abort();
        t_pc2.abort();
        t_bridge.abort();
    }
}
