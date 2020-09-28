use ogma_libs::clause;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    token, Attribute, Error, FnArg, GenericParam, Generics, Ident, ItemFn, Lifetime, LifetimeDef,
    LitStr, Pat, Path, Type, Visibility,
};

pub enum Bdd {
    Given,
    When,
    Then,
}

pub struct Descriptor {
    attrs: Vec<Attribute>,
    name: Ident,
    clause_span: Span,
    clause_str: String,
}

impl Descriptor {
    fn name(&self) -> Ident {
        self.name.clone()
    }

    fn clause(&self) -> LitStr {
        LitStr::new(&self.clause_str, self.clause_span)
    }

    fn attrs(&self) -> Vec<Attribute> {
        self.attrs.clone()
    }

    fn parse_clause(&self) -> Result<Vec<clause::Token>, Error> {
        clause::parse(&self.clause_str)
            .collect::<Result<Vec<clause::Token>, clause::ParseError>>()
            .map_err(|e| Error::new(self.clause_span, e))
    }

    fn parse_var_names(&self) -> Result<Vec<Ident>, Error> {
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

    fn parse_query_var_names(&self) -> Result<Vec<Ident>, Error> {
        self.parse_clause()?
            .into_iter()
            .filter_map(|t| match t {
                clause::Token::QueryVar(s) => Some(s),
                _ => None,
            })
            .map(|s| syn::parse_str::<Ident>(s))
            .collect()
    }

    fn parse_data_var_names(&self) -> Result<Vec<Ident>, Error> {
        self.parse_clause()?
            .into_iter()
            .filter_map(|t| match t {
                clause::Token::DataVar(s) => Some(s),
                _ => None,
            })
            .map(|s| syn::parse_str::<Ident>(s))
            .collect()
    }
}

impl Parse for Descriptor {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let attrs = Attribute::parse_outer(input)?;
        let name = input.parse()?;
        let _ = input.parse::<token::Comma>()?;
        let clause = input.parse::<LitStr>()?;
        let clause_span = clause.span();
        let clause_str = clause.value();
        Ok(Descriptor {
            attrs,
            name,
            clause_span,
            clause_str,
        })
    }
}

pub struct Func {
    inner: ItemFn,
}

impl Func {
    fn generics(&self) -> Generics {
        self.inner.sig.generics.clone()
    }

    fn vis(&self) -> Visibility {
        self.inner.vis.clone()
    }

    fn name(&self) -> Ident {
        self.inner.sig.ident.clone()
    }

    fn inner(&self) -> ItemFn {
        self.inner.clone()
    }

    fn vars(&self) -> Punctuated<FnArg, token::Comma> {
        self.inner.sig.inputs.clone().into_iter().skip(1).collect()
    }

    fn lifetime(&self) -> Option<LifetimeDef> {
        self.inner.sig.generics.lifetimes().next().cloned()
    }

    fn parse_var_names(&self) -> Result<Vec<Ident>, Error> {
        Ok(self.parse_vars()?.into_iter().map(|v| v.name).collect())
    }

