// This project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

//! ## The invoker is the virtual actor execution runtime.
//!
//! The including jobs are:
//!
//! 1. Cluster: manage the clustering nodes' lifecycle
//!
//! 2. Hosts: checkout where virtual actor is located, and where to call up.
//!
//! 3. Activation: responsible for call up virtual actor's behavior.
//!
//! 4. Network: handle network related operations.
//!

extern crate edge_actor;
extern crate futures;
extern crate futures_cpupool;
#[macro_use]
extern crate tokio_core;
extern crate tokio_io;

// Use pub modifier for rust-doc
pub mod hosts;
pub mod activation;
pub mod network;
pub mod cluster;

use hosts::loader;
use hosts::cache;
use network::boss::Boss;
use futures::Future;

fn main() {
    // let mut boss = Boss::new(8080);
    let cache = cache::Cache::new("cache").unwrap();
    let actor_frame = loader::LibraryFrame::new("cache/libsample.dylib").unwrap();
    let mut actor_system = actor_frame.get_actor_system().unwrap();
    let step_machine_actor = actor_system.get_virt_actor("step machine").unwrap();
    let pool = activation::ActivationPool::new(None);
    let res = pool.activate_actor(activation::ActivationFrame {
        msg: "hello".to_owned(),
        boxed_actor: step_machine_actor,
    }).wait()
        .unwrap()
        .unwrap();
    println!("{}", res.msg.unwrap_or("hi".to_string()));
}
