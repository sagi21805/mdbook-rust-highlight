use crate::{highlighter::RustHighlighter, tokens::Tag};
use mdbook_rust_highlight_derive::add_try_method;
use syn::{Path, PathArguments, PathSegment, QSelf};

impl<'a> RustHighlighter<'a> {
    pub(crate) fn register_path_argument(&mut self, token: &PathArguments) {
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

    pub(crate) fn register_path_segment(&mut self, token: &PathSegment, tag: Option<Tag>) {
        self.register_path_argument(&token.arguments);

        if let None = tag {
            self.register_unidentified(token.into());
        } else {
            self.register_ident(&token.ident, tag);
        }
    }

    #[add_try_method]
    pub(crate) fn register_qself(&mut self, token: &QSelf) {
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
    /// If last tag is not known, put need identification
    pub(crate) fn register_path(&mut self, token: &Path, last_tag: Option<Tag>) {
        let mut segment_iter = token.segments.iter().rev();
        let last_segment = segment_iter.next();
        for segment in segment_iter {
            if segment.ident.to_string() == "Self" {
                self.register_keyword_tag(segment);
            } else {
                self.register_segment_tag(segment);
            }
        }
        if let Some(seg) = last_segment {
            if let Some(known) = self.ident_map.get(seg.ident.to_string().as_str()) {
                self.register_path_segment(seg, Some(known.clone()));
            } else {
                self.register_path_segment(seg, last_tag);
            }
        }
    }
}
