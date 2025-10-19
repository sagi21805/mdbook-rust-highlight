use std::cmp::{Ordering, Reverse};

use crate::highlighter::RustHighlighter;
use mdbook_rust_highlight_derive::{RegisterVariants, add_try_method};
use strum_macros::AsRefStr;
use syn::{Ident, PathSegment, spanned::Spanned};

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
    Boring,
    EndOfToken,
    Expr,
    Pat,
    Const,
    Attribute,
    Item,
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
    pub(crate) kind: Option<TokenTag>,
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

#[derive(Clone)]
pub enum PathToken<'ast> {
    Segment(&'ast PathSegment),
    Ident(&'ast Ident),
}

impl<'ast> PathToken<'ast> {
    pub(crate) fn ident(&self) -> &'ast Ident {
        match self {
            Self::Ident(token) => token,
            Self::Segment(token) => &token.ident,
        }
    }

    pub(crate) fn span(&self) -> proc_macro2::Span {
        match self {
            Self::Ident(token) => token.span(),
            Self::Segment(token) => token.span(),
        }
    }
}

impl<'ast> From<&'ast PathSegment> for PathToken<'ast> {
    fn from(value: &'ast PathSegment) -> Self {
        PathToken::Segment(value)
    }
}

impl<'ast> From<&'ast Ident> for PathToken<'ast> {
    fn from(value: &'ast Ident) -> Self {
        PathToken::Ident(value)
    }
}
