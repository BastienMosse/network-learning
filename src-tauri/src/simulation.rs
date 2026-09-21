use crate::devices::device::Device;
use crate::devices::pc::PC;
use crate::devices::hub::Hub;
use crate::devices::switch::Switch;
use crate::devices::bridge::Bridge;
use crate::devices::router::Router;
use crate::events::{EventBus, StepReceiver};
use crate::utils::socket::Socket;

use std::collections::HashMap;
use tokio::task::JoinHandle;


pub const MAX_IFACES_PC: usize = 4;
pub const MAX_IFACES_HUB: usize = 4;
pub const MAX_IFACES_SWITCH: usize = 8;
pub const MAX_IFACES_BRIDGE: usize = 2;
pub const MAX_IFACES_ROUTER: usize = 4;


pub enum AnyDevice {
    Pc(PC),
    Hub(Hub),
    Switch(Switch),
    Bridge(Bridge),
    Router(Router),
}

impl AnyDevice {
    fn max_ifaces(&self) -> usize {
        match self {
            Self::Pc(_) => MAX_IFACES_PC,
            Self::Hub(_) => MAX_IFACES_HUB,
            Self::Switch(_) => MAX_IFACES_SWITCH,
            Self::Bridge(_) => MAX_IFACES_BRIDGE,
            Self::Router(_) => MAX_IFACES_ROUTER,
        }
    }

    fn type_name(&self) -> &str {
        match self {
            Self::Pc(_) => "pc",
            Self::Hub(_) => "hub",
            Self::Switch(_) => "switch",
            Self::Bridge(_) => "bridge",
            Self::Router(_) => "router",
        }
    }

    async fn iface_count(&self) -> usize {
        match self {
            Self::Pc(d) => d.ifaces.lock().await.len(),
            Self::Hub(d) => d.ifaces.lock().await.len(),
            Self::Switch(d) => d.ifaces.lock().await.len(),
            Self::Bridge(d) => d.ifaces.lock().await.len(),
            Self::Router(d) => d.ifaces.lock().await.len(),
        }
    }

    async fn has_iface_name(&self, name: &str) -> bool {
        match self {
            Self::Pc(d) => d.ifaces.lock().await.id_by_name(name).is_some(),
            Self::Hub(d) => d.ifaces.lock().await.id_by_name(name).is_some(),
            Self::Switch(d) => d.ifaces.lock().await.id_by_name(name).is_some(),
            Self::Bridge(d) => d.ifaces.lock().await.id_by_name(name).is_some(),
            Self::Router(d) => d.ifaces.lock().await.id_by_name(name).is_some(),
        }
    }

    async fn add_interface(&self, name: &str, ip: &str, mask: u8, mtu: u16) -> Result<(usize, [u8; 6]), String> {
        if self.has_iface_name(name).await {
            return Err(format!("l'interface '{name}' existe déjà sur cet appareil"));
        }
        let count = self.iface_count().await;
        if count >= self.max_ifaces() {
            return Err(format!(
                "{} supporte au maximum {} interfaces",
                self.type_name(), self.max_ifaces()
            ));
        }

        let result = match self {
            Self::Pc(d) => d.add_interface(name, ip, mask, mtu).await,
            Self::Hub(d) => d.add_interface(name, ip, mask, mtu).await,
            Self::Switch(d) => d.add_interface(name, ip, mask, mtu).await,
            Self::Bridge(d) => d.add_interface(name, ip, mask, mtu).await,
            Self::Router(d) => d.add_interface(name, ip, mask, mtu).await,
        };
        Ok(result)
    }

    async fn get_iface_id(&self, name: &str) -> Option<usize> {
        match self {
            Self::Pc(d) => d.get_iface_id(name).await,
            Self::Hub(d) => d.get_iface_id(name).await,
            Self::Switch(d) => d.get_iface_id(name).await,
            Self::Bridge(d) => d.get_iface_id(name).await,
            Self::Router(d) => d.get_iface_id(name).await,
        }
    }

    async fn set_socket(&self, id: usize, socket: Socket) {
        match self {
            Self::Pc(d) => d.set_socket(id, socket).await,
            Self::Hub(d) => d.set_socket(id, socket).await,
            Self::Switch(d) => d.set_socket(id, socket).await,
            Self::Bridge(d) => d.set_socket(id, socket).await,
            Self::Router(d) => d.set_socket(id, socket).await,
        }
    }

