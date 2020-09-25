use crate::error::Fallible;
use ogma::clause::Token;

clause! { const CLAUSE0 = ""; }
clause! { pub const CLAUSE1 = "the"; }
clause! { pub(crate) const CLAUSE2 = "q`query_name`"; }
clause! { const CLAUSE3 = "d`data_name`"; }
clause! { const CLAUSE4 = "the q`query_name` and d`data_name` tokens"; }

#[cfg_attr(feature = "std", test)]
#[cfg_attr(not(feature = "std"), test_case)]
fn empty() -> Fallible<()> {
    assert_eq!(CLAUSE0, []);
    Ok(())
}

#[cfg_attr(feature = "std", test)]
#[cfg_attr(not(feature = "std"), test_case)]
fn static_token() -> Fallible<()> {
    assert_eq!(CLAUSE1, [Token::Static("the")]);
    Ok(())
}

#[cfg_attr(feature = "std", test)]
#[cfg_attr(not(feature = "std"), test_case)]
fn query_token() -> Fallible<()> {
    assert_eq!(CLAUSE2, [Token::QueryVar("query_name")]);
    Ok(())
}

#[cfg_attr(feature = "std", test)]
#[cfg_attr(not(feature = "std"), test_case)]
fn data_token() -> Fallible<()> {
    assert_eq!(CLAUSE3, [Token::DataVar("data_name")]);
    Ok(())
}

#[cfg_attr(feature = "std", test)]
#[cfg_attr(not(feature = "std"), test_case)]
fn mixed_tokens() -> Fallible<()> {
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
    Ok(())
}
