use object_query::Query;
use serde::Deserialize;

#[derive(Debug)]
pub enum MatchError {
    Nlsd(nlsd::Error),
    MismatchedStaticToken,
    EmptyQuery,
    UnknownQueryVar,
    UnknownDataVar,
    UnfilledVar,
    UnexpectedEof,
}

pub struct Matcher<'a> {
    src: &'a str,
}

impl<'a> Matcher<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    pub fn next_static(&mut self) -> Result<&'a str, MatchError> {
        if let Ok((_, tok, rest)) = nl_parser::parse_token(self.src) {
            self.src = rest;
            Ok(tok)
        } else {
            Err(MatchError::UnexpectedEof)
        }
    }

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

pub trait Match<'a, C>: Sized {
    fn match_str(ctx: &mut C, string: &'a str) -> Result<Self, MatchError>;
}

impl From<nlsd::Error> for MatchError {
    fn from(err: nlsd::Error) -> Self {
        Self::Nlsd(err)
    }
}
