use crate::highlighter::RustHighlighter;
use mdbook_rust_highlight_derive::{add_try_method, register_variants};
use strum_macros::AsRefStr;

/// Token mapping with
#[register_variants]
#[derive(AsRefStr, Debug, Clone)]
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
    Segment,
    NeedIdentification,
    Comment,
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
