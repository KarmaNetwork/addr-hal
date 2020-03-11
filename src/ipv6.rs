// use crate::{AsInner, FromInner};

pub trait Ipv6Address {
    // pub trait Ipv6Address: AsInner + FromInner {
    fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self;

    fn segments(&self) -> [u16; 8];

    fn octets(&self) -> [u8; 16] {
        let segments = self.segments();

        let a = segments[0];
        let b = segments[1];
        let c = segments[2];
        let d = segments[3];
        let e = segments[4];
        let f = segments[5];
        let g = segments[6];
        let h = segments[7];

        [
            a.to_be_bytes()[0],
            a.to_be_bytes()[1],
            b.to_be_bytes()[0],
            b.to_be_bytes()[1],
            c.to_be_bytes()[0],
            c.to_be_bytes()[1],
            d.to_be_bytes()[0],
            d.to_be_bytes()[1],
            e.to_be_bytes()[0],
            e.to_be_bytes()[1],
            f.to_be_bytes()[0],
            f.to_be_bytes()[1],
            g.to_be_bytes()[0],
            g.to_be_bytes()[1],
            h.to_be_bytes()[0],
            h.to_be_bytes()[1],
        ]
    }

    fn is_documentation(&self) -> bool {
        (self.segments()[0] == 0x2001) && (self.segments()[1] == 0xdb8)
    }

    fn is_global(&self) -> bool {
        match self.multicast_scope() {
            Some(Ipv6MulticastScope::Global) => true,
            None => self.is_unicast_global(),
            _ => false,
        }
    }

    fn is_unspecified(&self) -> bool {
        self.segments() == [0, 0, 0, 0, 0, 0, 0, 0]
    }

    fn is_loopback(&self) -> bool {
        self.segments() == [0, 0, 0, 0, 0, 0, 0, 1]
    }

    fn is_multicast(&self) -> bool {
        (self.segments()[0] & 0xff00) == 0xff00
    }

    fn is_unicast_link_local(&self) -> bool {
        (self.segments()[0] & 0xffc0) == 0xfe80
    }

    fn is_unicast_link_local_strict(&self) -> bool {
        (self.segments()[0] & 0xffff) == 0xfe80
            && (self.segments()[1] & 0xffff) == 0
            && (self.segments()[2] & 0xffff) == 0
            && (self.segments()[3] & 0xffff) == 0
    }

    fn is_unicast_site_local(&self) -> bool {
        (self.segments()[0] & 0xffc0) == 0xfec0
    }

    fn is_unique_local(&self) -> bool {
        (self.segments()[0] & 0xfe00) == 0xfc00
    }

    fn is_unicast_global(&self) -> bool {
        !self.is_multicast()
            && !self.is_loopback()
            && !self.is_unicast_link_local()
            && !self.is_unique_local()
            && !self.is_unspecified()
            && !self.is_documentation()
    }

    fn multicast_scope(&self) -> Option<Ipv6MulticastScope> {
        if self.is_multicast() {
            match self.segments()[0] & 0x000f {
                1 => Some(Ipv6MulticastScope::InterfaceLocal),
                2 => Some(Ipv6MulticastScope::LinkLocal),
                3 => Some(Ipv6MulticastScope::RealmLocal),
                4 => Some(Ipv6MulticastScope::AdminLocal),
                5 => Some(Ipv6MulticastScope::SiteLocal),
                8 => Some(Ipv6MulticastScope::OrganizationLocal),
                14 => Some(Ipv6MulticastScope::Global),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Copy, PartialEq, Eq, Clone, Hash, Debug)]
pub enum Ipv6MulticastScope {
    InterfaceLocal,
    LinkLocal,
    RealmLocal,
    AdminLocal,
    SiteLocal,
    OrganizationLocal,
    Global,
}

#[cfg(feature = "impl-type")]
pub struct Ipv6Addr {
    inner: [u16; 8],
}

#[cfg(feature = "impl-type")]
impl Ipv6Address for Ipv6Addr {
    fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        Ipv6Addr {
            inner: [a, b, c, d, e, f, g, h],
        }
    }

    fn segments(&self) -> [u16; 8] {
        self.inner.clone()
    }
}

/* #[cfg(feature = "impl-type")] */
// impl AsInner for Ipv6Addr {
//     type Inner = [u16; 8];
//
//     fn as_inner(&self) -> &Self::Inner {
//         &self.inner
//     }
// }
//
// #[cfg(feature = "impl-type")]
// impl FromInner for Ipv6Addr {
//     type Inner = [u16; 8];
//     fn from_inner(inner: Self::Inner) -> Self {
//         Ipv6Addr { inner }
//     }
/* } */
