#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std as core;

#[cfg_attr(test, macro_use)]
extern crate alloc;

#[cfg_attr(test, macro_use)]
extern crate ogma;

#[cfg(test)]
mod bdd_macro;
#[cfg(test)]
mod bdd_matcher;
#[cfg(test)]
mod clause_macro;
#[cfg(test)]
mod fn_macro;
#[cfg(test)]
mod matcher;
