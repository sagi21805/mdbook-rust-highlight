use crate::highlighter::RustHighlighter;
use mdbook_rust_highlight_derive::{make_register_wrappers, register_variants};
use strum_macros::AsRefStr;

/// Token mapping with
#[register_variants]
#[derive(AsRefStr, Debug)]
pub enum TokenTag {
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
    LifeTime,
}

impl ToString for TokenTag {
    fn to_string(&self) -> String {
        match self {
            Self::EndOfToken => String::from("</span>"),
            _ => format!("<span class=\"hlrs-{}\">", self.as_ref()),
        }
    }
}
