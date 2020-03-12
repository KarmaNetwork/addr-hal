use crate::{Ipv4Address, Ipv6Address};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub enum IpAddr<IV4: Ipv4Address<IV6>, IV6: Ipv6Address<IV4>> {
    V4(IV4),
    V6(IV6),
}

impl<IV4: Ipv4Address<IV6>, IV6: Ipv6Address<IV4>> IpAddr<IV4, IV6> {
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

/* impl<IV4: Ipv4Address<IV6>, IV6: Ipv6Address<IV4>> From<[u16; 8]> for IpAddr<IV4, IV6> { */
// fn from(segments: [u16; 8]) -> Self {
//     IpAddr::V6(IV6::from(segments))
// }
/* } */

/* impl<IV4: Ipv4Address<IV6>, IV6: Ipv6Address<IV4>> From<[u8; 16]> for IpAddr<IV4, IV6> { */
//     fn from(octets: [u8; 16]) -> Self {
//         IpAddr::V6(IV6::from(octets))
//     }
// }
//
// impl<IV4: Ipv4Address<IV6>, IV6: Ipv6Address<IV4>> From<[u8; 4]> for IpAddr<IV4, IV6> {
//     fn from(octets: [u8; 4]) -> Self {
//         IpAddr::V4(IV4::from(octets))
//     }
/* } */

/* impl<IV4: Ipv4Address<IV6>, IV6: Ipv6Address<IV4>> From<IV6> for IpAddr<IV4, IV6> { */
//     fn from(addr: IV6) -> Self {
//         IpAddr::V6(addr)
//     }
// }
//
// impl<IV4: Ipv4Address<IV6>, IV6: Ipv6Address<IV4>> From<IV4> for IpAddr<IV4, IV6> {
//     fn from(addr: IV4) -> Self {
//         IpAddr::V4(addr)
//     }
/* } */
