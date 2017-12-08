// This project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

//! This crate is the virtual actor library.
pub mod system;
pub mod actor;

pub use actor::Behavior;
pub use system::ActorSystem;
pub use actor::Error;
