#![no_std]

mod ipv4;
pub use ipv4::Ipv4Address;

mod ipv6;
pub use ipv6::Ipv6Address;
pub use ipv6::Ipv6MulticastScope;

mod ip;
pub use ip::IpAddr;

/* mod socket4; */
// pub use socket4::SocketAddressV4;
//
// mod socket6;
// pub use socket6::SocketAddressV6;
//
// mod socket;
// pub use socket::SocketAddr;
/*  */
