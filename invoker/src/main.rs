//! Runtime entry point!

extern crate libloading;
extern crate edge_actor;
extern crate hello;

use std::any::Any;

mod loader;

fn main() {
    let actor_frame = loader::LibraryFrame::new("libsample.dylib").unwrap();
    let mut actor_system = actor_frame.get_actor_system().unwrap();
    let mut step_machine_actor = actor_system.get_virt_actor("step machine").unwrap();
    step_machine_actor.recv("hello");
    step_machine_actor.recv("hi");
    step_machine_actor.recv("hello");
}
