#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(custom_test_frameworks))]
#![cfg_attr(not(feature = "std"), feature(alloc_error_handler))]
#![cfg_attr(not(feature = "std"), feature(lang_items))]
#![cfg_attr(not(feature = "std"), test_runner(crate::no_std_tests::test_runner))]

#[cfg(feature = "std")]
extern crate std as core;

#[cfg_attr(test, macro_use)]
extern crate alloc;

#[cfg_attr(test, macro_use)]
extern crate ogma;

#[cfg(test)]
mod error;

#[cfg(test)]
#[cfg(not(feature = "std"))]
mod no_std_tests;

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
