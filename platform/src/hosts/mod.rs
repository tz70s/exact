// This project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

//! This module is responsible for handling hosts registry.
//! To store the metadata information and checkout if virtual actor instance existed or not.

extern crate libloading;

pub mod loader;
pub mod cache;
pub mod registry;
