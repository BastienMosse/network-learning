use crate::utils::addrs::ip_addr::Ipv4Addr;
use pnet_macros_support::types::*;

pub const IPV4_HEADER_LEN: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ipv4 {
    pub version: u4,
    pub header_length: u4,
    pub dscp: u6,
    pub ecn: u2,
    pub total_length: u16be,
    pub identification: u16be,
    pub flags: u3,
    pub fragment_offset: u13be,
    pub ttl: u8,
    pub next_level_protocol: IpNextHeaderProtocol,
    pub checksum: u16be,
    pub source: Ipv4Addr,
    pub destination: Ipv4Addr,
    pub payload: Vec<u8>
}

impl Ipv4 {
    pub fn new(
        source: Ipv4Addr,
        destination: Ipv4Addr,
        protocol: IpNextHeaderProtocol,
        payload: Vec<u8>,
    ) -> Self {
        let mut packet = Self {
            version: 4,
            header_length: 5,
            dscp: 0,
            ecn: 0,
            total_length: (IPV4_HEADER_LEN + payload.len()) as u16,
            identification: 0,
            flags: 0,
            fragment_offset: 0,
            ttl: 64,
            next_level_protocol: protocol,
            checksum: 0,
            source,
            destination,
            payload,
        };

        packet.recompute_checksum();
        packet
    }

    /// Sérialise l'en-tête IPv4 + payload.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.total_length as usize);

        // Version + IHL
        bytes.push((self.version << 4) | self.header_length);

        // DSCP + ECN
        bytes.push((self.dscp << 2) | self.ecn);

        bytes.extend_from_slice(&self.total_length.to_be_bytes());
        bytes.extend_from_slice(&self.identification.to_be_bytes());

        // Flags + Fragment Offset
        let flags_offset =
            ((self.flags as u16) << 13) | (self.fragment_offset as u16);

        bytes.extend_from_slice(&flags_offset.to_be_bytes());

        bytes.push(self.ttl);
        bytes.push(self.next_level_protocol.0);

        bytes.extend_from_slice(&self.checksum.to_be_bytes());

        bytes.extend_from_slice(&self.source.to_bytes());
        bytes.extend_from_slice(&self.destination.to_bytes());

        bytes.extend_from_slice(&self.payload);

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < IPV4_HEADER_LEN {
            return Err("paquet IPv4 trop court".into());
        }

        let version = bytes[0] >> 4;
        let header_length = bytes[0] & 0x0f;

        if version != 4 {
            return Err("version IPv4 invalide".into());
        }

        let header_len = header_length as usize * 4;

        if header_len < IPV4_HEADER_LEN {
            return Err("en-tête IPv4 invalide".into());
        }

        if bytes.len() < header_len {
            return Err("en-tête IPv4 incomplet".into());
        }

        let total_length =
            u16::from_be_bytes([bytes[2], bytes[3]]) as usize;

        if total_length < header_len {
            return Err("longueur totale IPv4 invalide".into());
        }

        if bytes.len() < total_length {
            return Err("paquet IPv4 incomplet".into());
        }

        if header_len != IPV4_HEADER_LEN {
            return Err("options IPv4 non supportées".into());
        }

        let flags_offset = u16::from_be_bytes([bytes[6], bytes[7]]);
        let flags = ((flags_offset >> 13) & 0x07) as u8;
        let fragment_offset = flags_offset & 0x1fff;
        let source = Ipv4Addr::from_bytes(&bytes[12..16])
            .map_err(|_| "adresse source invalide")?;
        let destination = Ipv4Addr::from_bytes(&bytes[16..20])
            .map_err(|_| "adresse destination invalide")?;
        let payload = bytes[header_len..total_length].to_vec();

        Ok(Self {
            version,
            header_length,
            dscp: bytes[1] >> 2,
            ecn: bytes[1] & 0x03,
            total_length: total_length as u16,
            identification: u16::from_be_bytes([bytes[4], bytes[5]]),
            flags,
            fragment_offset,
            ttl: bytes[8],
            next_level_protocol: IpNextHeaderProtocol(bytes[9]),
            checksum: u16::from_be_bytes([bytes[10], bytes[11]]),
            source,
            destination,
            payload,
        })
    }

    pub fn compute_checksum(&self) -> u16 {
        let mut bytes = self.to_bytes();

        bytes[10] = 0;
        bytes[11] = 0;

        checksum(&bytes[..IPV4_HEADER_LEN])
    }

    pub fn recompute_checksum(&mut self) {
        self.checksum = self.compute_checksum();
    }

    pub fn verify_checksum(&self) -> bool {
        self.compute_checksum() == self.checksum
    }
}

fn checksum(bytes: &[u8]) -> u16 {
    let mut sum: u32 = 0;

    for chunk in bytes.chunks(2) {
        let word = if chunk.len() == 2 {
            u16::from_be_bytes([chunk[0], chunk[1]]) as u32
        } else {
            (chunk[0] as u32) << 8
        };

        sum += word;
    }

    while (sum >> 16) != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }

    !(sum as u16)
}


#[allow(non_snake_case, non_upper_case_globals)]
pub mod Ipv4Flags {
    use pnet_macros_support::types::*;

    pub const DontFragment: u3 = 0b010;
    pub const MoreFragments: u3 = 0b001;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IpNextHeaderProtocol(pub u8);

impl IpNextHeaderProtocol {
    pub fn new(value: u8) -> Self {
        IpNextHeaderProtocol(value)
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod IpNextHeaderProtocols {
    use super::IpNextHeaderProtocol;

    pub const Icmp: IpNextHeaderProtocol = IpNextHeaderProtocol(1);
    pub const Tcp: IpNextHeaderProtocol = IpNextHeaderProtocol(6);
    pub const Udp: IpNextHeaderProtocol = IpNextHeaderProtocol(17);
}
