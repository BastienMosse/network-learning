use std::fmt;

pub const ICMP_HEADER_LEN: usize = 4;

// Types
pub const ECHO_REPLY: u8 = 0;
pub const DESTINATION_UNREACHABLE: u8 = 3;
pub const ECHO_REQUEST: u8 = 8;
pub const TIME_EXCEEDED: u8 = 11;

// Destination Unreachable codes
pub const NETWORK_UNREACHABLE: u8 = 0;
pub const HOST_UNREACHABLE: u8 = 1;
pub const FRAGMENTATION_NEEDED: u8 = 4;

// Time Exceeded codes
pub const TTL_EXCEEDED: u8 = 0;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Icmp {
    pub icmp_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub payload: Vec<u8>,
}


impl Icmp {
    pub fn new(
        icmp_type: u8,
        code: u8,
        payload: Vec<u8>,
    ) -> Self {
        let mut packet = Self {
            icmp_type,
            code,
            checksum: 0,
            payload,
        };

        packet.recompute_checksum();

        packet
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes =
            Vec::with_capacity(
                ICMP_HEADER_LEN + self.payload.len()
            );

        bytes.push(self.icmp_type);
        bytes.push(self.code);
        bytes.extend_from_slice(&self.checksum.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes
    }

    pub fn from_bytes(
        bytes: &[u8],
    ) -> Result<Self, ParseIcmpError> {

        if bytes.len() < ICMP_HEADER_LEN {
            return Err(ParseIcmpError);
        }

        let icmp_type = bytes[0];
        let code = bytes[1];
        let checksum = u16::from_be_bytes([bytes[2], bytes[3]]);
        let payload = bytes[ICMP_HEADER_LEN..].to_vec();

        Ok(Self {
            icmp_type,
            code,
            checksum,
            payload,
        })
    }


    pub fn compute_checksum(&self) -> u16 {
        let mut bytes =
            Vec::with_capacity(
                ICMP_HEADER_LEN + self.payload.len()
            );

        bytes.push(self.icmp_type);
        bytes.push(self.code);
        bytes.push(0);
        bytes.push(0);
        bytes.extend_from_slice(&self.payload);

        checksum(&bytes)
    }


    pub fn recompute_checksum(&mut self) {
        self.checksum = self.compute_checksum();
    }


    pub fn verify_checksum(&self) -> bool {
        self.compute_checksum() == self.checksum
    }


    // ========================================================
    // Echo
    // ========================================================

    pub fn is_echo_request(&self) -> bool {
        self.icmp_type == ECHO_REQUEST && self.code == 0
    }

    pub fn is_echo_reply(&self) -> bool {
        self.icmp_type == ECHO_REPLY && self.code == 0
    }

    // ========================================================
    // Destination Unreachable
    // ========================================================
    
    pub fn is_destination_unreachable(&self) -> bool {
        self.icmp_type == DESTINATION_UNREACHABLE
    }

    pub fn is_network_unreachable(&self) -> bool {
        self.icmp_type == DESTINATION_UNREACHABLE && self.code == NETWORK_UNREACHABLE
    }

    pub fn is_host_unreachable(&self) -> bool {
        self.icmp_type == DESTINATION_UNREACHABLE && self.code == HOST_UNREACHABLE
    }

    pub fn is_fragmentation_needed(&self) -> bool {
        self.icmp_type == DESTINATION_UNREACHABLE && self.code == FRAGMENTATION_NEEDED
    }

    // ========================================================
    // Time Exceeded
    // ========================================================

    pub fn is_time_exceeded(&self) -> bool {
        self.icmp_type == TIME_EXCEEDED
    }

    pub fn is_ttl_exceeded(&self) -> bool {
        self.icmp_type == TIME_EXCEEDED && self.code == TTL_EXCEEDED
    }

}


fn checksum(bytes: &[u8]) -> u16 {
    let mut sum: u32 = 0;

    let mut chunks = bytes.chunks_exact(2);

    for chunk in &mut chunks {
        let word = u16::from_be_bytes([
            chunk[0],
            chunk[1],
        ]);

        sum += word as u32;
    }

    if let Some(&byte) = chunks.remainder().first() {
        sum += (byte as u32) << 8;
    }

    while (sum >> 16) != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }

    !(sum as u16)
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseIcmpError;

impl fmt::Display for ParseIcmpError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "paquet ICMP invalide")
    }
}

impl std::error::Error for ParseIcmpError {}