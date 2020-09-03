#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{token, Attribute, Error, FnArg, Generics, Ident, ItemFn, LitStr, Pat, Visibility};

struct Descriptor {
    name: Ident,
    clause_span: Span,
    clause_str: String,
}

impl Descriptor {
    fn name(&self) -> Ident {
        self.name.clone()
    }

    fn parse_clause(&self) -> Result<Vec<clause::Token>, Error> {
        clause::parse(&self.clause_str)
            .collect::<Result<Vec<clause::Token>, clause::ParseError>>()
            .map_err(|e| Error::new(self.clause_span, e))
    }

    fn var_names(&self) -> Result<Vec<Ident>, Error> {
        self.parse_clause()?
            .into_iter()
            .filter_map(|t| match t {
                clause::Token::Static(_) => None,
                clause::Token::QueryVar(s) => Some(s),
                clause::Token::DataVar(s) => Some(s),
            })
            .map(|s| syn::parse_str::<Ident>(s))
            .collect()
    }
}

impl Parse for Descriptor {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let name = input.parse()?;
        let _ = input.parse::<token::Comma>()?;
        let clause = input.parse::<LitStr>()?;
        let clause_span = clause.span();
        let clause_str = clause.value();
        Ok(Descriptor {
            name,
            clause_span,
            clause_str,
        })
    }
}

struct Func {
    inner: ItemFn,
}

impl Func {
    fn parse_generics(&self) -> Result<Generics, Error> {
        let generics = self.inner.sig.generics.clone();
        if generics.lifetimes().count() > 1 {
            return Err(Error::new(
                generics.lifetimes().nth(1).unwrap().lifetime.ident.span(),
                "invalid multiple lifetimes",
            ));
        }
        Ok(generics)
    }

    fn vis(&self) -> Visibility {
        self.inner.vis.clone()
    }

    fn attrs(&self) -> Vec<Attribute> {
        self.inner.attrs.clone()
    }

    fn vars(&self) -> Punctuated<FnArg, token::Comma> {
        self.inner.sig.inputs.clone().into_iter().skip(1).collect()
    }

    fn parse_var_names(&self) -> Result<Vec<Ident>, Error> {
        self.vars()
            .iter()
            .map(|v| match v {
                FnArg::Receiver(r) => {
                    return Err(Error::new(r.self_token.span, "invalid self"));
                }
                FnArg::Typed(t) => match t.pat.as_ref() {
                    Pat::Ident(n) => Ok(n.ident.clone()),
                    t => Err(Error::new_spanned(t, "invalid argument type")),
                },
            })
            .collect()
    }
}

impl Parse for Func {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let inner = input.parse()?;
        Ok(Func { inner })
    }
}

struct Struct {
    attrs: Vec<Attribute>,
    name: Ident,
    vis: Visibility,
    generics: Generics,
    vars: Punctuated<FnArg, token::Comma>,
}

impl Struct {
    fn construct(desc: &Descriptor, func: &Func) -> Result<Self, Error> {
        let possible_names = desc.var_names()?;
        for var in func.parse_var_names()? {
            if !possible_names.iter().any(|v1| v1 == &var) {
                return Err(Error::new(var.span(), "variable not found in clause"));
            }
        }
        Ok(Struct {
            attrs: func.attrs(),
            name: desc.name(),
            vis: func.vis(),
            generics: func.parse_generics()?,
            vars: func.vars(),
        })
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attrs = &self.attrs;
        let vis = &self.vis;
        let name = &self.name;
        let generics = &self.generics;
        let vars = &self.vars;
        tokens.extend(quote! {
            #(#attrs)*
            #vis struct #name#generics {
                #vars
            }
        });
    }
}

#[proc_macro_attribute]
pub fn ogma_fn(desc: TokenStream, func: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(desc as Descriptor);
    let func = parse_macro_input!(func as Func);
    let st = match Struct::construct(&desc, &func) {
        Ok(st) => st,
        Err(err) => return err.to_compile_error().into(),
    };
    let tokens = quote! {
        #st
    };
    tokens.into()
}
