//! The actor interfaces

/// The default actor trait for implementation.
pub trait Actor {

    /// The actor will be create/get from context to retrieve a reference binding.
    /// Instead of direct using it.

    /// Asynchronous sending, also, fire and forget.
    // TODO: The default sending method for actor
    // TODO: May makes this to be macro as dsl.
    // Parameters will accpet an actor reference.
    fn send(&self) {}

    /// The recv function accept a serializable message.
    /// It can throws out error if there exists.
    // TODO: Customized error type.
    fn recv(&mut self, msg: &str) -> Result<(), &'static str>;

    // TODO: Become and unbecome
    // Implementation tips and plan:
    // The actor instance will keep track the invocation methods.
    fn bec(&self);

}
