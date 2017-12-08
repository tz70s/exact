// This project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

//!
//! Boss is responsible for dealing with external and internal network.
//! It's a fancy name which equivalent as reactor.
//!

use tokio_core::net::UdpSocket;
use tokio_core::reactor::{Core, Handle};
use std::net::SocketAddr;

use futures::{Async, Future, Poll};
use std::io;
use std::ops::Add;

pub struct Boss {
    core: Core,
    addr: SocketAddr,
}

struct Router {
    socket: UdpSocket,
    buf: Vec<u8>,
    // Need an event queue to send
    to_send: Option<(usize, SocketAddr)>,
}

impl Router {
    fn new(addr: &SocketAddr, handle: &Handle) -> Self {
        Router {
            socket: UdpSocket::bind(addr, handle)
                .expect("The router address (UdpSocket) can't be bound."),
            buf: vec![0; 1024],
            to_send: None,
        }
    }
}

impl Future for Router {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        // Receive and write into buffer.
        loop {
            // Receive and write into buffer.
            let (amt, addr) = try_nb!(self.socket.recv_from(&mut self.buf));

            let _ = try_nb!(self.socket.send_to(&self.buf[..amt + 2], &addr));
        }
    }
}

impl Boss {
    /// Create a new reactor and sets up the hosting environment.
    pub fn new(port: u32) -> Self {
        let mut core = Core::new().expect("The reactor core initiates failed");
        let sock_addr = "0.0.0.0:"
            .to_string()
            .add(&port.to_string()[..])
            .parse::<SocketAddr>()
            .expect("The socket address can't be parsed");
        Boss {
            core: core,
            addr: sock_addr,
        }
    }

    /// Serve the default receiver
    pub fn serve(&mut self) -> Result<(), io::Error> {
        let handle = self.core.handle();
        self.core.run(Router::new(&self.addr, &handle))
    }
}
