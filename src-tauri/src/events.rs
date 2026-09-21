use crate::utils::addrs::ip_addr::Ipv4Addr;
use crate::utils::addrs::mac_addr::MacAddr;

use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, oneshot};


// ============================================================
// Event types
// ============================================================

#[derive(Debug, Clone)]
pub enum EventKind {
    // ARP
    ArpLookup { ip: Ipv4Addr, found: bool },
    ArpRequestSent { target_ip: Ipv4Addr },
    ArpRequestReceived { sender_ip: Ipv4Addr, sender_mac: MacAddr },
    ArpReplySent { target_ip: Ipv4Addr, mac: MacAddr },
    ArpReplyReceived { sender_ip: Ipv4Addr, sender_mac: MacAddr },
    ArpTableUpdate { ip: Ipv4Addr, mac: MacAddr },

    // ICMP
    IcmpEchoRequest { src: Ipv4Addr, dst: Ipv4Addr },
    IcmpEchoReply { src: Ipv4Addr, dst: Ipv4Addr },

    // L2
    FrameSent { iface: String, dst_mac: MacAddr },
    FrameReceived { iface: String, src_mac: MacAddr },
    FrameDropped { reason: String },

    // Bridge / Switch
    MacTableLearn { mac: MacAddr, port: usize },
    MacTableLookup { mac: MacAddr, found: bool },
    FrameFlooded { port_count: usize },
    FrameForwarded { out_port: usize },

    // Hub
    HubFlooded { port_count: usize },

    // Router
    RouteFound { dst: Ipv4Addr, next_hop: Ipv4Addr, iface: String },
    RouteNotFound { dst: Ipv4Addr },
    TtlExpired { src: Ipv4Addr, dst: Ipv4Addr },

    // Generic
    Info { message: String },
}

impl fmt::Display for EventKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ArpLookup { ip, found } => {
                if *found { write!(f, "ARP lookup {ip} -> hit") }
                else { write!(f, "ARP lookup {ip} -> miss") }
            }
            Self::ArpRequestSent { target_ip } =>
                write!(f, "ARP request: who has {target_ip}?"),
            Self::ArpRequestReceived { sender_ip, sender_mac } =>
                write!(f, "ARP request from {sender_ip} ({sender_mac})"),
            Self::ArpReplySent { target_ip, mac } =>
                write!(f, "ARP reply: {target_ip} is at {mac}"),
            Self::ArpReplyReceived { sender_ip, sender_mac } =>
                write!(f, "ARP reply: {sender_ip} is at {sender_mac}"),
            Self::ArpTableUpdate { ip, mac } =>
                write!(f, "ARP table updated: {ip} -> {mac}"),
            Self::IcmpEchoRequest { src, dst } =>
                write!(f, "ICMP echo request {src} -> {dst}"),
            Self::IcmpEchoReply { src, dst } =>
                write!(f, "ICMP echo reply {src} -> {dst}"),
            Self::FrameSent { iface, dst_mac } =>
                write!(f, "frame sent on {iface} -> {dst_mac}"),
            Self::FrameReceived { iface, src_mac } =>
                write!(f, "frame received on {iface} from {src_mac}"),
            Self::FrameDropped { reason } =>
                write!(f, "frame dropped: {reason}"),
            Self::MacTableLearn { mac, port } =>
                write!(f, "MAC table: learned {mac} on port {port}"),
            Self::MacTableLookup { mac, found } => {
                if *found { write!(f, "MAC lookup {mac} -> hit") }
                else { write!(f, "MAC lookup {mac} -> miss") }
            }
            Self::FrameFlooded { port_count } =>
                write!(f, "flooding frame to {port_count} ports"),
            Self::FrameForwarded { out_port } =>
                write!(f, "forwarding to port {out_port}"),
            Self::HubFlooded { port_count } =>
                write!(f, "hub: repeating to {port_count} ports"),
            Self::RouteFound { dst, next_hop, iface } =>
                write!(f, "route for {dst}: via {next_hop} on {iface}"),
            Self::RouteNotFound { dst } =>
                write!(f, "no route to {dst}"),
            Self::TtlExpired { src, dst } =>
                write!(f, "TTL expired: {src} -> {dst}"),
            Self::Info { message } =>
                write!(f, "{message}"),
        }
    }
}


