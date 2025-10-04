use syn::Attribute;

use crate::highlighter::RustHighlighter;

impl<'a, 'ast> RustHighlighter<'a, 'ast> {
    pub(crate) fn register_attribute(&mut self, token: &'ast Attribute) {}
}
