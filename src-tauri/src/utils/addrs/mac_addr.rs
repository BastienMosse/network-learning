use core::str::FromStr;
use core::fmt;

pub const ETHER_ADDR_LEN: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MacAddr([u8; ETHER_ADDR_LEN]);


impl MacAddr {
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self([a, b, c, d, e, f])
    }

    pub fn zero() -> Self {
        Self([0; ETHER_ADDR_LEN])
    }

    pub fn broadcast() -> Self {
        Self([0xff; ETHER_ADDR_LEN])
    }

    pub fn is_zero(&self) -> bool {
        self.0 == [0; ETHER_ADDR_LEN]
    }

    pub fn is_broadcast(&self) -> bool {
        self.0 == [0xff; ETHER_ADDR_LEN]
    }

    pub fn to_bytes(self) -> [u8; ETHER_ADDR_LEN] {
        self.0
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseMacAddrErr> {
        let bytes: [u8; ETHER_ADDR_LEN] = bytes
            .try_into()
            .map_err(|_| ParseMacAddrErr::InvalidLength)?;

        Ok(Self(bytes))
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}


impl FromStr for MacAddr {
    type Err = ParseMacAddrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() != ETHER_ADDR_LEN {
            return Err(ParseMacAddrErr::InvalidLength);
        }

        let mut bytes = [0u8; ETHER_ADDR_LEN];

        for (i, part) in parts.iter().enumerate() {
            bytes[i] = u8::from_str_radix(part, 16)
                .map_err(|_| ParseMacAddrErr::InvalidComponent)?;
        }

        Ok(Self(bytes))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMacAddrErr {
    InvalidLength,
    InvalidComponent,
}

impl fmt::Display for ParseMacAddrErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "invalid MAC address length"),
            Self::InvalidComponent => write!(f, "invalid MAC address component"),
        }
    }
}

impl std::error::Error for ParseMacAddrErr {}
