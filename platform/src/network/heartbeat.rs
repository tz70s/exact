// This project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

//!
//! Heartbeat module detects liveness of neighbor nodes.
//!

use futures::{Future, Poll};
use std::net::SocketAddr;

use std::io;
use tokio_core::net::UdpSocket;

pub struct HeartBeat {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}
