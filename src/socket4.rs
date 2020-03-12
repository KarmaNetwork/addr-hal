use crate::Ipv4Address;

pub trait SocketAddressV4 {
    type IpAddress: Ipv4Address + Copy;

    fn new(ip: Self::IpAddress, port: u16) -> Self;

    fn ip(&self) -> &Self::IpAddress;

    fn set_ip(&mut self, ip: Self::IpAddress);

    fn port(&self) -> u16;

    fn set_port(&mut self, port: u16);
}

#[cfg(feature = "impl-type")]
mod impl_type {
    use super::SocketAddressV4;
    use crate::Ipv4Address;

    struct SocketAddrV4Inner<I: Ipv4Address> {
        pub addr: I,
        pub port: u16,
    }

    pub struct SocketAddrV4<I: Ipv4Address> {
        inner: SocketAddrV4Inner<I>,
    }

    impl<I: Ipv4Address> SocketAddressV4 for SocketAddrV4<I> {
        type IpAddress = I;

        fn new(ip: Self::IpAddress, port: u16) -> Self {
            SocketAddrV4 {
                inner: SocketAddrV4Inner { addr: ip, port },
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
    }
}

#[cfg(feature = "impl-type")]
pub use impl_type::SocketAddrV4;
