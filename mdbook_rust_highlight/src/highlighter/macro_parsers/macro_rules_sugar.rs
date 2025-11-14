use syn::{Block, Token, parse::Parse, token::Paren};

use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};

#[derive(Debug)]
pub struct EmptyMacroRulesDefinition {
    pub paren: Paren,
    pub fat_arrow: Token![=>],
    pub block: Block,
    pub semi: Option<Token![;]>,
}

impl Parse for EmptyMacroRulesDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _paren_content;
        Ok(Self {
            paren: syn::parenthesized!(_paren_content in input),
            fat_arrow: input.parse()?,
            block: input.parse()?,
            semi: input.parse()?,
        })
    }
}

impl Register for EmptyMacroRulesDefinition {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.block);
    }
}
