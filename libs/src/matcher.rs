//! Function matching utilities

use crate::vm::{Callable, Func};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use object_query::Query;
use serde::Deserialize;

/// An error which can occur during matchng
#[derive(Debug)]
pub enum MatchError {
    /// NLSD parse error
    Nlsd(nlsd::Error),
    /// Token mismatch
    MismatchedStaticToken,
    /// Missing query
    EmptyQuery,
    /// Mismatched query Name
    UnknownQueryVar,
    /// Mismatched data Name
    UnknownDataVar,
    /// Var left empty
    UnfilledVar,
    /// Expected more tokens to match against
    UnexpectedEof,
    /// Matching has finished but there is still more string
    ExpectedEof,
    /// Invalid matching context
    InvalidCtx,
}

/// Reads tokens, queries and data from a string
pub struct Matcher<'a> {
    src: &'a str,
}

impl<'a> Matcher<'a> {
    /// Create a new Matcher instance
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    /// Get the next static token from the string
    pub fn next_static(&mut self) -> Result<&'a str, MatchError> {
        if let Ok((_, tok, rest)) = nl_parser::parse_token(self.src) {
            self.src = rest;
            Ok(tok)
        } else {
            Err(MatchError::UnexpectedEof)
        }
    }

    /// Get the next NLOQ query from the string
    pub fn next_query(&mut self) -> Result<Vec<Query<'a>>, MatchError> {
        let mut nloq_de = nloq::Deserializer::from_str(self.src);
        let query = nloq_de.query();
        if query.is_empty() {
            Err(MatchError::EmptyQuery)
        } else {
            self.src = nloq_de.rest();
            Ok(query)
        }
    }

    /// Get the next NLOQ query from the string
    pub fn next_query_owned(&mut self) -> Result<Vec<Query<'static>>, MatchError> {
        Ok(self
            .next_query()?
            .into_iter()
            .map(|q| q.to_owned())
            .collect())
    }

    /// Get the next NLSD object from the string and deserialize into `T`
    pub fn next_data<T>(&mut self) -> Result<T, MatchError>
    where
        T: Deserialize<'a>,
    {
        let mut nlsd_de = nlsd::Deserializer::from_str(self.src);
        let out = T::deserialize(&mut nlsd_de)?;
        self.src = nlsd_de.rest();
        Ok(out)
    }

    /// Check if the matcher contains more tokens
    pub fn is_empty(&self) -> bool {
        self.src.trim_start().is_empty()
    }
}

/// Create `Self` from a string slice given a context. This should be implemented by functions that
/// wish to be compileable and read their arguments from an English string
pub trait Match<'a, C>: Sized {
    fn match_str(ctx: &mut C, string: &'a str) -> Result<Self, MatchError>;
}

/// A function pointer which matches a string given a context to a callable Func
pub type FuncMatcher<'a, C> = fn(&mut C, &'a str) -> Result<Func<'a>, MatchError>;

/// Auto implemented trait for types which both implement `Match` and `Callable`
pub trait MatchFunc<'a, C>: 'a + Match<'a, C> + Callable {
    fn match_func(ctx: &mut C, string: &'a str) -> Result<Func<'a>, MatchError> {
        Self::match_str(ctx, string).map(|this| Box::new(this) as Box<dyn Callable>)
    }
}

impl<'a, C, T> MatchFunc<'a, C> for T where T: 'a + Match<'a, C> + Callable {}

impl From<nlsd::Error> for MatchError {
    fn from(err: nlsd::Error) -> Self {
        Self::Nlsd(err)
    }
}

impl fmt::Display for MatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nlsd(err) => f.write_fmt(format_args!("NLSD err: {}", err)),
            Self::MismatchedStaticToken => f.write_str("mismatched static token"),
            Self::EmptyQuery => f.write_str("empty NLOQ query"),
            Self::UnknownQueryVar => f.write_str("mismatched query variable name"),
            Self::UnknownDataVar => f.write_str("mismatched data variable name"),
            Self::UnfilledVar => f.write_str("variable not set"),
            Self::UnexpectedEof => f.write_str("unexpected end of file"),
            Self::ExpectedEof => f.write_str("clause has extra tokens"),
            Self::InvalidCtx => f.write_str("context error when parsing"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MatchError {}
