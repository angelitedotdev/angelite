use crate::raw;
use std::{
    net::{Ipv6Addr, SocketAddr, ToSocketAddrs},
    ptr,
};

use crate::bindings::io;
use crate::bindings::socket as ffi;

use crate::ffi::CheckOperation;

pub struct Socket(*mut ffi::Socket);
pub struct Connection(*mut ffi::Socket);
pub struct Listener(*mut ffi::Socket);

macro_rules! socket_create {
    ($name:ident, $out:tt, $ty:expr) => {
        impl $out {
            fn $name(addrs: impl ToSocketAddrs) -> Result<$out, ()> {
                for addr in addrs.to_socket_addrs().map_err(|_| ())? {
                    let ipv6 = matches!(addr, SocketAddr::V6(_));
                    let socket = unsafe {
                        ffi::create(ipv6, $ty, ptr::null_mut()).expect("failed to allocate socket")
                    };

                    match unsafe { ffi::bind(socket, &addr.into()).check_operation() } {
                        Ok(_)
                            if let Ok(_) =
                                unsafe { ffi::listen(socket, 1000).check_operation() } =>
                        {
                            return Ok($out(socket));
                        }
                        Err(_) => {
                            unsafe { ffi::release(socket) };
                            continue;
                        }
                        _ => continue,
                    }
                }

                return Err(());
            }
        }
    };
}

socket_create!(bind, Socket, io::SockType::Dgram);
socket_create!(listen, Listener, io::SockType::Stream);
socket_create!(connect, Connection, io::SockType::Stream);

raw!(Socket, *mut ffi::Socket);
raw!(Listener, *mut ffi::Socket);
raw!(Connection, *mut ffi::Socket);

impl Listener {
    fn accept(&self) -> Accept<'a> {
        let operation_id = OperationId(0);
        unsafe { ffi::accept(self.0, &mut operation_id as *mut _ as *mut _) }
        future::Accept(operation_id, self.0)
    }
}

mod future {
    use crate::io::OperationId;

    pub struct Accept(OperationId, *mut ffi::Socket);

    impl Future for Accept<'_> {
        type Output = *mut ffi::Socket;
        fn poll(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
        }
    }
}
