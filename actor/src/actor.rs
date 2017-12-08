// This project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

///! The actor interfaces.

/// Mask the error type.
pub type Error = ::std::io::Error;

#[macro_export]
macro_rules! continuation {
    (become_ => $become:expr, envelope => $envelope:expr) => {
        Continuation {
            become_: Some(Box::new($become)),
            envelope: Some($envelope)
        }
    };
    (become_ => $become:expr) => {
        Continuation {
            become_: Some(Box::new($become)),
            envelope: None,
        }
    };
    (envelope => $envelope:expr) => {
        Continuation {
            become_: None,
            envelope: $envelope,
        }
    };
    () => {
        Continuation {
            become_: None,
            envelope: None,
        }
    }
}

/// The send back structure as record frame.
/// Each fields can be ignored, but the return Send need to be returned.
pub struct Continuation<T> {
    pub become_: Option<Box<Behavior<States = T> + Send>>,
    pub envelope: Option<Envelope>,
}

pub struct Envelope {
    pub target_type: String,
    pub msg: Option<String>,
}

/// The behavior for implementation.
/// The state machine pattern.
pub trait Behavior {
    type States;

    /// The recv function accept a serializable message.
    /// It can throws out error if there exists.
    fn recv(
        &mut self,
        msg: &str,
        states: &mut Self::States,
    ) -> Result<Continuation<Self::States>, Error>;
}

/// Actor records the virtual actor's context and associated behavior.
pub struct ActorContext<T: Send> {
    pub actor_type: String,
    pub behavior: Box<Behavior<States = T> + Send>,
    pub states: T,
}

impl<T: Send> ActorContext<T> {
    /// Create virtual actor's context, with type aliases and initial behavior.
    pub fn new(actor_type: &str, behavior: Box<Behavior<States = T> + Send>, states: T) -> Self {
        ActorContext {
            actor_type: actor_type.to_owned(),
            behavior,
            states,
        }
    }

    /// The become_ method is an awesome design in actor model, composed like state machine patterns.
    /// But how to use this? Once recv is waked, and return value will contains Send
    pub fn become_(&mut self, behavior: Option<Box<Behavior<States = T> + Send>>) {
        if let Some(b) = behavior {
            self.behavior = b;
        }
    }
}

pub trait ActorContextBehavior {
    fn recv(&mut self, msg: &str) -> Result<Option<Envelope>, Error>;
}

impl<T: Send> ActorContextBehavior for ActorContext<T> {
    fn recv(&mut self, msg: &str) -> Result<Option<Envelope>, Error> {
        let cps = self.behavior.recv(msg, &mut self.states)?;
        let (become_, envelope) = (cps.become_, cps.envelope);
        self.become_(become_);
        Ok(envelope)
    }
}
