#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

mod clause_macro;
mod fn_macro;

use proc_macro::TokenStream;

/// Parse a clause into a static list of tokens
#[proc_macro]
pub fn clause(args: TokenStream) -> TokenStream {
    let clause_macro::ClauseArgs { vis, name, tokens } =
        parse_macro_input!(args as clause_macro::ClauseArgs);
    let len = tokens.len();
    let out = quote! {
        #vis const #name: [::ogma::clause::Token<'static>; #len] = [#tokens];
    };
    out.into()
}

/// Derive a callable and matchable function structure from a function
#[proc_macro_attribute]
pub fn ogma_fn(desc: TokenStream, func: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(desc as fn_macro::Descriptor);
    let func = parse_macro_input!(func as fn_macro::Func);
    let tokens = match fn_macro::ogma_fn(&desc, &func, None) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error().into(),
    };
    tokens.into()
}

/// Derive a function structure which matches during the BDD "Given" state
#[proc_macro_attribute]
pub fn given(desc: TokenStream, func: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(desc as fn_macro::Descriptor);
    let func = parse_macro_input!(func as fn_macro::Func);
    let tokens = match fn_macro::ogma_fn(&desc, &func, Some(fn_macro::Bdd::Given)) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error().into(),
    };
    tokens.into()
}

/// Derive a function structure which matches during the BDD "When" state
#[proc_macro_attribute]
pub fn when(desc: TokenStream, func: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(desc as fn_macro::Descriptor);
    let func = parse_macro_input!(func as fn_macro::Func);
    let tokens = match fn_macro::ogma_fn(&desc, &func, Some(fn_macro::Bdd::When)) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error().into(),
    };
    tokens.into()
}

/// Derive a function structure which matches during the BDD "Then" state
#[proc_macro_attribute]
pub fn then(desc: TokenStream, func: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(desc as fn_macro::Descriptor);
    let func = parse_macro_input!(func as fn_macro::Func);
    let tokens = match fn_macro::ogma_fn(&desc, &func, Some(fn_macro::Bdd::Then)) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error().into(),
    };
    tokens.into()
}
