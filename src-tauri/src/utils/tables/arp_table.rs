use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::utils::addrs::mac_addr::MacAddr;
use crate::utils::addrs::ip_addr::Ipv4Addr;

/// Durée de vie d'une entrée avant expiration (valeur typique côté OS réels).
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

    /// Apprend ou rafraîchit une correspondance IP -> MAC (ex: reçue via une ARP reply).
    pub fn learn(&mut self, ip: Ipv4Addr, mac: MacAddr) {
        self.entries.insert(ip, ArpEntry { mac, learned_at: Instant::now() });
    }

    /// Cherche la MAC pour une IP. Retourne None si absente ou expirée (et la retire alors).
    pub fn lookup(&mut self, ip: &Ipv4Addr) -> Option<MacAddr> {
        match self.entries.get(ip) {
            Some(entry) if entry.learned_at.elapsed() < ARP_ENTRY_TTL => Some(entry.mac),
            Some(_) => { self.entries.remove(ip); None }
            None => None,
        }
    }

    /// À appeler périodiquement (ex: tokio::time::interval) pour nettoyer les entrées mortes.
    pub fn purge_expired(&mut self) {
        self.entries.retain(|_, e| e.learned_at.elapsed() < ARP_ENTRY_TTL);
    }

    /// Pour la commande "show arp".
    pub fn entries(&self) -> impl Iterator<Item = (&Ipv4Addr, MacAddr, Duration)> {
        self.entries.iter().map(|(ip, e)| (ip, e.mac, e.learned_at.elapsed()))
    }

    pub fn remove(&mut self, ip: &Ipv4Addr) {
        self.entries.remove(ip);
    }
}