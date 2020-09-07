use super::matcher::{Match, MatchError};
use super::vm::{Callable, Func, Script};
use std::marker::PhantomData;

pub struct Cons<H, T> {
    head: PhantomData<H>,
    tail: PhantomData<T>,
}

pub struct Nil;

pub trait ModuleType<'a> {
    type Error;
    fn compile_line(string: &'a str) -> Result<Func<'a>, Self::Error>;
    fn compile(string: &'a str) -> Result<Script<'a>, Self::Error> {
        string
            .lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| Self::compile_line(s))
            .collect::<Result<Vec<Func<'a>>, Self::Error>>()
            .map(|v| v.into())
    }
}

impl<'a, H, T> ModuleType<'a> for Cons<H, T>
where
    H: 'a + Match<'a> + Callable,
    T: ModuleType<'a, Error = MatchError>,
    <T as ModuleType<'a>>::Error: Into<MatchError>,
{
    type Error = MatchError;
    fn compile_line(string: &'a str) -> Result<Box<dyn Callable + 'a>, Self::Error> {
        if let Ok(matched) = H::match_str(string) {
            Ok(Box::new(matched))
        } else {
            T::compile_line(string)
        }
    }
}

impl<'a> ModuleType<'a> for Nil {
    type Error = MatchError;
    fn compile_line(_: &'a str) -> Result<Box<dyn Callable + 'a>, Self::Error> {
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
