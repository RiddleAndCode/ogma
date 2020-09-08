//! Clause parsing utilities

use core::fmt;

/// A token parsed from a clause string
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token<'a> {
    /// A static one word token
    Static(&'a str),
    /// A query variable
    QueryVar(&'a str),
    /// A data variable
    DataVar(&'a str),
}

/// An error which may occur during parsing
#[derive(Debug)]
pub enum ParseError {
    /// Variable name is invalid
    InvalidVariableName,
    /// Prefix is invalid. May only be `q` or `d`
    InvalidVariablePrefix,
}

impl<'a> Token<'a> {
    /// Is the token a static token
    pub fn is_static(&self) -> bool {
        match self {
            Self::Static(_) => true,
            _ => false,
        }
    }

    /// Is the token a query variable
    pub fn is_query_var(&self) -> bool {
        match self {
            Self::QueryVar(_) => true,
            _ => false,
        }
    }

    /// Is the token a data variable
    pub fn is_data_var(&self) -> bool {
        match self {
            Self::DataVar(_) => true,
            _ => false,
        }
    }
}

fn parse_next(src: &str) -> Result<Option<(Token, &str)>, ParseError> {
    let (tok, rest) = if let Ok((_, tok, rest)) = nl_parser::parse_token(src) {
        (tok, rest)
    } else {
        return Ok(None);
    };
    if tok.contains('`') {
        if !tok.ends_with('`') {
            return Err(ParseError::InvalidVariableName);
        } else if tok.starts_with("q`") {
            // TODO check variable name format?
            let var = &tok[2..tok.len() - 1];
            return Ok(Some((Token::QueryVar(var), rest)));
        } else if tok.starts_with("d`") {
            // TODO check variable name format?
            let var = &tok[2..tok.len() - 1];
            return Ok(Some((Token::DataVar(var), rest)));
        } else {
            return Err(ParseError::InvalidVariablePrefix);
        }
    } else {
        return Ok(Some((Token::Static(tok), rest)));
    }
}

/// A parser for a clause. Iterates over tokens
pub struct Parser<'a> {
    src: &'a str,
}

impl<'a> Parser<'a> {
    /// Create a new Parser for a string
    #[inline]
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Token<'a>, ParseError>;
    fn next(&mut self) -> Option<Self::Item> {
        match parse_next(self.src) {
            Ok(Some((tok, rest))) => {
                self.src = rest;
                Some(Ok(tok))
            }
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidVariablePrefix => f.write_str("invalid variable prefix"),
            ParseError::InvalidVariableName => f.write_str("invalid variable name"),
        }
    }
}

/// Parse a string into tokens
#[inline]
pub fn parse<'a>(string: &'a str) -> impl Iterator<Item = Result<Token<'a>, ParseError>> {
    Parser::new(string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_next() -> Result<(), ParseError> {
        assert_eq!(parse_next("the")?, Some((Token::Static("the"), "")));
        assert_eq!(
            parse_next("the token")?,
            Some((Token::Static("the"), "token"))
        );
        assert_eq!(
            parse_next("q`variable`")?,
            Some((Token::QueryVar("variable"), ""))
        );
        assert_eq!(
            parse_next("q`variable` token")?,
            Some((Token::QueryVar("variable"), "token"))
        );
        assert_eq!(
            parse_next("d`variable`")?,
            Some((Token::DataVar("variable"), ""))
        );
        assert_eq!(
            parse_next("d`variable` token")?,
            Some((Token::DataVar("variable"), "token"))
        );
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(), ParseError> {
        assert_eq!(
            parse("the token string").collect::<Result<Vec<Token>, ParseError>>()?,
            vec![
                Token::Static("the"),
                Token::Static("token"),
                Token::Static("string")
            ]
        );
        assert_eq!(
            parse("the q`variable` token").collect::<Result<Vec<Token>, ParseError>>()?,
            vec![
                Token::Static("the"),
                Token::QueryVar("variable"),
                Token::Static("token")
            ]
        );
        assert_eq!(
            parse("the d`variable` token").collect::<Result<Vec<Token>, ParseError>>()?,
            vec![
                Token::Static("the"),
                Token::DataVar("variable"),
                Token::Static("token")
            ]
        );
        Ok(())
    }
}
