//! Actor System lib

use std::collections::hash_map::HashMap;
use actor::ActorContext;
use actor::ActorContextBehavior;

pub struct ActorSystem {
    pub name: String,
    virtual_actors: HashMap<String, Box<ActorContextBehavior>>,
}

impl ActorSystem {

    pub fn new(name: &str) -> Self {
        ActorSystem {
            name: name.to_owned(),
            virtual_actors: HashMap::new()
        }
    }

    /// Create an virtual actor into actor system.
    /// Which passes actor context as props.
    pub fn create_virt_actor<T: 'static>(&mut self, actor_context: ActorContext<T>) {
        self.virtual_actors.insert(actor_context.actor_type.to_owned(),
                   Box::new(actor_context) as Box<ActorContextBehavior>);
    }

    /// Get actor ref.
    pub fn get_virt_actor(&mut self, actor_type: &str) -> Option<&mut Box<ActorContextBehavior>> {
        self.virtual_actors.get_mut(actor_type)
    }
}
