use std::cmp::{Ordering, Reverse};

use crate::highlighter::RustHighlighter;
use mdbook_rust_highlight_derive::{RegisterVariants, add_try_method};
use strum_macros::{AsRefStr, EnumString};
use syn::Ident;

#[derive(
    AsRefStr, EnumString, RegisterVariants, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy,
)]
pub enum Tag {
    Keyword,
    Variable,
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
    Boring,
    EndOfToken,
    Expr,
    Pat,
    Const,
    Attribute,
    Item,
    MacroAsm,
    MacroIdent,
    MacroExpr,
    MacroCode,
    MacroRulesCode,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Tag {
    fn to_string(&self) -> String {
        match self {
            Self::Boring => String::from("<span class=\"boring\">"),
            Self::EndOfToken => String::from("</span>"),
            _ => format!("<span class=\"hlrs-{}\">", self.as_ref().to_lowercase()),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct SpannedToken {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) kind: Option<Tag>,
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for SpannedToken {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.start, Reverse(self.end), self.kind).partial_cmp(&(
            other.start,
            Reverse(other.end),
            other.kind,
        ))
    }
}

impl Ord for SpannedToken {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.start, Reverse(self.end), self.kind)
            .cmp(&(other.start, Reverse(other.end), other.kind))
            .then(Ordering::Greater)
    }
}
