// #![feature(test)]

extern crate libloading;
extern crate edge_actor;
extern crate hello;
// extern crate test;

mod loader;

fn main() {
    let actor_frame = loader::ActorFrame::new("libhello.dylib").unwrap();
    // Stateless function
    actor_frame.hello().unwrap();

    // A stateful object!
    let mut actor_ = actor_frame.get_actor().unwrap();
    actor_.count();
    actor_.count();
}
