#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std as core;

extern crate alloc;

pub mod bdd;
pub mod clause;
pub mod matcher;
pub mod module;
pub mod vm;
