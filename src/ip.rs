use crate::{Ipv4Address, Ipv6Address};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub enum IpAddr<IV4: Ipv4Address, IV6: Ipv6Address> {
    V4(IV4),
    V6(IV6),
}

impl<IV4: Ipv4Address, IV6: Ipv6Address> IpAddr<IV4, IV6> {
    pub fn is_documentation(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_documentation(),
            IpAddr::V6(ip) => ip.is_documentation(),
        }
    }

    pub fn is_ipv4(&self) -> bool {
        match self {
            IpAddr::V4(_) => true,
            IpAddr::V6(_) => false,
        }
    }

    pub fn is_ipv6(&self) -> bool {
        match self {
            IpAddr::V4(_) => false,
            IpAddr::V6(_) => true,
        }
    }

    pub fn is_multicast(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_multicast(),
            IpAddr::V6(ip) => ip.is_multicast(),
        }
    }

    pub fn is_global(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_global(),
            IpAddr::V6(ip) => ip.is_global(),
        }
    }

    pub fn is_loopback(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_loopback(),
            IpAddr::V6(ip) => ip.is_loopback(),
        }
    }

    pub fn is_unspecified(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_unspecified(),
            IpAddr::V6(ip) => ip.is_unspecified(),
        }
    }
}

impl<IV4: Ipv4Address, IV6: Ipv6Address> From<[u16; 8]> for IpAddr<IV4, IV6> {
    fn from(segments: [u16; 8]) -> Self {
        let [a, b, c, d, e, f, g, h] = segments;
        IpAddr::V6(IV6::new(a, b, c, d, e, f, g, h))
    }
}

impl<IV4: Ipv4Address, IV6: Ipv6Address> From<[u8; 16]> for IpAddr<IV4, IV6> {
    fn from(octets: [u8; 16]) -> Self {
        let [a0, a1, b0, b1, c0, c1, d0, d1, e0, e1, f0, f1, g0, g1, h0, h1] = octets;
        let a = u16::from_be_bytes([a0, a1]);
        let b = u16::from_be_bytes([b0, b1]);
        let c = u16::from_be_bytes([c0, c1]);
        let d = u16::from_be_bytes([d0, d1]);
        let e = u16::from_be_bytes([e0, e1]);
        let f = u16::from_be_bytes([f0, f1]);
        let g = u16::from_be_bytes([g0, g1]);
        let h = u16::from_be_bytes([h0, h1]);
        IpAddr::V6(IV6::new(a, b, c, d, e, f, g, h))
    }
}

impl<IV4: Ipv4Address, IV6: Ipv6Address> From<[u8; 4]> for IpAddr<IV4, IV6> {
    fn from(octets: [u8; 4]) -> Self {
        let [a, b, c, d] = octets;
        IpAddr::V4(IV4::new(a, b, c, d))
    }
}

impl<IV4: Ipv4Address, IV6: Ipv6Address> From<IV4> for IpAddr<IV4, IV6> {
    fn from(ipv4: IV4) -> Self {
        IpAddr::V4(ipv4)
    }
}

// TODO: impl Display
// TODO: impl IV4
// TODO: impl IV6

