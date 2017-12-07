//! Internal implementation of wrapping dynamic lib loading.
//! The target loading library is the implementation from user's application logics.

use libloading;
use edge_actor::ActorSystem;
use std::io;

/// The record entity of implementation from user, also, a meta information of dynamic lib.
pub struct LibraryFrame<'a> {
    lib_name: &'a str,
    dyn_lib: libloading::Library,
}

impl<'a> LibraryFrame<'a> {

    /// Create a new actor lib frame.
    pub fn new(lib_name: &'a str) -> Result<Self, io::Error> {
        let dyn_lib = libloading::Library::new(lib_name)?;
        let actor_frame = LibraryFrame {
            lib_name,
            dyn_lib,
        };
        Ok(actor_frame)
    }

    pub fn get_actor_system(&self) -> Result<ActorSystem, io::Error> {
        unsafe {
            let init_func: libloading::Symbol<unsafe extern fn() -> ActorSystem> =
                self.dyn_lib.get(b"init")?;
            Ok(init_func())
        }
    }
}
