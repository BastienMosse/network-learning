use crate::utils::addrs::ip_addr::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct Route {
    pub network: u32,
    pub mask: u32,
    pub next_hop: Ipv4Addr,
    pub iface_id: usize,
    pub iface_name: String,
}

#[derive(Debug, Default)]
pub struct RoutingTable {
    routes: Vec<Route>,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add(&mut self, network: Ipv4Addr, prefix_len: u8, next_hop: Ipv4Addr, iface_id: usize, iface_name: &str) {
        let mask = if prefix_len == 0 { 0 } else { !0u32 << (32 - prefix_len) };
        self.routes.push(Route {
            network: u32::from(network) & mask,
            mask,
            next_hop,
            iface_id,
            iface_name: iface_name.to_string(),
        });
    }

    pub fn lookup(&self, dst: Ipv4Addr) -> Option<&Route> {
        let dst_u32 = u32::from(dst);
        self.routes.iter()
            .filter(|r| (dst_u32 & r.mask) == r.network)
            .max_by_key(|r| r.mask.count_ones())
    }

    pub fn entries(&self) -> impl Iterator<Item = &Route> {
        self.routes.iter()
    }

    pub fn remove(&mut self, network: Ipv4Addr, prefix_len: u8) {
        let mask = if prefix_len == 0 { 0 } else { !0u32 << (32 - prefix_len) };
        let net = u32::from(network) & mask;
        self.routes.retain(|r| r.network != net || r.mask != mask);
    }
}
