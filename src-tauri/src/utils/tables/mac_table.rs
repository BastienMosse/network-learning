use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::utils::addrs::mac_addr::MacAddr;

/// Aging time par défaut (300s = valeur par défaut sur la plupart des switches réels).
pub const MAC_ENTRY_TTL: Duration = Duration::from_secs(300);

#[derive(Debug, Clone)]
struct MacEntry {
    port: usize,
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

    /// Apprentissage : à chaque trame reçue, associer la MAC source au port d'entrée.
    pub fn learn(&mut self, mac: MacAddr, port: usize) {
        self.entries.insert(mac, MacEntry { port, learned_at: Instant::now() });
    }

    /// Cherche le port de sortie pour une MAC destination.
    pub fn lookup(&mut self, mac: &MacAddr) -> Option<usize> {
        match self.entries.get(mac) {
            Some(entry) if entry.learned_at.elapsed() < MAC_ENTRY_TTL => Some(entry.port),
            Some(_) => { self.entries.remove(mac); None }
            None => None,
        }
    }

    pub fn purge_expired(&mut self) {
        self.entries.retain(|_, e| e.learned_at.elapsed() < MAC_ENTRY_TTL);
    }

    /// Pour la commande "show mac address-table".
    pub fn entries(&self) -> impl Iterator<Item = (MacAddr, usize, Duration)> + '_ {
        self.entries.iter().map(|(mac, e)| (*mac, e.port, e.learned_at.elapsed()))
    }

    /// Utile si un lien tombe : purge tout ce qui pointait vers ce port.
    pub fn remove_port(&mut self, port: usize) {
        self.entries.retain(|_, e| e.port != port);
    }
}