    fn start(&mut self) -> JoinHandle<()> {
        match self {
            Self::Pc(d) => d.start(),
            Self::Hub(d) => d.start(),
            Self::Switch(d) => d.start(),
            Self::Bridge(d) => d.start(),
            Self::Router(d) => d.start(),
        }
    }

    async fn edit_interface(
        &self,
        old_name: &str,
        new_name: Option<&str>,
        ip: Option<&str>,
        mask: Option<u8>,
        mtu: Option<u16>,
    ) -> Result<(), String> {
        let ifaces_arc = match self {
            Self::Pc(d) => &d.ifaces,
            Self::Hub(d) => &d.ifaces,
            Self::Switch(d) => &d.ifaces,
            Self::Bridge(d) => &d.ifaces,
            Self::Router(d) => &d.ifaces,
        };
        let mut ifaces = ifaces_arc.lock().await;
        let id = ifaces.id_by_name(old_name)
            .ok_or_else(|| format!("interface '{old_name}' introuvable"))?;

        if let Some(n) = new_name {
            if n != old_name {
                if ifaces.id_by_name(n).is_some() {
                    return Err(format!("l'interface '{n}' existe déjà"));
                }
            }
        }

        let addr = match ip {
            Some(s) => {
                let parsed: crate::utils::addrs::ip_addr::Ipv4Addr = s.parse()
                    .map_err(|_| format!("adresse IP invalide: {s}"))?;
                Some(u32::from(parsed))
            }
            None => None,
        };

        ifaces.update(id, new_name, addr, mask, mtu);
        Ok(())
    }

    fn set_name(&mut self, name: &str) {
        match self {
            Self::Pc(d) => d.name = name.to_string(),
            Self::Hub(d) => d.name = name.to_string(),
            Self::Switch(d) => d.name = name.to_string(),
            Self::Bridge(d) => d.name = name.to_string(),
            Self::Router(d) => d.name = name.to_string(),
        }
    }

    fn prepare_restart(&mut self) {
        match self {
            Self::Pc(d) => d.prepare_restart(),
            Self::Hub(d) => d.prepare_restart(),
            Self::Switch(d) => d.prepare_restart(),
            Self::Bridge(d) => d.prepare_restart(),
            Self::Router(d) => d.prepare_restart(),
        }
    }

    async fn exec(&self, input: &str) -> Result<String, String> {
        match self {
            Self::Pc(d) => Ok(d.exec(input).await),
            Self::Hub(d) => Ok(d.exec(input).await),
            Self::Switch(d) => Ok(d.exec(input).await),
            Self::Bridge(d) => Ok(d.exec(input).await),
            Self::Router(d) => Ok(d.exec(input).await),
        }
    }
}


pub struct SimulationManager {
    devices: HashMap<String, AnyDevice>,
    handles: Vec<JoinHandle<()>>,
    running: bool,
    event_bus: EventBus,
}

impl SimulationManager {
    pub fn new() -> (Self, StepReceiver) {
        let (event_bus, step_receiver) = EventBus::new();
        let mgr = Self {
            devices: HashMap::new(),
            handles: Vec::new(),
            running: false,
            event_bus,
        };
        (mgr, step_receiver)
    }

    pub fn set_step_mode(&self, enabled: bool) {
        self.event_bus.set_step_mode(enabled);
    }

    pub fn create_device(&mut self, name: &str, device_type: &str) -> Result<(), String> {
        if self.running {
            return Err("impossible d'ajouter un appareil pendant la simulation".into());
        }
        if self.devices.contains_key(name) {
            return Err(format!("l'appareil '{name}' existe déjà"));
        }
        let bus = self.event_bus.clone();
        let device = match device_type {
            "pc" => AnyDevice::Pc(PC::new(name).with_event_bus(bus)),
            "hub" => AnyDevice::Hub(Hub::new(name).with_event_bus(bus)),
            "switch" => AnyDevice::Switch(Switch::new(name).with_event_bus(bus)),
            "bridge" => AnyDevice::Bridge(Bridge::new(name).with_event_bus(bus)),
            "router" => AnyDevice::Router(Router::new(name).with_event_bus(bus)),
            _ => return Err(format!("type inconnu: {device_type}")),
        };
        self.devices.insert(name.to_string(), device);
        Ok(())
    }

