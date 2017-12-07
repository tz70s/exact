//! This crate is the virtual actor library for iot edge environment.
pub mod system;
pub mod actor;

pub use actor::Behavior;
pub use system::ActorSystem;
