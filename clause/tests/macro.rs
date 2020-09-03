#[macro_use]
extern crate clause;

use clause::Token;

clause! { const CLAUSE0 = ""; }
clause! { pub const CLAUSE1 = "the"; }
clause! { pub(crate) const CLAUSE2 = "q`query_name`"; }
clause! { const CLAUSE3 = "d`data_name`"; }
clause! { const CLAUSE4 = "the q`query_name` and d`data_name` tokens"; }

#[test]
fn empty() {
    assert_eq!(CLAUSE0, []);
}

#[test]
fn static_token() {
    assert_eq!(CLAUSE1, [Token::Static("the")]);
}

#[test]
fn query_token() {
    assert_eq!(CLAUSE2, [Token::QueryVar("query_name")]);
}

#[test]
fn data_token() {
    assert_eq!(CLAUSE3, [Token::DataVar("data_name")]);
}

#[test]
fn mixed_tokens() {
    assert_eq!(
        CLAUSE4,
        [
            Token::Static("the"),
            Token::QueryVar("query_name"),
            Token::Static("and"),
            Token::DataVar("data_name"),
            Token::Static("tokens")
        ]
    );
}