    pub fn rename_device(&mut self, old_name: &str, new_name: &str) -> Result<(), String> {
        if self.running {
            return Err("impossible de renommer pendant la simulation".into());
        }
        if old_name == new_name {
            return Ok(());
        }
        if self.devices.contains_key(new_name) {
            return Err(format!("l'appareil '{new_name}' existe déjà"));
        }
        let mut device = self.devices.remove(old_name)
            .ok_or_else(|| format!("appareil '{old_name}' introuvable"))?;
        device.set_name(new_name);
        self.devices.insert(new_name.to_string(), device);
        Ok(())
    }

    pub fn remove_device(&mut self, name: &str) -> Result<(), String> {
        if self.running {
            return Err("impossible de supprimer un appareil pendant la simulation".into());
        }
        self.devices.remove(name)
            .map(|_| ())
            .ok_or_else(|| format!("appareil '{name}' introuvable"))
    }

    pub async fn add_interface(
        &self,
        device_name: &str,
        iface_name: &str,
        ip: &str,
        mask: u8,
        mtu: u16,
    ) -> Result<(usize, [u8; 6]), String> {
        let device = self.devices.get(device_name)
            .ok_or_else(|| format!("appareil '{device_name}' introuvable"))?;
        device.add_interface(iface_name, ip, mask, mtu).await
    }

    pub async fn edit_interface(
        &self,
        device_name: &str,
        old_iface_name: &str,
        new_name: Option<&str>,
        ip: Option<&str>,
        mask: Option<u8>,
        mtu: Option<u16>,
    ) -> Result<(), String> {
        let device = self.devices.get(device_name)
            .ok_or_else(|| format!("appareil '{device_name}' introuvable"))?;
        device.edit_interface(old_iface_name, new_name, ip, mask, mtu).await
    }

    pub async fn link_interfaces(
        &self,
        dev_a: &str,
        iface_a: &str,
        dev_b: &str,
        iface_b: &str,
    ) -> Result<(), String> {
        if dev_a == dev_b {
            return Err("impossible de lier un appareil à lui-même".into());
        }

        let a_id = {
            let device = self.devices.get(dev_a)
                .ok_or_else(|| format!("appareil '{dev_a}' introuvable"))?;
            device.get_iface_id(iface_a).await
                .ok_or_else(|| format!("interface '{iface_a}' introuvable sur '{dev_a}'"))?
        };

        let b_id = {
            let device = self.devices.get(dev_b)
                .ok_or_else(|| format!("appareil '{dev_b}' introuvable"))?;
            device.get_iface_id(iface_b).await
                .ok_or_else(|| format!("interface '{iface_b}' introuvable sur '{dev_b}'"))?
        };

        let (soc_a, soc_b) = Socket::pair(8);
        self.devices.get(dev_a).unwrap().set_socket(a_id, soc_a).await;
        self.devices.get(dev_b).unwrap().set_socket(b_id, soc_b).await;
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.running {
            return Err("simulation déjà en cours".into());
        }
        if self.devices.is_empty() {
            return Err("aucun appareil dans la topologie".into());
        }
        for device in self.devices.values_mut() {
            self.handles.push(device.start());
        }
        self.running = true;
        Ok(())
    }

    pub async fn exec(&self, device_name: &str, input: &str) -> Result<String, String> {
        if !self.running {
            return Err("la simulation n'est pas démarrée".into());
        }
        let device = self.devices.get(device_name)
            .ok_or_else(|| format!("appareil '{device_name}' introuvable"))?;
        device.exec(input).await
    }

    pub fn stop(&mut self) {
        for handle in self.handles.drain(..) {
            handle.abort();
        }
        for device in self.devices.values_mut() {
            device.prepare_restart();
        }
        self.running = false;
    }

    pub fn reset(&mut self) {
        self.stop();
        self.devices.clear();
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn max_interfaces(device_type: &str) -> usize {
        match device_type {
            "pc" => MAX_IFACES_PC,
            "hub" => MAX_IFACES_HUB,
            "switch" => MAX_IFACES_SWITCH,
            "bridge" => MAX_IFACES_BRIDGE,
            "router" => MAX_IFACES_ROUTER,
            _ => 0,
        }
    }
}
