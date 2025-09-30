use std::cmp::{Ordering, Reverse};

use crate::highlighter::RustHighlighter;
use mdbook_rust_highlight_derive::{RegisterVariants, add_try_method};
use strum_macros::AsRefStr;

#[derive(AsRefStr, RegisterVariants, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum TokenTag {
    Keyword,
    Ident,
    LitStr,
    LitNum,
    LitBool,
    Function,
    SelfToken,
    Macro,
    Type,
    Enum,
    Segment,
    Comment,
    LifeTime,
    NeedIdentification,
    Boring,
    EndOfToken,
}

impl ToString for TokenTag {
    fn to_string(&self) -> String {
        match self {
            Self::Boring => String::from("<span class=\"boring\">"),
            Self::EndOfToken => String::from("</span>"),
            _ => format!("<span class=\"hlrs-{}\">", self.as_ref()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SpannedToken {
    pub(crate) kind: TokenTag,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl PartialOrd for SpannedToken {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.start, Reverse(self.end), self.kind.clone()).partial_cmp(&(
            other.start,
            Reverse(other.end),
            other.kind.clone(),
        ))
    }
}

impl Ord for SpannedToken {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.start, Reverse(self.end), self.kind.clone())
            .cmp(&(other.start, Reverse(other.end), other.kind.clone()))
            .then(Ordering::Greater)
    }
}
