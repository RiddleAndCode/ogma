use ogma_libs::clause::{parse, ParseError, Token};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{
    punctuated::Punctuated,
    token::{Comma, Const, Eq, Semi},
    Error, Ident, LitStr, Visibility,
};

pub struct ClauseArgs {
    pub vis: Visibility,
    pub name: Ident,
    pub tokens: Punctuated<TokenOwned, Comma>,
}

pub enum TokenOwned {
    Static(String),
    QueryVar(String),
    DataVar(String),
}

impl<'a> From<Token<'a>> for TokenOwned {
    fn from(t: Token<'a>) -> Self {
        match t {
            Token::Static(s) => TokenOwned::Static(s.to_owned()),
            Token::QueryVar(s) => TokenOwned::QueryVar(s.to_owned()),
            Token::DataVar(s) => TokenOwned::DataVar(s.to_owned()),
        }
    }
}

impl ToTokens for TokenOwned {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            TokenOwned::Static(s) => quote! { ::ogma::clause::Token::Static(#s) },
            TokenOwned::QueryVar(s) => quote! { ::ogma::clause::Token::QueryVar(#s) },
            TokenOwned::DataVar(s) => quote! { ::ogma::clause::Token::DataVar(#s) },
        });
    }
}

impl Parse for ClauseArgs {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let vis = input.parse()?;
        let _ = input.parse::<Const>();
        let name = input.parse()?;
        let _ = input.parse::<Eq>()?;
        let clause_lit_str = input.parse::<LitStr>()?;
        let clause_str = clause_lit_str.value();
        let tokens = parse(&clause_str)
            .collect::<Result<Vec<Token>, ParseError>>()
            .map_err(|e| Error::new(clause_lit_str.span(), e))?
            .into_iter()
            .map(TokenOwned::from)
            .collect();
        let _ = input.parse::<Semi>();
        Ok(ClauseArgs { vis, name, tokens })
    }
}
