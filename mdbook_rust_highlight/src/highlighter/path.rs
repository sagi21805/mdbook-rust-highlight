use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};
use syn::{Path, PathArguments, PathSegment, QSelf, punctuated::Punctuated};

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
            h.register_ident(&self.ident, None);
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
            if segment.ident == "Self" {
                h.register_keyword_tag(segment);
            } else {
                h.register_segment_tag(segment);
            }
        }

        if let Some(seg) = last_segment {
            if let Some(tag) = h.tracer.manual.get(seg.ident.to_string().as_str()) {
                h.register_as(seg, Some(*tag));
                return;
            }
            if let Some(tag) = _tag {
                h.register_as(seg, Some(tag));
                h.tracer.map(self, tag);
                return;
            }
            if let Some(tag) = h.tracer.get(self) {
                h.register_as(seg, Some(tag));
                return;
            }
            let path = Path {
                leading_colon: None,
                segments: Punctuated::from_iter(vec![seg.clone()]),
            };
            h.register_as(seg, h.tracer.get(&path));
        }
    }
}
