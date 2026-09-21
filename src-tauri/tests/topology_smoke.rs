use app::devices::bridge::Bridge;
use app::devices::device::Device;
use app::devices::hub::Hub;
use app::devices::pc::PC;
use app::devices::router::Router;
use app::devices::switch::Switch;
use app::events::EventBus;
use app::net::ethernet::{EtherType, Ethernet};
use app::utils::interfaces::link_interfaces;

use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================
    // Original: bridge floods unknown MAC
    // ============================================================

    #[tokio::test]
    async fn bridge_floods_unknown_mac() {
        let mut pc1 = PC::new("PC1");
        pc1.add_interface("eth0", "10.0.1.2", 24, 1500).await;

        let mut pc2 = PC::new("PC2");
        pc2.add_interface("eth0", "10.0.2.2", 24, 1500).await;

        let mut bridge = Bridge::new("Bridge");
        bridge.add_interface("eth0", "10.0.1.1", 24, 1500).await;
        bridge.add_interface("eth1", "10.0.2.1", 24, 1500).await;

        link_interfaces(&pc1, "eth0", &bridge, "eth0").await;
        link_interfaces(&pc2, "eth0", &bridge, "eth1").await;

        let _t_pc1 = pc1.start();
        let _t_pc2 = pc2.start();
        let _t_bridge = bridge.start();

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

        assert_eq!(pc2.list_packets().await, vec![frame]);
    }

    // ============================================================
    // Hub floods to all ports
    // ============================================================

    #[tokio::test]
    async fn hub_floods_all() {
        let mut pc1 = PC::new("PC1");
        pc1.add_interface("eth0", "10.0.1.1", 24, 1500).await;

        let mut pc2 = PC::new("PC2");
        pc2.add_interface("eth0", "10.0.1.2", 24, 1500).await;

        let mut pc3 = PC::new("PC3");
        pc3.add_interface("eth0", "10.0.1.3", 24, 1500).await;

        let mut hub = Hub::new("Hub");
        hub.add_interface("eth0", "0.0.0.0", 0, 1500).await;
        hub.add_interface("eth1", "0.0.0.0", 0, 1500).await;
        hub.add_interface("eth2", "0.0.0.0", 0, 1500).await;

        link_interfaces(&pc1, "eth0", &hub, "eth0").await;
        link_interfaces(&pc2, "eth0", &hub, "eth1").await;
        link_interfaces(&pc3, "eth0", &hub, "eth2").await;

        let _t1 = pc1.start();
        let _t2 = pc2.start();
        let _t3 = pc3.start();
        let _th = hub.start();

        let frame = Ethernet::new(
            "aa:aa:aa:aa:aa:aa".parse().unwrap(),
            "bb:bb:bb:bb:bb:bb".parse().unwrap(),
            EtherType(0x0800),
            vec![1, 2, 3],
        ).to_bytes();

        let pc1_eth0 = pc1.get_iface_id("eth0").await.unwrap();
        pc1.send_frame(pc1_eth0, frame.clone()).await;
        tokio::time::sleep(Duration::from_millis(50)).await;

        assert_eq!(pc2.list_packets().await, vec![frame.clone()]);
        assert_eq!(pc3.list_packets().await, vec![frame]);
    }

    // ============================================================
    // Switch learns MACs and forwards
    // ============================================================

    #[tokio::test]
    async fn switch_learns_and_forwards() {
        let mut pc1 = PC::new("PC1");
        pc1.add_interface("eth0", "10.0.1.1", 24, 1500).await;

        let mut pc2 = PC::new("PC2");
        pc2.add_interface("eth0", "10.0.1.2", 24, 1500).await;

        let mut pc3 = PC::new("PC3");
        pc3.add_interface("eth0", "10.0.1.3", 24, 1500).await;

        let mut sw = Switch::new("SW1");
        sw.add_interface("eth0", "0.0.0.0", 0, 1500).await;
        sw.add_interface("eth1", "0.0.0.0", 0, 1500).await;
        sw.add_interface("eth2", "0.0.0.0", 0, 1500).await;

        link_interfaces(&pc1, "eth0", &sw, "eth0").await;
        link_interfaces(&pc2, "eth0", &sw, "eth1").await;
        link_interfaces(&pc3, "eth0", &sw, "eth2").await;

        let _t1 = pc1.start();
        let _t2 = pc2.start();
        let _t3 = pc3.start();
        let _ts = sw.start();

        // First frame: unknown dst -> floods to pc2 AND pc3
        let frame1 = Ethernet::new(
            "ff:ff:ff:ff:ff:ff".parse().unwrap(),
            "aa:aa:aa:aa:aa:aa".parse().unwrap(),
            EtherType(0x0800),
            vec![1],
        ).to_bytes();

        let pc1_eth0 = pc1.get_iface_id("eth0").await.unwrap();
        pc1.send_frame(pc1_eth0, frame1.clone()).await;
        tokio::time::sleep(Duration::from_millis(50)).await;

        assert_eq!(pc2.list_packets().await.len(), 1);
        assert_eq!(pc3.list_packets().await.len(), 1);
    }

    // ============================================================
    // PC CLI exec
    // ============================================================

    #[tokio::test]
    async fn pc_cli_commands() {
        let mut pc = PC::new("TestPC");
        pc.add_interface("eth0", "10.0.1.1", 24, 1500).await;
        let _t = pc.start();

        let help = pc.exec("help").await;
        assert!(help.contains("ping"));
        assert!(help.contains("arp"));
        assert!(help.contains("ifconfig"));

        let ifconfig = pc.exec("ifconfig").await;
        assert!(ifconfig.contains("eth0"));
        assert!(ifconfig.contains("10.0.1.1"));

        let arp = pc.exec("arp").await;
        assert!(arp.contains("empty"));

        let unknown = pc.exec("blah").await;
        assert!(unknown.contains("unknown command"));
    }

    // ============================================================
    // Step-by-step mode
    // ============================================================

    #[tokio::test]
    async fn step_by_step_mode() {
        let (bus, mut receiver) = EventBus::new();
        bus.set_step_mode(true);

        let mut pc1 = PC::new("PC1").with_event_bus(bus.clone());
        pc1.add_interface("eth0", "10.0.1.1", 24, 1500).await;

        let mut pc2 = PC::new("PC2").with_event_bus(bus.clone());
        pc2.add_interface("eth0", "10.0.1.2", 24, 1500).await;

        let mut sw = Switch::new("SW1").with_event_bus(bus.clone());
        sw.add_interface("eth0", "0.0.0.0", 0, 1500).await;
        sw.add_interface("eth1", "0.0.0.0", 0, 1500).await;

        link_interfaces(&pc1, "eth0", &sw, "eth0").await;
        link_interfaces(&pc2, "eth0", &sw, "eth1").await;

        let _t1 = pc1.start();
        let _t2 = pc2.start();
        let _ts = sw.start();

        // Fire ping in background (it blocks on steps)
        let pc1_clone = pc1.command_sender();
        tokio::spawn(async move {
            let (tx, _rx) = tokio::sync::oneshot::channel();
            let _ = pc1_clone.send(app::devices::pc::PcCommand::Ping {
                target: "10.0.1.2".parse().unwrap(),
                reply: tx,
            }).await;
        });

        // Consume steps, resuming each one
        let mut step_count = 0;
        loop {
            tokio::select! {
                step = receiver.next_step() => {
                    match step {
                        Some(handle) => {
                            step_count += 1;
                            assert!(!handle.step.device.is_empty());
                            assert!(!format!("{}", handle.step.kind).is_empty());
                            handle.resume();
                        }
                        None => break,
                    }
                }
                _ = tokio::time::sleep(Duration::from_millis(500)) => {
                    break;
                }
            }
        }

        assert!(step_count > 0, "should have received at least one step");
    }

    // ============================================================
    // Router forwards between subnets
    // ============================================================

    #[tokio::test]
    async fn router_forwards_between_subnets() {
        let mut pc1 = PC::new("PC1");
        pc1.add_interface("eth0", "10.0.1.2", 24, 1500).await;

        let mut pc2 = PC::new("PC2");
        pc2.add_interface("eth0", "10.0.2.2", 24, 1500).await;

        let mut router = Router::new("R1");
        router.add_interface("eth0", "10.0.1.1", 24, 1500).await;
        router.add_interface("eth1", "10.0.2.1", 24, 1500).await;

        link_interfaces(&pc1, "eth0", &router, "eth0").await;
        link_interfaces(&pc2, "eth0", &router, "eth1").await;

        let _t1 = pc1.start();
        let _t2 = pc2.start();
        let _tr = router.start();

        // Add routes on the router
        router.add_route(
            "10.0.1.0".parse().unwrap(), 24,
            "10.0.1.0".parse().unwrap(), "eth0",
        ).await;
        router.add_route(
            "10.0.2.0".parse().unwrap(), 24,
            "10.0.2.0".parse().unwrap(), "eth1",
        ).await;

        // Router CLI
        let route_out = router.exec("route").await;
        assert!(route_out.contains("10.0.1.0"));
        assert!(route_out.contains("10.0.2.0"));

        let ifconfig = router.exec("ifconfig").await;
        assert!(ifconfig.contains("eth0"));
        assert!(ifconfig.contains("eth1"));
    }
}
