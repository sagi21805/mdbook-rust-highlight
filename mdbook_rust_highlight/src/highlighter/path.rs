use mdbook_rust_highlight_derive::add_try_method;
use syn::{Path, PathArguments, PathSegment, QSelf, spanned::Spanned};

use crate::{highlighter::RustHighlighter, tokens::TokenTag};

impl<'a, 'ast> RustHighlighter<'a, 'ast> {
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

    pub(crate) fn register_path_segment(
        &mut self,
        token: &'ast PathSegment,
        tag: Option<TokenTag>,
    ) {
        self.register_path_argument(&token.arguments);
        match tag {
            None => {
                self.register_tag(&token.ident, TokenTag::NeedIdentification);
                self.unidentified
                    .insert(token.span().byte_range().start, &token);
            }
            Some(tag) => self.register_ident(&token.ident, tag),
        }
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
    pub(crate) fn register_path(&mut self, token: &'ast Path, last_tag: Option<TokenTag>) {
        let mut segment_iter = token.segments.iter().rev();
        let last_segment = segment_iter.next();
        for segment in segment_iter {
            self.register_segment_tag(segment);
        }
        if let Some(seg) = last_segment {
            self.register_path_segment(seg, last_tag);
        }
    }
}
