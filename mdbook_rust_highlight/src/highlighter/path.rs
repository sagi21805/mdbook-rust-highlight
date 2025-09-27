use mdbook_rust_highlight_derive::add_try_method;
use syn::{Path, PathArguments, PathSegment, QSelf, spanned::Spanned};

use crate::{highlighter::RustHighlighter, tokens::TokenTag};

impl<'ast> RustHighlighter<'ast> {
    pub(crate) fn register_path_argument(&mut self, token: &'ast PathArguments) {
        match token {
            PathArguments::Parenthesized(token) => {
                self.register_parenthesized_arg(token);
            }
            PathArguments::AngleBracketed(token) => {
                self.register_angle_brackets_arg(token);
            }
            PathArguments::None => {}
        }
    }

    pub(crate) fn register_path_segment(&mut self, token: &'ast PathSegment, tag: TokenTag) {
        self.register_tag(&token.ident, tag);
        self.register_path_argument(&token.arguments);
    }

    #[add_try_method]
    pub(crate) fn register_qself(&mut self, token: &'ast QSelf) {
        self.register_type(&token.ty);
        self.try_register_keyword_tag(token.as_token.as_ref());
    }

    /// Register a path token
    ///
    /// # Parameters
    ///
    /// - `token:` - The path segment
    /// - `last:` - Optional tag to put for the last item of the path.
    ///
    /// TODO ADD DOCUMENTATION AND PLAN WHAT HAPPENS IF NONE IS GIVEN
    pub(crate) fn register_path(&mut self, token: &'ast Path, last: Option<TokenTag>) {
        let mut segment_iter = token.segments.iter().rev();
        let last_segment = segment_iter.next();
        for segment in &token.segments {
            self.register_segment_tag(segment);
        }
        match last_segment {
            Some(segment) => match last {
                Some(tag) => self.register_path_segment(segment, tag),
                None => {
                    self.register_path_segment(segment, TokenTag::NeedIdentification);
                    self.unidentified
                        .insert(segment.span().byte_range().start, &segment);
                }
            },
            None => {}
        }
    }
}
