use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};
use syn::{Path, PathArguments, PathSegment, QSelf};

impl Register for PathArguments {
    fn register_as(&self, h: &mut RustHighlighter, tag: Option<Tag>) {
        match self {
            PathArguments::Parenthesized(token) | PathArguments::AngleBracketed(token) => {
                h.register(token);
            }
            PathArguments::None => {}
        }
    }
}

impl Register for PathSegment {
    fn register_as(&self, h: &mut RustHighlighter, tag: Option<Tag>) {
        h.register(&self.arguments);

        if let None = tag {
            h.register_unidentified(self.into());
        } else {
            h.register_ident(&self.ident, tag);
        }
    }
}

impl Register for QSelf {
    fn register_as(&self, h: &mut RustHighlighter, tag: Option<Tag>) {
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
    fn register_as(&self, h: &mut RustHighlighter, tag: Option<Tag>) {
        let mut segment_iter = self.segments.iter().rev();
        let last_segment = segment_iter.next();
        for segment in segment_iter {
            if segment.ident.to_string() == "Self" {
                h.register_keyword_tag(segment);
            } else {
                h.register_segment_tag(segment);
            }
        }
        if let Some(seg) = last_segment {
            if let Some(known) = h.ident_map.get(seg.ident.to_string().as_str()) {
                h.register_as(seg, Some(known.clone()));
            } else {
                h.register_as(seg, tag);
            }
        }
    }
}
