//! Actor System lib

use std::collections::hash_map::HashMap;
use ::Actor;

pub struct ActorSystem<'a> {
    pub name: String,
    // Store the pointer of actor.
    actors: HashMap<String, Box<Actor + 'a>>,
}

impl<'a> ActorSystem<'a> {

    /// Create a new actor system.
    pub fn new(name: &str) -> Self {
        ActorSystem {
            name: name.to_owned(),
            actors: HashMap::new(),
        }
    }

    /// Create an actor in the actor system.
    /// Will return an actor reference by default.
    /// The lifetime of actor will be enclosed in the actor context.
    /// **Currently, passing a construction lambda and returning the object.**
    pub fn create_actor<T: Actor + 'a, F>(&mut self, name: &str, f: F) -> Result<&mut Box<Actor + 'a>, &'static str>
        where F: Fn() -> T {
        self.actors.insert(name.to_owned(), Box::new(f()) as Box<Actor + 'a>);
        let bind = self.actors.get_mut(name).unwrap();
        Ok(bind)
    }

    /// Retrieve actor ref.
    /// The most important thing is that the reference of actor will be immutable.
    pub fn get_ref(&mut self, name: &str) -> Option<&mut Box<Actor + 'a>> {
        self.actors.get_mut(name)
    }

    pub fn get_and_remove(&mut self, name: &str) -> Option<Box<Actor + 'a>> {
        self.actors.remove(name)
    }

    // TODO: Safer way.
    pub fn destroy(&mut self) {
        self.actors.clear();
    }

    // TODO: This will be routed to a handler for handling routing message.
    pub fn route_to(&self, target: String, msg: String) -> (String, String) {
        (target, msg)
    }
}

#[cfg(test)]
mod actor_system_test {
    use super::*;

    struct TmpActor {
        count: i32
    }

    impl TmpActor {
        fn new() -> Self {
            TmpActor {
                count: 0
            }
        }
    }

    impl Actor for TmpActor {
        fn recv(&mut self, msg: &str) -> Result<(), &'static str> {
            self.count += 1;
            println!("Count : {}", self.count);
            Ok(())
        }
    }

    fn create_actor_system<'a>(name: &str) -> ActorSystem<'a> {
        ActorSystem::new(name)
    }

    #[test]
    fn test_create() {
        let actor_system = create_actor_system("foo");
    }

    #[test]
    fn test_create_actor() {
        let mut actor_system = create_actor_system("foo");
        let mut bind = actor_system.create_actor("bar", TmpActor::new).unwrap();
        bind.recv("hey");
        bind.recv("hi");

    }
}