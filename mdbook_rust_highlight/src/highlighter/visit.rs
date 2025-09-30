use syn::{Item, visit::Visit};

use crate::highlighter::RustHighlighter;

impl<'a, 'ast> Visit<'ast> for RustHighlighter<'a, 'ast> {
    // AT THE END USE ONLY THIS FUNCTION
    fn visit_item(&mut self, i: &'ast Item) {
        self.register_item(i);
    }
}
