// This project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

//! The sample implemntation of actor
#[macro_use]
extern crate edge_actor;

use edge_actor::{ActorSystem, Behavior, Error};
use edge_actor::actor::{ActorContext, Continuation};

/// The entry point is needed, with following type signature.
#[no_mangle]
pub fn init() -> ActorSystem {
    // Create an actor system
    let mut actor_system = ActorSystem::new("sample-actor-system");

    actor_system.create_virt_actor(ActorContext {
        actor_type: "step machine".to_owned(),
        behavior: Box::new(StepInit {}),
        states: InternalStates { count: 0 },
    });

    // Transfer the ownership out, and pass this to runtime to control.
    actor_system
}

struct InternalStates {
    count: i32,
}

struct StepInit {}

/// Define behavior of actor, this is the first step
impl Behavior for StepInit {
    type States = InternalStates;

    fn recv(
        &mut self,
        msg: &str,
        states: &mut InternalStates,
    ) -> Result<Continuation<InternalStates>, Error> {
        println!("The receive msg is : {}", msg);
        states.count += 2;
        println!("The default count will plus 2 => {}", states.count);
        Ok(continuation!( become_ => StepTwo {}))
    }
}

struct StepTwo {}

impl Behavior for StepTwo {
    type States = InternalStates;

    fn recv(
        &mut self,
        msg: &str,
        states: &mut InternalStates,
    ) -> Result<Continuation<InternalStates>, Error> {
        println!("The receive msg is : {}", msg);
        states.count += 1;
        println!("The step two count will plus only 1 => {}", states.count);
        Ok(continuation!( become_ => StepInit {}))
    }
}
