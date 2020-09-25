//! Ogma (named after the [this guy](https://en.wikipedia.org/wiki/Ogma)) is a library
//! to create Natural Language DSLs. Specifically, the library provides convenience macros
//! for wrapping a function with implementations to parse parameters from English.
//!
//! # Examples
//!
//! ```
//! # use ogma::vm::{Context, Trap};
//! # use ogma::object_query::Query;
//! # use ogma::given;
//! #[given(Add, "the addition of q`input` and d`constant` henceforth q`out`")]
//! fn add<'a>(
//!     ctx: &mut Context,
//!     input: &Vec<Query<'a>>,
//!     constant: i32,
//!     out: &Vec<Query<'a>>,
//! ) -> Result<(), Trap> {
//!     // get global variable from `ctx` using `input`, add `constant` to it
//!     // and save to `ctx` via `out`
//!     Ok(())
//! }
//! ```
//!
//! which you can then use in a Script
//!
//! ```ignore
//!    let mut ctx = bdd::Step::new();
//!    let script = Module::compile(
//!        &mut ctx,
//!        r#"
//!        Given the addition of the input and 2 henceforth the left
//!        And the product of the input and 2 henceforth the right
//!        When the left is equal to the right
//!        Then do nothing
//!        "#,
//!    )
//!    .unwrap();
//!    let mut instance = script.instance();
//!    instance.ctx_mut().set_global::<_, i32>("input", 2);
//!    assert!(instance.exec().is_ok());
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std as core;

pub extern crate object_query;

pub use ogma_libs::*;
pub use ogma_macros::*;
