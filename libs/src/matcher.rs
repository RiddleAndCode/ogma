//! Function matching utilities

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
    /// Expected more tokens
    UnexpectedEof,
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
}

/// Create `Self` from a string slice given a context. This should be implemented by functions that
/// wish to be compileable and read their arguments from an English string
pub trait Match<'a, C>: Sized {
    fn match_str(ctx: &mut C, string: &'a str) -> Result<Self, MatchError>;
}

impl From<nlsd::Error> for MatchError {
    fn from(err: nlsd::Error) -> Self {
        Self::Nlsd(err)
    }
}
