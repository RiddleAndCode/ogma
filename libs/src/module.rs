use super::matcher::{Match, MatchError};
use super::vm::{Callable, Func, Script};
use std::marker::PhantomData;

pub struct Cons<H, T> {
    head: PhantomData<H>,
    tail: PhantomData<T>,
}

pub struct Nil;

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

impl<'a, H, T, C> ModuleType<'a, C> for Cons<H, T>
where
    H: 'a + Match<'a, C> + Callable,
    T: ModuleType<'a, C, Error = MatchError>,
    <T as ModuleType<'a, C>>::Error: Into<MatchError>,
{
    type Error = MatchError;
    fn compile_line(ctx: &mut C, string: &'a str) -> Result<Box<dyn Callable + 'a>, Self::Error> {
        if let Ok(matched) = H::match_str(ctx, string) {
            Ok(Box::new(matched))
        } else {
            T::compile_line(ctx, string)
        }
    }
}

impl<'a, C> ModuleType<'a, C> for Nil {
    type Error = MatchError;
    fn compile_line(_: &mut C, _: &'a str) -> Result<Box<dyn Callable + 'a>, Self::Error> {
        Err(MatchError::UnexpectedEof)
    }
}

#[macro_export]
macro_rules! ogma_mod {
    () => {
        ::ogma::module::Nil
    };
    ($head:ty) => {
        ::ogma::module::Cons<$head, ::ogma::module::Nil>
    };
    ($head:ty, $($tail:ty),*) => {
        ::ogma::module::Cons<$head, ogma_mod!($($tail),*)>
    }
}
