use crate::highlighter::RustHighlighter;
use mdbook_rust_highlight_derive::{add_try_method, register_variants};
use strum_macros::AsRefStr;

/// Token mapping with
#[register_variants]
#[derive(AsRefStr, Debug, Clone)]
pub enum TokenTag {
    Keyword(usize),
    Ident(usize),
    LitStr(usize),
    LitNum(usize),
    LitBool(usize),
    EndOfToken(usize),
    Function(usize),
    SelfToken(usize),
    Macro(usize),
    Type(usize),
    Enum(usize),
    Segment(usize),
    Comment(usize),
    LifeTime(usize),
    NeedIdentification(usize),
    Boring(usize),
}

impl ToString for TokenTag {
    fn to_string(&self) -> String {
        match self {
            Self::Boring(_) => String::from("<span class=\"boring\">"),
            Self::EndOfToken(_) => String::from("</span>"),
            _ => format!("<span class=\"hlrs-{}\">", self.as_ref()),
        }
    }
}
