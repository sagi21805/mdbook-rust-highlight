use syn::{Ident, Token, parse::Parse, punctuated::Punctuated};

pub struct IdentList {
    pub idents: Punctuated<Ident, Token![,]>,
}

impl Parse for IdentList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let idents = input.parse_terminated(Ident::parse, Token![,])?;
        Ok(IdentList { idents })
    }
}

pub struct ExperList {
    pub exprs: Punctuated<syn::Expr, Token![,]>,
}

impl Parse for ExperList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exprs = input.parse_terminated(syn::Expr::parse, Token![,])?;
        Ok(ExperList { exprs })
    }
}
