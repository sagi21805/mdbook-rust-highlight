use crate::highlighter::RustHighlighter;
use mdbook_rust_highlight_derive::{add_try_method, register_variants};
use proc_macro2::TokenStream;
use strum_macros::AsRefStr;

/// Token mapping with
#[register_variants]
#[derive(AsRefStr, Debug, Clone)]
pub enum TokenTag<'ast> {
    Keyword,
    Ident,
    LitStr,
    LitNum,
    LitBool,
    EndOfToken,
    Function,
    SelfToken,
    Macro,
    Type,
    Enum,
    Segment,
    Comment,
    LifeTime,
    NeedIdentification,
    TokenStream(&'ast TokenStream),
}

impl<'ast> ToString for TokenTag<'ast> {
    fn to_string(&self) -> String {
        match self {
            Self::EndOfToken => String::from("</span>"),
            _ => format!("<span class=\"hlrs-{}\">", self.as_ref()),
        }
    }
}
