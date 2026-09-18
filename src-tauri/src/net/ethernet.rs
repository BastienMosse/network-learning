use crate::utils::addrs::mac_addr::MacAddr;
use core::fmt;

pub const MIN_PAYLOAD_LEN: usize = 46;
pub const MAX_PAYLOAD_LEN: usize = 1500;
pub const ETH_HEADER_LEN: usize = 14;
pub const ETH_FCS_LEN: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ethernet {
    pub destination: MacAddr,
    pub source: MacAddr,
    pub ethertype: EtherType,
    pub payload: Vec<u8>,
    pub fcs: u32,
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}


impl Ethernet {
    pub fn new(
        destination: MacAddr,
        source: MacAddr,
        ethertype: EtherType,
        payload: Vec<u8>
    ) -> Self {
        let mut frame = Self {
            destination,
            source,
            ethertype,
            payload,
            fcs: 0,
        };

        frame.fcs = frame.compute_fcs();
        frame
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(
            ETH_HEADER_LEN + self.payload.len() + ETH_FCS_LEN
        );

        bytes.extend_from_slice(&self.destination.to_bytes());
        bytes.extend_from_slice(&self.source.to_bytes());
        bytes.extend_from_slice(&self.ethertype.to_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes.extend_from_slice(&self.fcs.to_le_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < ETH_HEADER_LEN + ETH_FCS_LEN {
            return Err("trame Ethernet trop courte".into());
        }

        let fcs_start = bytes.len() - ETH_FCS_LEN;

        let destination = MacAddr::from_bytes(&bytes[0..6])
            .map_err(|_| "MAC destination invalide".to_string())?;

        let source = MacAddr::from_bytes(&bytes[6..12])
            .map_err(|_| "MAC source invalide".to_string())?;

        let ethertype = EtherType::from_bytes(&bytes[12..14]);

        let payload = bytes[14..fcs_start].to_vec();

        let fcs = u32::from_le_bytes(bytes[fcs_start..].try_into()
                .map_err(|_| "FCS invalide".to_string())?,
        );

        Ok(Self {
            destination,
            source,
            ethertype,
            payload,
            fcs,
        })
    }

    pub fn compute_fcs(&self) -> u32 {
        let mut bytes = Vec::with_capacity(
            ETH_HEADER_LEN + self.payload.len()
        );

        bytes.extend_from_slice(&self.destination.to_bytes());
        bytes.extend_from_slice(&self.source.to_bytes());
        bytes.extend_from_slice(&self.ethertype.to_bytes());
        bytes.extend_from_slice(&self.payload);

        crc32(&bytes)
    }

    pub fn verify_fcs(&self) -> bool {
        self.compute_fcs() == self.fcs
    }
}


// --------------------------------------------------
// EtherType
// --------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EtherType(pub u16);

impl EtherType {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn to_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }

    pub const fn from_bytes(bytes: &[u8]) -> Self {
        Self(u16::from_be_bytes([bytes[0], bytes[1]]))
    }
}

impl fmt::Display for EtherType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:04x}", self.0)
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod EtherTypes {
    use super::EtherType;

    pub const Ipv4: EtherType = EtherType(0x0800);
    pub const Arp: EtherType = EtherType(0x0806);
    pub const Ipv6: EtherType = EtherType(0x86DD);
    pub const Vlan: EtherType = EtherType(0x8100);
}