use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use crate::bindings::socket::{IpAddress, IpAddressUnion, IpV4Address, IpV6Address};

impl From<SocketAddr> for IpAddress {
    fn from(socket_addr: SocketAddr) -> Self {
        match socket_addr {
            SocketAddr::V4(socket_addr_v4) => {
                let ipv4_bytes = socket_addr_v4.ip().octets();
                let port = socket_addr_v4.port();

                IpAddress {
                    is_ipv6: false,
                    addr: IpAddressUnion {
                        ipv4: IpV4Address {
                            addr: ipv4_bytes,
                            port,
                        },
                    },
                }
            }
            SocketAddr::V6(socket_addr_v6) => {
                let ipv6_bytes = socket_addr_v6.ip().octets();
                let port = socket_addr_v6.port();

                IpAddress {
                    is_ipv6: true,
                    addr: IpAddressUnion {
                        ipv6: IpV6Address {
                            addr: ipv6_bytes,
                            port,
                        },
                    },
                }
            }
        }
    }
}

// Implementation of From<IpAddress> for SocketAddr
impl From<IpAddress> for SocketAddr {
    fn from(ip_address: IpAddress) -> Self {
        unsafe {
            if ip_address.is_ipv6 {
                let ipv6 = ip_address.addr.ipv6;
                let ip = Ipv6Addr::from(ipv6.addr);
                SocketAddr::V6(SocketAddrV6::new(ip, ipv6.port, 0, 0))
            } else {
                let ipv4 = ip_address.addr.ipv4;
                let ip = Ipv4Addr::from(ipv4.addr);
                SocketAddr::V4(SocketAddrV4::new(ip, ipv4.port))
            }
        }
    }
}
