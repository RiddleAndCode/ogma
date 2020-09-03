#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use clause_parser::{parse, ParseError, Token};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{
    punctuated::Punctuated,
    token::{Comma, Const, Eq, Pub, Semi},
    Error, Ident, LitStr,
};

struct ClauseArgs {
    vis: Option<Pub>,
    name: Ident,
    tokens: Punctuated<TokenOwned, Comma>,
}

enum TokenOwned {
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            TokenOwned::Static(s) => quote! { ::clause::Token::Static(#s) },
            TokenOwned::QueryVar(s) => quote! { ::clause::Token::QueryVar(#s) },
            TokenOwned::DataVar(s) => quote! { ::clause::Token::DataVar(#s) },
        });
    }
}

impl Parse for ClauseArgs {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let lookahead = input.lookahead1();
        let vis = if lookahead.peek(Pub) {
            Some(input.parse()?)
        } else {
            None
        };
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

#[proc_macro]
pub fn clause(args: TokenStream) -> TokenStream {
    let ClauseArgs { vis, name, tokens } = parse_macro_input!(args as ClauseArgs);
    let len = tokens.len();
    let out = quote! {
        #vis const #name: [::clause::Token<'static>; #len] = [#tokens];
    };
    out.into()
}
