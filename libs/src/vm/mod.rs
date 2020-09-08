//! Virtual machine for running functions

mod context;
mod func;
mod script;
mod trap;

pub use context::Context;
pub use func::{Callable, Func};
pub use script::{Instance, Script};
pub use trap::Trap;
