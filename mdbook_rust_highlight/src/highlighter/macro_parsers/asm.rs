use syn::{Ident, LitStr, Token, parse::Parse, punctuated::Punctuated, token::Paren};

use super::ident_list::IdentList;

pub struct AsmInputs {
    inputs: Punctuated<AsmInput, Token![,]>,
}

pub enum AsmInput {
    Instruction(LitStr),
    Options(AsmOptions),
}

/// Structs represents the options on asm! macro `options(nostack, noreturn)`
pub struct AsmOptions {
    pub option_token: Ident,
    pub paren: Paren,
    pub options: IdentList,
}

impl Parse for AsmOptions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let option_token: Ident = input.parse()?;
        if option_token.to_string() != "options" {
            return Err(syn::Error::new_spanned(
                option_token,
                "Expected ident to be options",
            ));
        }
        let content;
        let paren = syn::parenthesized!(content in input);
        let options: IdentList = content.parse()?;
        Ok(Self {
            option_token,
            paren,
            options,
        })
    }
}
