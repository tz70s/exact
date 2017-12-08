// This project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

//! Node

use std::net::SocketAddr;

pub enum NodeState {
    Running,
    Pending,
    Failed,
}

pub struct Predicate<'a> {
    pub alias: &'a str,
    pub sock_addr: SocketAddr,
    pub state: NodeState,
}

pub struct Observer<'a> {
    pub predicates: Vec<Predicate<'a>>,
}

impl<'a> Observer<'a> {
    fn new() -> Self {
        Observer {
            predicates: Vec::new(),
        }
    }

    fn get_state(&mut self, alias: &'a str) -> Option<&NodeState> {
        match self.predicates.iter().find(|p| p.alias == alias) {
            Some(p) => Some(&p.state),
            None => None,
        }
    }

    fn insert_predicate(&mut self, predicate: Predicate<'a>) {
        // First, match if the predicate existed or not.
        let create = match self.predicates
            .iter_mut()
            .find(|p| p.alias == predicate.alias)
        {
            Some(p) => {
                p.sock_addr = predicate.sock_addr;
                p.state = predicate.state;
                None
            }
            None => Some(predicate),
        };
        // To avoid double mutable reference, split out here.
        if let Some(predicate) = create {
            self.predicates.push(predicate);
        }
    }

    fn remove_predicate(&mut self, alias: &'a str) -> Option<Predicate<'a>> {
        match self.predicates.iter().position(|p| p.alias == alias) {
            Some(p) => Some(self.predicates.remove(p)),
            None => None,
        }
    }
}

/// Information of self node.
pub struct Me<'a> {
    pub state: NodeState,
    pub expect_observers: usize,
    pub observers: Vec<&'a str>,
}

impl<'a> Me<'a> {
    pub fn new(expect_observers: usize) -> Self {
        Me {
            state: NodeState::Running,
            expect_observers,
            observers: Vec::with_capacity(expect_observers),
        }
    }

    pub fn add_observer(&mut self, ip_address: &'a str) {
        self.observers.push(ip_address)
    }
}
