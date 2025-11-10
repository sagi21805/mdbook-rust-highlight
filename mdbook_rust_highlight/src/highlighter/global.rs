use syn::{ItemConst, ItemStatic, StaticMutability};

use crate::highlighter::RustHighlighter;

impl<'a> RustHighlighter<'a> {
    pub(crate) fn register_static_item(&mut self, token: &ItemStatic) {
        self.register_attributes(&token.attrs);
        self.register_visibility(&token.vis);
        self.register_keyword_tag(&token.static_token);
        if let StaticMutability::Mut(mut_token) = &token.mutability {
            self.register_keyword_tag(mut_token);
        }
        self.register_variable_tag(&token.ident);
        self.register_type(&token.ty);
        self.register_expr(&token.expr, None);
    }

    pub(crate) fn register_const_item(&mut self, token: &ItemConst) {
        self.register_attributes(&token.attrs);
        self.register_visibility(&token.vis);
        self.register_keyword_tag(&token.const_token);
        self.register_const_tag(&token.ident);
        self.register_generics(&token.generics);
        self.register_type(&token.ty);
        self.register_expr(&token.expr, None);
    }
}
