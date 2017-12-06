//! Actor System lib

use std::collections::hash_map::HashMap;
use ::actor::Actor;

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
    /// The most important thing is that the reference of actor will be immutable.
    /// **Currently, passing a construction lambda and returning the object.**
    pub fn create_actor<T: Actor + 'a, F>(&mut self, name: &str, f: F) -> Result<(), &'static str>
        where F: Fn() -> T {
        self.actors.insert(name.to_owned(), Box::new(f()) as Box<Actor + 'a>).unwrap();
        Ok(())
    }

    /// Retrieve actor ref.
    /// The most important thing is that the reference of actor will be immutable.
    pub fn get_ref(&self, name: &str) -> Option<&Box<Actor + 'a>> {
        self.actors.get(name)
    }

    pub fn get_and_remove(&mut self, name: &str) -> Option<Box<Actor + 'a>> {
        self.actors.remove(name)
    }

    // TODO: Safer way.
    pub fn destroy(&mut self) {
        self.actors.clear();
    }
}

#[cfg(test)]
mod actor_system_test {
    use super::*;

    fn create_actor_system() -> ActorSystem {
        ActorSystem::new("foo")
    }

}