    fn parse_vars(&self) -> Result<Vec<FuncVar>, Error> {
        self.vars()
            .iter()
            .map(|v| match v {
                FnArg::Receiver(r) => {
                    return Err(Error::new(r.self_token.span, "invalid self"));
                }
                FnArg::Typed(t) => {
                    let name = match t.pat.as_ref() {
                        Pat::Ident(n) => n.ident.clone(),
                        t => return Err(Error::new_spanned(t, "invalid argument type")),
                    };
                    let (is_referenced, ty) = match t.ty.as_ref() {
                        Type::Reference(ty) => {
                            if let Some(m) = ty.mutability {
                                return Err(Error::new(m.span, "arguments cannot be mutable"));
                            }
                            if let Some(l) = ty.lifetime.clone() {
                                if Some(l.clone()) != self.lifetime().map(|l| l.lifetime) {
                                    return Err(Error::new(
                                        l.ident.span(),
                                        "mismatching lifetimes",
                                    ));
                                }
                                (false, Box::new(Type::Reference(ty.clone())))
                            } else {
                                (true, ty.elem.clone())
                            }
                        }
                        ty => (false, Box::new(ty.clone())),
                    };
                    Ok(FuncVar {
                        name,
                        is_referenced,
                        ty,
                    })
                }
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct FuncVar {
    name: Ident,
    is_referenced: bool,
    ty: Box<Type>,
}

impl FuncVar {
    pub fn is_static_type(&self) -> bool {
        let mut expecting_lifetime = false;
        for token in self.ty.to_token_stream() {
            match token {
                proc_macro2::TokenTree::Punct(punct) => {
                    if punct.as_char() == '\'' {
                        expecting_lifetime = true;
                    }
                }
                proc_macro2::TokenTree::Ident(ident) => {
                    if expecting_lifetime && ident.to_string() != "static" {
                        return false;
                    }
                    expecting_lifetime = false;
                }
                _ => (),
            }
        }
        true
    }
}

impl Parse for Func {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let inner = input.parse()?;
        Ok(Func { inner })
    }
}

pub struct Struct {
    attrs: Vec<Attribute>,
    name: Ident,
    vis: Visibility,
    generics: Generics,
    vars: Vec<FuncVar>,
}

impl Struct {
    pub fn build(desc: &Descriptor, func: &Func) -> Result<Self, Error> {
        Ok(Struct {
            attrs: desc.attrs(),
            name: desc.name(),
            vis: func.vis(),
            generics: func.generics(),
            vars: func.parse_vars()?,
        })
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = &self.attrs;
        let vis = &self.vis;
        let name = &self.name;
        let generics = &self.generics;
        let vars = self.vars.iter().map(|var| {
            let name = &var.name;
            let ty = &var.ty;
            quote! { #name: #ty, }
        });
        tokens.extend(quote! {
            #(#attrs)*
            #vis struct #name #generics {
                #(#vars)*
            }
        });
    }
}

pub struct StructImpl {
    name: Ident,
    generics: Generics,
    clause: LitStr,
    func: ItemFn,
}

impl StructImpl {
    pub fn build(desc: &Descriptor, func: &Func) -> Self {
        let mut generics = func.generics();
        generics.lifetimes_mut().for_each(|mut l| {
            l.lifetime = Lifetime::new(&format!("'_{}", l.lifetime.ident), Span::call_site());
        });
        Self {
            name: desc.name(),
            generics,
            clause: desc.clause(),
            func: func.inner(),
        }
    }
}

impl ToTokens for StructImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let generics = &self.generics;
        let clause = &self.clause;
        let func = &self.func;
        tokens.extend(quote! {
            impl #generics #name #generics {
                ::ogma::clause! {
                    const CLAUSE = #clause
                }
                #func
            }
        });
    }
}

pub struct MatchImpl {
    name: Ident,
    struct_generics: Generics,
    impl_generics: Generics,
    lifetime: Lifetime,
    match_ctx: Path,
    bdd: Option<Bdd>,
    query_vars: Vec<FuncVar>,
    data_vars: Vec<FuncVar>,
}

