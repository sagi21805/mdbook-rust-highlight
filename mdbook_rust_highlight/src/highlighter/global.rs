use syn::{ItemStatic, StaticMutability};

use crate::highlighter::RustHighlighter;



impl<'a, 'ast> RustHighlighter<'a, 'ast> {

    pub(crate) fn register_static_item(&mut self, token: &'ast ItemStatic) {
        self.register_attributes(&token.attrs);
        self.register_visibility(&token.vis);
        self.register_keyword_tag(&token.static_token);
        if let StaticMutability::Mut(mut_token) = &token.mutability {
            self.register_keyword_tag(mut_token);
        }
        self.register_ident_tag(&token.ident);
        self.register_type(&token.ty);
        self.register_expr(&token.expr);   
    } 

}