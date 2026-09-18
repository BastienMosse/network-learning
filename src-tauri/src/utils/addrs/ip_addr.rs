use core::fmt;
use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddr {
    pub fn is_ipv4(&self) -> bool {
        matches!(self, Self::V4(_))
    }

    pub fn is_ipv6(&self) -> bool {
        matches!(self, Self::V6(_))
    }
}

// --------------------------------------------------
// IPv4
// --------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv4Addr([u8; 4]);


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseIpv4Error;

impl Ipv4Addr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self([a, b, c, d])
    }

    pub const fn to_bytes(self) -> [u8; 4] {
        self.0
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseIpv4Error> {
        Ok(Self(
            bytes.try_into().map_err(|_| ParseIpv4Error)?
        ))
    }

    pub fn to_u32(self) -> u32 {
        u32::from_be_bytes(self.0)
    }
}

impl From<Ipv4Addr> for u32 {
    fn from(addr: Ipv4Addr) -> u32 {
        addr.to_u32()
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}


impl FromStr for Ipv4Addr {
    type Err = ParseIpv4Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u8> = s
            .split('.')
            .map(|p| p.parse().map_err(|_| ParseIpv4Error))
            .collect::<Result<_, _>>()?;

        Ok(Self(
            parts.try_into().map_err(|_| ParseIpv4Error)?
        ))
    }
}


// --------------------------------------------------
// IPv6
// --------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv6Addr([u8; 16]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseIpv6Error;


impl Ipv6Addr {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        Self([
            (a >> 8) as u8, a as u8,
            (b >> 8) as u8, b as u8,
            (c >> 8) as u8, c as u8,
            (d >> 8) as u8, d as u8,
            (e >> 8) as u8, e as u8,
            (f >> 8) as u8, f as u8,
            (g >> 8) as u8, g as u8,
            (h >> 8) as u8, h as u8,
        ])
    }

    pub const fn to_bytes(self) -> [u8; 16] {
        self.0
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseIpv6Error> {
        Ok(Self(
            bytes.try_into().map_err(|_| ParseIpv6Error)?
        ))
    }
}

impl fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..8 {
            if i != 0 {
                write!(f, ":")?;
            }

            let group = u16::from_be_bytes([
                self.0[i * 2],
                self.0[i * 2 + 1],
            ]);

            write!(f, "{group:x}")?;
        }

        Ok(())
    }
}




impl FromStr for Ipv6Addr {
    type Err = ParseIpv6Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = [0u8; 16];

        if let Some((head, tail)) = s.split_once("::") {
            if tail.contains("::") {
                return Err(ParseIpv6Error);
            }

            let head: Vec<u16> = parse_groups(head)?;
            let tail: Vec<u16> = parse_groups(tail)?;

            if head.len() + tail.len() > 8 {
                return Err(ParseIpv6Error);
            }

            for (i, group) in head.iter().enumerate() {
                bytes[i * 2..i * 2 + 2]
                    .copy_from_slice(&group.to_be_bytes());
            }

            let start = 8 - tail.len();

            for (i, group) in tail.iter().enumerate() {
                bytes[(start + i) * 2..(start + i) * 2 + 2]
                    .copy_from_slice(&group.to_be_bytes());
            }
        } else {
            let groups = parse_groups(s)?;

            if groups.len() != 8 {
                return Err(ParseIpv6Error);
            }

            for (i, group) in groups.iter().enumerate() {
                bytes[i * 2..i * 2 + 2]
                    .copy_from_slice(&group.to_be_bytes());
            }
        }

        Ok(Self(bytes))
    }
}

fn parse_groups(s: &str) -> Result<Vec<u16>, ParseIpv6Error> {
    if s.is_empty() {
        return Ok(Vec::new());
    }

    s.split(':')
        .map(|x| u16::from_str_radix(x, 16).map_err(|_| ParseIpv6Error))
        .collect()
}