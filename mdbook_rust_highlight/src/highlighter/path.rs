use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};
use syn::{Path, PathArguments, PathSegment, QSelf};

impl Register for PathArguments {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            PathArguments::Parenthesized(token) => h.register_as(token, _tag),
            PathArguments::AngleBracketed(token) => h.register_as(token, _tag),
            PathArguments::None => {}
        }
    }
}

impl Register for PathSegment {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.arguments);

        if let Some(tag) = _tag {
            h.register_ident(&self.ident, Some(tag));
        } else {
            h.register_unidentified_ident(&self.ident);
        }
    }
}

impl Register for QSelf {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.ty);
        h.try_register_keyword_tag(self.as_token.as_ref());
    }
}

impl Register for Path {
    /// Register a path token
    ///
    /// # Parameters
    ///
    /// - `token:` - The path segment
    /// - `last:` - Optional tag to put for the last item of the path.
    ///
    /// If last tag is not known, put need identification
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        let mut segment_iter = self.segments.iter().rev();
        let last_segment = segment_iter.next();
        for segment in segment_iter {
            if segment.ident.to_string() == "Self" {
                h.register_keyword_tag(segment);
            } else {
                h.register_segment_tag(segment);
            }
        }

        let known = h.tracer.get(self);
        if let (Some(tag), Some(seg)) = (known, last_segment) {
            h.register_as(seg, Some(tag));
        }
    }
}
