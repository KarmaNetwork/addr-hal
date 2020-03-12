use crate::{IpAddr, SocketAddressV4, SocketAddressV6};

pub enum SocketAddr<SA4: SocketAddressV4, SA6: SocketAddressV6> {
    V4(SA4),
    V6(SA6),
}

impl<SA4: SocketAddressV4, SA6: SocketAddressV6> SocketAddr<SA4, SA6> {
    pub fn new(ip: IpAddr<SA4::IpAddress, SA6::IpAddress>, port: u16) -> Self {
        match ip {
            IpAddr::V4(a) => SocketAddr::V4(SA4::new(a, port)),
            IpAddr::V6(a) => SocketAddr::V6(SA6::new(a, port, 0, 0)),
        }
    }

    pub fn ip(&self) -> IpAddr<SA4::IpAddress, SA6::IpAddress> {
        match *self {
            SocketAddr::V4(ref a) => IpAddr::V4(*a.ip()),
            SocketAddr::V6(ref a) => IpAddr::V6(*a.ip()),
        }
    }

    pub fn set_ip(&mut self, new_ip: IpAddr<SA4::IpAddress, SA6::IpAddress>) {
        match (self, new_ip) {
            (&mut SocketAddr::V4(ref mut a), IpAddr::V4(new_ip)) => a.set_ip(new_ip),
            (&mut SocketAddr::V6(ref mut a), IpAddr::V6(new_ip)) => a.set_ip(new_ip),
            (self_, new_ip) => *self_ = Self::new(new_ip, self_.port()),
        }
    }

    pub fn port(&self) -> u16 {
        match *self {
            SocketAddr::V4(ref a) => a.port(),
            SocketAddr::V6(ref a) => a.port(),
        }
    }

    pub fn set_port(&mut self, new_port: u16) {
        match *self {
            SocketAddr::V4(ref mut a) => a.set_port(new_port),
            SocketAddr::V6(ref mut a) => a.set_port(new_port),
        }
    }

    pub fn is_ipv4(&self) -> bool {
        match *self {
            SocketAddr::V4(_) => true,
            SocketAddr::V6(_) => false,
        }
    }

    pub fn is_ipv6(&self) -> bool {
        match *self {
            SocketAddr::V4(_) => false,
            SocketAddr::V6(_) => true,
        }
    }
}