// ============================================================
// State snapshots (displayed at each pause)
// ============================================================

#[derive(Debug, Clone)]
pub struct TableView {
    pub name: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl fmt::Display for TableView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  {}:", self.name)?;
        if self.rows.is_empty() {
            writeln!(f, "    (empty)")?;
            return Ok(());
        }
        writeln!(f, "    {}", self.headers.join(" | "))?;
        for row in &self.rows {
            writeln!(f, "    {}", row.join(" | "))?;
        }
        Ok(())
    }
}


// ============================================================
// Step: one pause point in the simulation
// ============================================================

#[derive(Debug, Clone)]
pub struct Step {
    pub device: String,
    pub kind: EventKind,
    pub tables: Vec<TableView>,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[{}] {}", self.device, self.kind)?;
        for table in &self.tables {
            write!(f, "{table}")?;
        }
        Ok(())
    }
}


// ============================================================
// EventBus: logging (non-blocking) + step-by-step (blocking)
// ============================================================

#[derive(Clone)]
pub struct EventBus {
    log_tx: broadcast::Sender<Step>,
    step_tx: mpsc::Sender<(Step, oneshot::Sender<()>)>,
    step_mode: Arc<AtomicBool>,
}

impl EventBus {
    pub fn new() -> (Self, StepReceiver) {
        let (log_tx, _) = broadcast::channel(256);
        let (step_tx, step_rx) = mpsc::channel(64);

        let bus = Self {
            log_tx,
            step_tx,
            step_mode: Arc::new(AtomicBool::new(false)),
        };

        let receiver = StepReceiver { rx: step_rx };
        (bus, receiver)
    }

    pub fn log_only() -> Self {
        let (log_tx, _) = broadcast::channel(256);
        let (step_tx, _) = mpsc::channel(1);
        Self {
            log_tx,
            step_tx,
            step_mode: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_step_mode(&self, enabled: bool) {
        self.step_mode.store(enabled, Ordering::Relaxed);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Step> {
        self.log_tx.subscribe()
    }

    pub fn emit(&self, device: &str, kind: EventKind) {
        let step = Step {
            device: device.to_string(),
            kind,
            tables: Vec::new(),
        };
        let _ = self.log_tx.send(step);
    }

    pub async fn step(&self, device: &str, kind: EventKind, tables: Vec<TableView>) {
        let step = Step {
            device: device.to_string(),
            kind,
            tables,
        };

        let _ = self.log_tx.send(step.clone());

        if self.step_mode.load(Ordering::Relaxed) {
            let (resume_tx, resume_rx) = oneshot::channel();
            let _ = self.step_tx.send((step, resume_tx)).await;
            let _ = resume_rx.await;
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::log_only()
    }
}


// ============================================================
// StepReceiver: the controller side (CLI / frontend)
// ============================================================

pub struct StepReceiver {
    rx: mpsc::Receiver<(Step, oneshot::Sender<()>)>,
}

impl StepReceiver {
    pub async fn next_step(&mut self) -> Option<StepHandle> {
        let (step, resume) = self.rx.recv().await?;
        Some(StepHandle { step, resume: Some(resume) })
    }
}

pub struct StepHandle {
    pub step: Step,
    resume: Option<oneshot::Sender<()>>,
}

impl StepHandle {
    pub fn resume(mut self) {
        if let Some(tx) = self.resume.take() {
            let _ = tx.send(());
        }
    }
}

impl Drop for StepHandle {
    fn drop(&mut self) {
        if let Some(tx) = self.resume.take() {
            let _ = tx.send(());
        }
    }
}
