use crate::utils::addrs::mac_addr::MacAddr;
use crate::utils::addrs::ip_addr::Ipv4Addr;

use std::time::{Duration, Instant};
use std::collections::HashMap;

pub const ARP_ENTRY_TTL: Duration = Duration::from_secs(300);

#[derive(Debug, Clone)]
struct ArpEntry {
    mac: MacAddr,
    learned_at: Instant,
}

#[derive(Debug, Default)]
pub struct ArpTable {
    entries: HashMap<Ipv4Addr, ArpEntry>,
}

impl ArpTable {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub fn learn(&mut self, ip: Ipv4Addr, mac: MacAddr) {
        self.entries.insert(ip, ArpEntry { mac, learned_at: Instant::now() });
    }

    pub fn lookup(&mut self, ip: &Ipv4Addr) -> Option<MacAddr> {
        match self.entries.get(ip) {
            Some(entry) if entry.learned_at.elapsed() < ARP_ENTRY_TTL => Some(entry.mac),
            Some(_) => { self.entries.remove(ip); None }
            None => None,
        }
    }

    pub fn purge_expired(&mut self) {
        self.entries.retain(|_, e| e.learned_at.elapsed() < ARP_ENTRY_TTL);
    }

    pub fn entries(&self) -> impl Iterator<Item = (&Ipv4Addr, MacAddr, Duration)> {
        self.entries.iter().map(|(ip, e)| (ip, e.mac, e.learned_at.elapsed()))
    }

    pub fn remove(&mut self, ip: &Ipv4Addr) {
        self.entries.remove(ip);
    }
}