use syn::Item;

use crate::highlighter::RustHighlighter;

impl<'ast> RustHighlighter<'ast> {
    pub(crate) fn register_item(&mut self, token: &Item) {}
}
