use syn::{Item, visit::Visit};

use crate::highlighter::RustHighlighter;

impl<'ast> Visit<'ast> for RustHighlighter {
    fn visit_item(&mut self, i: &Item) {
        self.register(i);
    }

    fn visit_attribute(&mut self, i: &syn::Attribute) {
        self.register(i);
    }
}
