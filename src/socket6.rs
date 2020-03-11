use crate::Ipv6Address;

pub trait SocketAddressV6 {
    type IpAddress: Ipv6Address;

    fn new(ip: Self::IpAddress, port: u16, flowinfo: u32, scope_id: u32) -> Self;

    fn ip(&self) -> &Self::IpAddress;

    fn set_ip(&mut self, ip: Self::IpAddress);

    fn port(&self) -> u16;

    fn set_port(&mut self, port: u16);

    fn set_flowinfo(&mut self, new_flowinfo: u32);

    fn flowinfo(&self) -> u32;

    fn set_scope_id(&mut self, new_scope_id: u32);

    fn scope_id(&self) -> u32;
}

#[cfg(feature = "impl-type")]
mod impl_type {
    use super::SocketAddressV6;
    use crate::Ipv6Address;

    struct SocketAddrV6Inner<I: Ipv6Address> {
        pub addr: I,
        pub port: u16,
        pub flowinfo: u32,
        pub scope_id: u32,
    }

    pub struct SocketAddrV6<I: Ipv6Address> {
        inner: SocketAddrV6Inner<I>,
    }

    impl<I: Ipv6Address> SocketAddressV6 for SocketAddrV6<I> {
        type IpAddress = I;

        fn new(ip: Self::IpAddress, port: u16, flowinfo: u32, scope_id: u32) -> Self {
            SocketAddrV6 {
                inner: SocketAddrV6Inner {
                    addr: ip,
                    port,
                    flowinfo,
                    scope_id,
                },
            }
        }

        fn ip(&self) -> &Self::IpAddress {
            &self.inner.addr
        }

        fn set_ip(&mut self, ip: Self::IpAddress) {
            self.inner.addr = ip;
        }

        fn port(&self) -> u16 {
            self.inner.port
        }

        fn set_port(&mut self, port: u16) {
            self.inner.port = port;
        }

        fn set_flowinfo(&mut self, new_flowinfo: u32) {
            self.inner.flowinfo = new_flowinfo;
        }

        fn flowinfo(&self) -> u32 {
            self.inner.flowinfo
        }

        fn set_scope_id(&mut self, new_scope_id: u32) {
            self.inner.scope_id = new_scope_id;
        }

        fn scope_id(&self) -> u32 {
            self.inner.scope_id
        }
    }
}

#[cfg(feature = "impl-type")]
pub use impl_type::SocketAddrV6;
