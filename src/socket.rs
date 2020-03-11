use crate::{IpAddr, Ipv4Address, Ipv6Address, SocketAddressV4, SocketAddressV6};

pub enum SocketAddr<SA4: SocketAddressV4 + Clone, SA6: SocketAddressV6 + Clone> {
    V4(SA4),
    V6(SA6),
}

impl<SA4: SocketAddressV4 + Clone, SA6: SocketAddressV6 + Clone> SocketAddr<SA4, SA6> {
    pub fn ip(&self) -> IpAddr<SA4::IpAddress, SA6::IpAddress> {
        match *self {
            SocketAddr::V4(ref a) => IpAddr::V4(*a.ip()),
            SocketAddr::V6(ref a) => IpAddr::V6(*a.ip()),
        }
    }
}
