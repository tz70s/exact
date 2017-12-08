// This project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

//! Actor System lib

use std::collections::hash_map::HashMap;
use actor::{ActorContext, ActorContextBehavior};
use std::sync::Arc;

pub struct ActorSystem {
    pub name: String,
    virtual_actors: HashMap<String, Arc<ActorContextBehavior + Send>>,
}

impl ActorSystem {
    pub fn new(name: &str) -> Self {
        ActorSystem {
            name: name.to_owned(),
            virtual_actors: HashMap::new(),
        }
    }

    /// Create an virtual actor into actor system.
    /// Which passes actor context as props.
    pub fn create_virt_actor<T: 'static + Send>(&mut self, actor_context: ActorContext<T>) {
        self.virtual_actors.insert(
            actor_context.actor_type.to_owned(),
            Arc::new(actor_context) as Arc<ActorContextBehavior + Send>,
        );
    }

    /// Get actor ref.
    pub fn get_virt_actor(
        &mut self,
        actor_type: &str,
    ) -> Option<Arc<ActorContextBehavior + Send + 'static>> {
        match self.virtual_actors.get(actor_type) {
            Some(arc) => Some(arc.clone()),
            None => None,
        }
    }
}
