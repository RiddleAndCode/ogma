#[macro_use]
extern crate ogma;

use ogma::clause::Token;
use ogma::matcher::*;
use ogma::object_query::Query;

#[derive(Debug, Eq, PartialEq)]
pub struct Add<'a> {
    a: i32,
    b: i32,
    out: Vec<Query<'a>>,
}

impl<'a> Add<'a> {
    clause! {
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