impl MatchImpl {
    pub fn build(desc: &Descriptor, func: &Func, bdd: Option<Bdd>) -> Result<Self, Error> {
        let struct_generics = func.generics();
        let (mut impl_generics, lifetime) = if struct_generics.lifetimes().count() < 1 {
            let lifetime = Lifetime::new("'a", Span::call_site());
            let param = GenericParam::Lifetime(LifetimeDef::new(lifetime.clone()));
            let mut impl_generics = struct_generics.clone();
            impl_generics.params.insert(0, param);
            (impl_generics, lifetime)
        } else {
            let lifetime = struct_generics.lifetimes().next().unwrap().lifetime.clone();
            (struct_generics.clone(), lifetime)
        };
        let match_ctx = if bdd.is_none() {
            let match_ctx = Ident::new("MCtx", Span::call_site());
            impl_generics
                .params
                .push(GenericParam::Type(match_ctx.clone().into()));
            match_ctx.into()
        } else {
            parse_quote!(::ogma::bdd::Step)
        };
        let func_vars = func.parse_vars()?;
        let query_vars = desc
            .parse_query_var_names()?
            .iter()
            .map(|ident| {
                func_vars
                    .iter()
                    .find(|var| &var.name == ident)
                    .cloned()
                    .ok_or_else(|| Error::new(ident.span(), "could not find ariable in func"))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let data_vars = desc
            .parse_data_var_names()?
            .iter()
            .map(|ident| {
                func_vars
                    .iter()
                    .find(|var| &var.name == ident)
                    .cloned()
                    .ok_or_else(|| Error::new(ident.span(), "could not find ariable in func"))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(Self {
            name: desc.name(),
            lifetime,
            struct_generics,
            impl_generics,
            match_ctx,
            bdd,
            query_vars,
            data_vars,
        })
    }
}

impl ToTokens for MatchImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let struct_generics = &self.struct_generics;
        let impl_generics = &self.impl_generics;
        let lifetime = &self.lifetime;
        let match_ctx = &self.match_ctx;
        let var_declarations = self
            .query_vars
            .iter()
            .chain(self.data_vars.iter())
            .map(|var| {
                let name = &var.name;
                quote! { let mut #name = None; }
            });
        let var_assignments = self
            .query_vars
            .iter()
            .chain(self.data_vars.iter())
            .map(|var| {
                let name = &var.name;
                quote! { #name: #name.ok_or(::ogma::matcher::MatchError::UnfilledVar)?, }
            });
        let query_var_matches = self.query_vars.iter().map(|var| {
            let name = &var.name;
            let name_str = name.to_string();
            if var.is_static_type() {
                quote! { #name_str => #name = Some(m.next_query_owned()?), }
            } else {
                quote! { #name_str => #name = Some(m.next_query()?), }
            }
        });
        let data_var_matches = self.data_vars.iter().map(|var| {
            let name = &var.name;
            let name_str = name.to_string();
            quote! { #name_str => #name = Some(m.next_data()?), }
        });
        let bdd_check = if let Some(ref bdd) = self.bdd {
            let verb = match bdd {
                Bdd::Given => "Given",
                Bdd::When => "When",
                Bdd::Then => "Then",
            };
            quote! {
                let token = match m.next_static()? {
                    #verb => #verb,
                    "And" => "And",
                    _ => return Err(::ogma::matcher::MatchError::MismatchedStaticToken),
                };
                if let Some(next_state) = ctx.next(token) {
                    *ctx = next_state;
                } else {
                    return Err(::ogma::matcher::MatchError::InvalidCtx);
                }
            }
        } else {
            quote! {}
        };
        tokens.extend(quote! {
            impl #impl_generics ::ogma::matcher::Match<#lifetime, #match_ctx> for #name #struct_generics {
                fn match_str(ctx: &mut #match_ctx, s: &#lifetime str) -> Result<Self, ::ogma::matcher::MatchError> {
                    let mut m = ::ogma::matcher::Matcher::new(s);
                    #bdd_check
                    #(#var_declarations)*
                    for token in &Self::CLAUSE {
                        match *token {
                            ::ogma::clause::Token::Static(token) => {
                                if m.next_static()? != token {
                                    return Err(::ogma::matcher::MatchError::MismatchedStaticToken);
                                }
                            },
                            ::ogma::clause::Token::QueryVar(name) => match name {
                                #(#query_var_matches)*
                                _ => return Err(::ogma::matcher::MatchError::UnknownQueryVar),
                            },
                            ::ogma::clause::Token::DataVar(name) => match name {
                                #(#data_var_matches)*
                                _ => return Err(::ogma::matcher::MatchError::UnknownDataVar),
                            },
                        }
                    }
                    if m.is_empty() {
                        Ok(#name {
                            #(#var_assignments)*
                        })
                    } else {
                        Err(::ogma::matcher::MatchError::ExpectedEof)
                    }
                }
            }
        });
    }
}

pub struct CallableImpl {
    name: Ident,
    generics: Generics,
    fn_name: Ident,
    vars: Vec<FuncVar>,
}

impl CallableImpl {
    pub fn build(desc: &Descriptor, func: &Func) -> Result<Self, Error> {
        Ok(Self {
            name: desc.name(),
            generics: func.generics(),
            fn_name: func.name(),
            vars: func.parse_vars()?,
        })
    }
}

impl ToTokens for CallableImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let generics = &self.generics;
        let fn_name = &self.fn_name;
        let fn_args = self.vars.iter().map(|var| {
            let name = &var.name;
            if var.is_referenced {
                quote! { &self.#name, }
            } else {
                quote! { self.#name, }
            }
        });
        tokens.extend(quote! {
            impl #generics ::ogma::vm::Callable for #name #generics {
                fn call(&self, ctx: &mut ::ogma::vm::Context) -> Result<(), ::ogma::vm::Trap> {
                    Self::#fn_name(
                        ctx,
                        #(#fn_args)*
                    )?;
                    Ok(())
                }
            }
        })
    }
}

fn check_var_names(desc: &Descriptor, func: &Func) -> Result<(), Error> {
    let possible_names = desc.parse_var_names()?;
    let func_names = func.parse_var_names()?;
    if possible_names.len() != func_names.len() {
        return Err(Error::new(desc.clause().span(), "variable number mismatch"));
    }
    for var in func.parse_var_names()? {
        if !possible_names.iter().any(|v1| v1 == &var) {
            return Err(Error::new(var.span(), "variable not found in clause"));
        }
    }
    Ok(())
}

fn check_generics(func: &Func) -> Result<(), Error> {
    let generics = func.generics();
    if generics.lifetimes().count() > 1 {
        return Err(Error::new(
            generics.lifetimes().nth(1).unwrap().lifetime.ident.span(),
            "invalid multiple lifetimes",
        ));
    }
    Ok(())
}

fn validate(desc: &Descriptor, func: &Func) -> Result<(), Error> {
    check_var_names(desc, func)?;
    check_generics(func)?;
    Ok(())
}

pub fn ogma_fn(desc: &Descriptor, func: &Func, bdd: Option<Bdd>) -> Result<TokenStream, Error> {
    validate(&desc, &func)?;
    let fn_struct = Struct::build(&desc, &func)?;
    let struct_impl = StructImpl::build(&desc, &func);
    let match_impl = MatchImpl::build(&desc, &func, bdd)?;
    let callable_impl = CallableImpl::build(&desc, &func)?;
    Ok(quote! {
        #fn_struct
        #struct_impl
        #match_impl
        #callable_impl
    })
}
