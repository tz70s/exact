//! The sample implemntation of actor
extern crate edge_actor;

use edge_actor::ActorSystem;
use edge_actor::Actor;

/// The entry point is needed, with following type signature.
#[no_mangle]
pub extern fn init<'a>() -> ActorSystem<'a> {
    // Create an actor system
    let mut actor_system = ActorSystem::new("sample-actor-system");

    // Pass the reference functions to construct actor.
    actor_system.create_actor("counter-actor-0", SampleActor::new);
    actor_system.create_actor("counter-actor-1", SampleActor::new);

    // Transfer the ownership out, and pass this to runtime to control.
    actor_system
}

/// Define the sample actor, which store a count.
struct SampleActor {
    count: i32
}

/// Can implement any functions, one strict limitation is,
/// the construction is currently only support for type signature Fn() -> T.
/// That is, there's no passing argument, currently.
impl SampleActor {
    fn new() -> Self {
        SampleActor {
            count: 0
        }
    }
}

/// Should implement the Actor trait for dynamic dispatching in runtime.
/// The core implementation is here.
impl Actor for SampleActor {

    /// The implementation of recv function.
    // TODO: Will find a better way to set become (state machine).
    // TODO: Send! function is still investigated.
    fn recv(&mut self, msg: &str) -> Result<(), &'static str> {
        // You can access the receive message and the internal states which defined.
        println!("The recv msg is : {} and the current count is : {}", msg, self.count);
        self.count += 1;
        // Remember to pass the result back
        Ok(())
    }
}
