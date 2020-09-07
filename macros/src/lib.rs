#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

mod clause_macro;
mod fn_macro;

use proc_macro::TokenStream;

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

#[proc_macro_attribute]
pub fn ogma_fn(desc: TokenStream, func: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(desc as fn_macro::Descriptor);
    let func = parse_macro_input!(func as fn_macro::Func);
    let tokens = match fn_macro::ogma_fn(&desc, &func) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error().into(),
    };
    tokens.into()
}
