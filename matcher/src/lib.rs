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

pub trait Match<'a>: Sized {
    fn match_str(string: &'a str) -> Result<Self, MatchError>;
}

impl From<nlsd::Error> for MatchError {
    fn from(err: nlsd::Error) -> Self {
        Self::Nlsd(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clause::Token;

    #[derive(Debug, Eq, PartialEq)]
    pub struct Add<'a> {
        a: i32,
        b: i32,
        out: Vec<Query<'a>>,
    }

    impl<'a> Add<'a> {
        clause::clause! {
            const CLAUSE = "Given the addition of d`b` and d`a` henceforth q`out`"
        }
    }

    impl<'a> Match<'a> for Add<'a> {
        fn match_str(string: &'a str) -> Result<Self, MatchError> {
            let mut a = None;
            let mut b = None;
            let mut out = None;
            let mut matcher = Matcher::new(string);
            for token in &Self::CLAUSE {
                match *token {
                    Token::Static(token) => {
                        if matcher.next_static()? != token {
                            return Err(MatchError::MismatchedStaticToken);
                        }
                    }
                    Token::QueryVar(name) => match name {
                        "out" => out = Some(matcher.next_query()?),
                        _ => return Err(MatchError::UnknownQueryVar),
                    },
                    Token::DataVar(name) => match name {
                        "a" => a = Some(matcher.next_data()?),
                        "b" => b = Some(matcher.next_data()?),
                        _ => return Err(MatchError::UnknownDataVar),
                    },
                }
            }
            Ok(Add {
                a: a.ok_or(MatchError::UnfilledVar)?,
                b: b.ok_or(MatchError::UnfilledVar)?,
                out: out.ok_or(MatchError::UnfilledVar)?,
            })
        }
    }

    #[test]
    fn add_match() -> Result<(), MatchError> {
        let add = Add::match_str("Given the addition of 4 and 3 henceforth the addition")?;
        assert_eq!(
            add,
            Add {
                a: 3,
                b: 4,
                out: vec![Query::key("addition")]
            }
        );
        Ok(())
    }
}
