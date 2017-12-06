// #![feature(test)]

extern crate libloading;
extern crate edge_actor;
extern crate hello;
// extern crate test;

mod loader;

fn main() {
    let actor_frame = loader::LibraryFrame::new("libsample.dylib").unwrap();
    let mut actor_system = actor_frame.get_actor_system().unwrap();
    let actor_ref = actor_system.get_ref("counter-actor-0").unwrap();
    actor_ref.recv("hi");
    actor_ref.recv("hey");
}
