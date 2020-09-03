#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std as core;

extern crate alloc;

mod context;
mod func;
mod script;
mod trap;

pub use context::Context;
pub use func::{Callable, Func};
pub use script::Script;
pub use trap::Trap;
