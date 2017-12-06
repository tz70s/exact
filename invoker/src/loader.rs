//! Internal implementation of wrapping dynamic lib loading.
//! The target loading library is the implementation from user's application logics.

use ::libloading;
use ::hello;

/// The record entity of implementation from user, also, a meta information of dynamic lib.
pub struct ActorFrame<'a> {
    lib_name: &'a str,
    dyn_lib: libloading::Library,
}

pub type Error = ::std::io::Error;

impl<'a> ActorFrame<'a> {

    /// Create a new actor lib frame.
    pub fn new(lib_name: &'a str) -> Result<Self, Error> {
        let dyn_lib = libloading::Library::new(lib_name)?;
        let actor_frame = ActorFrame {
            lib_name,
            dyn_lib,
        };
        Ok(actor_frame)
    }

    // TODO: Customized error type
    // Approximate hundreds ns at each call.
    pub fn recv(&self) -> Result<(), Error> {
        unsafe {
            let recv: libloading::Symbol<unsafe extern fn()> = self.dyn_lib.get(b"recv")?;
            Ok(recv())
        }
    }

    pub fn hello(&self) -> Result<(), Error> {
        unsafe {
            let hello: libloading::Symbol<unsafe extern fn()> = self.dyn_lib.get(b"hello")?;
            Ok(hello())
        }
    }

    pub fn get_actor(&self) -> Result<Box<hello::SayHello>, Error> {
        unsafe {
            let new_func: libloading::Symbol<unsafe extern fn() -> Box<hello::SayHello>> =
                self.dyn_lib.get(b"get_hello")?;
            Ok(new_func())
        }
    }
}
