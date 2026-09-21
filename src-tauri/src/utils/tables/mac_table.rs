use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::utils::addrs::mac_addr::MacAddr;

pub const MAC_ENTRY_TTL: Duration = Duration::from_secs(300);

#[derive(Debug, Clone)]
struct MacEntry {
    iface_id: usize,
    learned_at: Instant,
}

#[derive(Debug, Default)]
pub struct MacTable {
    entries: HashMap<MacAddr, MacEntry>,
}

impl MacTable {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub fn learn(&mut self, mac: MacAddr, iface_id: usize) {
        self.entries.insert(mac, MacEntry { iface_id, learned_at: Instant::now() });
    }

    pub fn lookup(&mut self, mac: &MacAddr) -> Option<usize> {
        match self.entries.get(mac) {
            Some(entry) if entry.learned_at.elapsed() < MAC_ENTRY_TTL => Some(entry.iface_id),
            Some(_) => { self.entries.remove(mac); None }
            None => None,
        }
    }

    pub fn purge_expired(&mut self) {
        self.entries.retain(|_, e| e.learned_at.elapsed() < MAC_ENTRY_TTL);
    }

    pub fn entries(&self) -> impl Iterator<Item = (MacAddr, usize)> + '_ {
        self.entries.iter().map(|(mac, e)| (*mac, e.iface_id))
    }

    pub fn remove_port(&mut self, port: usize) {
        self.entries.retain(|_, e| e.iface_id != port);
    }
}