use syn::{Item, visit::Visit};

use crate::highlighter::RustHighlighter;

impl<'a> Visit for RustHighlighter<'a> {
    fn visit_item(&mut self, i: &Item) {
        self.register_item(i);
    }

    fn visit_attribute(&mut self, i: &syn::Attribute) {
        self.register_attribute(i);
    }
}
