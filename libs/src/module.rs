//! Script parsing utilities

use super::matcher::{FuncMatcher, Match, MatchError};
use super::vm::{Callable, Func, Script};
use std::marker::PhantomData;

/// A list of FuncMatchers for a given context. Output of `mod_list!` macro
pub type ModuleList<'a, C> = Box<[FuncMatcher<'a, C>]>;

/// A Type which represents two types H and T
pub struct Cons<H, T> {
    head: PhantomData<H>,
    tail: PhantomData<T>,
}

/// A Type which represents the empty Type
pub struct Nil;

/// Types which implement `ModuleType` can compile a line into a `Func` and multiple lines into a
/// `Script`
pub trait ModuleType<'a, C> {
    type Error;
    fn compile_line(ctx: &mut C, string: &'a str) -> Result<Func<'a>, Self::Error>;
    fn compile(ctx: &mut C, string: &'a str) -> Result<Script<'a>, Self::Error> {
        let mut script = Vec::new();
        for line in string.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let func = Self::compile_line(ctx, line)?;
            script.push(func);
        }
        Ok(script.into())
    }
}

/// Types which implement `Module` can compile a line into a `Func` and multiple lines into a
/// `Script` through instance methods
pub trait Module<'a, C> {
    type Error;
    fn compile_line(&self, ctx: &mut C, string: &'a str) -> Result<Func<'a>, Self::Error>;
    fn compile(&self, ctx: &mut C, string: &'a str) -> Result<Script<'a>, Self::Error> {
        let mut script = Vec::new();
        for line in string.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let func = self.compile_line(ctx, line)?;
            script.push(func);
        }
        Ok(script.into())
    }
}

impl<'a, H, T, C> ModuleType<'a, C> for Cons<H, T>
where
    H: 'a + Match<'a, C> + Callable,
    T: ModuleType<'a, C, Error = MatchError>,
    <T as ModuleType<'a, C>>::Error: Into<MatchError>,
{
    type Error = MatchError;
    fn compile_line(ctx: &mut C, string: &'a str) -> Result<Box<dyn Callable + 'a>, Self::Error> {
        match H::match_str(ctx, string) {
            Ok(matched) => Ok(Box::new(matched)),
            Err(_) => T::compile_line(ctx, string),
        }
    }
}

impl<'a, C> ModuleType<'a, C> for Nil {
    type Error = MatchError;
    fn compile_line(_: &mut C, _: &'a str) -> Result<Box<dyn Callable + 'a>, Self::Error> {
        Err(MatchError::UnexpectedEof)
    }
}

impl<'a, C, T> Module<'a, C> for T
where
    T: AsRef<[FuncMatcher<'a, C>]>,
{
    type Error = MatchError;
    fn compile_line(&self, ctx: &mut C, string: &'a str) -> Result<Func<'a>, Self::Error> {
        for match_str in self.as_ref().into_iter() {
            if let Ok(func) = match_str(ctx, string) {
                return Ok(func);
            }
        }
        Err(MatchError::UnexpectedEof)
    }
}

/// Creates a ModuleType from a list of Types
///
/// ```skip
/// ogma::mod_type!(A, B, C) // => Cons<A, Cons<B, Cons<C, Nil>>>
/// ```
///
/// If `A`, `B` and `C` implement `Matcher` then `mod_type!(A, B, C)` should implement `Matcher`
#[macro_export]
macro_rules! mod_type {
    () => {
        $crate::module::Nil
    };
    ($head:ty) => {
        $crate::module::Cons<$head, $crate::module::Nil>
    };
    ($head:ty, $($tail:ty),*) => {
        $crate::module::Cons<$head, $crate::mod_type!($($tail),*)>
    }
}

/// Creates a Module from a list of types
///
/// ```skip
/// ogma::mod_list!(Ctx => A, B, C) // => ModuleList<'a¸ Ctx>
/// ```
///
/// If `A`, `B` and `C` implement `Matcher` and `Callable` then `mod_list!(Ctx => A, B, C)` should implement `Module<'a, Ctx>`
#[macro_export]
macro_rules! mod_list {
    () => {
        Box::new([])
    };
    ($ctx:ty => $($item:ty),*) => {
        Box::new([$(<$item as $crate::matcher::MatchFunc<$ctx>>::match_func),*])
    }
}
