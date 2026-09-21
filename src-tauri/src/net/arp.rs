use crate::utils::addrs::{mac_addr::MacAddr, ip_addr::Ipv4Addr};
use super::ethernet::EtherType;

pub const ARP_MESSAGE_LEN: usize = 28;

pub struct Arp {
    pub hardware_type: ArpHardwareType,
    pub protocol_type: EtherType,
    pub hw_addr_len: u8,
    pub proto_addr_len: u8,
    pub operation: ArpOperation,
    pub sender_mac: MacAddr,
    pub sender_ip: Ipv4Addr,
    pub target_mac: MacAddr,
    pub target_ip: Ipv4Addr,
}

impl Arp {
    pub const ETHERNET_HW_ADDR_LEN: u8 = 6;
    pub const IPV4_PROTO_ADDR_LEN: u8 = 4;

    pub fn new(
        operation: ArpOperation,
        sender_mac: MacAddr,
        sender_ip: Ipv4Addr,
        target_mac: MacAddr,
        target_ip: Ipv4Addr,
    ) -> Self {
        Self {
            hardware_type: ArpHardwareTypes::ETHERNET,
            protocol_type: super::ethernet::EtherTypes::Ipv4,
            hw_addr_len: Self::ETHERNET_HW_ADDR_LEN,
            proto_addr_len: Self::IPV4_PROTO_ADDR_LEN,
            operation,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(ARP_MESSAGE_LEN);
        b.extend_from_slice(&self.hardware_type.0.to_be_bytes());
        b.extend_from_slice(&self.protocol_type.to_bytes());
        b.push(self.hw_addr_len);
        b.push(self.proto_addr_len);
        b.extend_from_slice(&self.operation.0.to_be_bytes());
        b.extend_from_slice(&self.sender_mac.to_bytes());
        b.extend_from_slice(&self.sender_ip.to_bytes());
        b.extend_from_slice(&self.target_mac.to_bytes());
        b.extend_from_slice(&self.target_ip.to_bytes());
        b
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < ARP_MESSAGE_LEN {
            return Err("paquet ARP trop court".into());
        }

        let hardware_type =
            u16::from_be_bytes([bytes[0], bytes[1]]);

        let protocol_type =
            u16::from_be_bytes([bytes[2], bytes[3]]);

        let hw_addr_len = bytes[4];
        let proto_addr_len = bytes[5];

        if hw_addr_len != Self::ETHERNET_HW_ADDR_LEN {
            return Err("longueur d'adresse matérielle invalide".into());
        }

        if proto_addr_len != Self::IPV4_PROTO_ADDR_LEN {
            return Err("longueur d'adresse protocole invalide".into());
        }

        let operation =
            u16::from_be_bytes([bytes[6], bytes[7]]);

        let sender_mac =
            MacAddr::from_bytes(&bytes[8..14])
                .map_err(|_| "adresse MAC source invalide")?;

        let sender_ip =
            Ipv4Addr::from_bytes(&bytes[14..18])
                .map_err(|_| "adresse IPv4 source invalide")?;

        let target_mac =
            MacAddr::from_bytes(&bytes[18..24])
                .map_err(|_| "adresse MAC destination invalide")?;

        let target_ip =
            Ipv4Addr::from_bytes(&bytes[24..28])
                .map_err(|_| "adresse IPv4 destination invalide")?;

        Ok(Self {
            hardware_type: ArpHardwareType(hardware_type),
            protocol_type: EtherType::new(protocol_type),
            hw_addr_len,
            proto_addr_len,
            operation: ArpOperation(operation),
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        })
    }
}



/// Represents the ARP operation type.

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ArpHardwareType(pub u16);

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ArpHardwareTypes {
    use super::ArpHardwareType;

    pub const ETHERNET: ArpHardwareType = ArpHardwareType(1);
}



/// The ARP protocol operations.

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ArpOperation(pub u16);

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ArpOperations {
    use super::ArpOperation;

    pub const Request: ArpOperation = ArpOperation(1);
    pub const Reply: ArpOperation = ArpOperation(2);
}
