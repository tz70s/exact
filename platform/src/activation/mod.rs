// This project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

//! Activation executes the actor frame

use futures_cpupool::{CpuFuture, CpuPool};
use futures::Future;
use std::collections::VecDeque;
use std::sync::Arc;
use edge_actor::actor::{ActorContextBehavior, Envelope, Error};

/// Activation pool wraps cpu pool.
pub struct ActivationPool {
    cpu_pool: CpuPool,
    // predicates: VecDeque
}

impl ActivationPool {
    /// The default sizes of pool will be followed in num of cpus.
    pub fn new(size: Option<usize>) -> Self {
        match size {
            Some(s) => ActivationPool {
                cpu_pool: CpuPool::new(s),
            },
            None => ActivationPool {
                cpu_pool: CpuPool::new_num_cpus(),
            },
        }
    }

    /// Methods for invoking actor, will receive an execution frame and activate after parsing.
    /// The execution will wraps the invoking methods in and return a future.
    pub fn activate_actor(
        &self,
        activation_frame: ActivationFrame,
    ) -> CpuFuture<Option<Envelope>, Error> {
        self.cpu_pool.spawn_fn(move || {
            let ret = activation_frame
                .boxed_actor
                .clone()
                .recv(&activation_frame.msg[..]);
            ret
        })
    }
}

pub struct ActivationFrame {
    pub msg: String,
    pub boxed_actor: Arc<ActorContextBehavior + Send + 'static + Sync>,
}
