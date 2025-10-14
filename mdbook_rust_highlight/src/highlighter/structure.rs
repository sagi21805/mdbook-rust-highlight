use syn::{Field, FieldValue, Fields, Member};

use crate::highlighter::RustHighlighter;


impl<'a, 'ast> RustHighlighter<'a, 'ast> {

    pub(crate) fn register_struct_fields(&mut self, token: &'ast Fields) {
        match token {
            Fields::Unit => {},
            Fields::Named(token) => {
                for token in &token.named {
                    self.register_field(token);
                }
            }
            Fields::Unnamed(token) => {
                for token in &token.unnamed {
                    self.register_field(token);
                }
            }
        }

    }

    pub(crate) fn register_field(&mut self, token: &'ast Field) {
        self.register_visibility(&token.vis);
        self.try_register_ident_tag(token.ident.as_ref());
        self.register_type(&token.ty); 
    }

    pub(crate) fn register_field_value(&mut self, token: &'ast FieldValue) {
        match &token.member {
            Member::Named(token) => {
                self.register_ident_tag(token);
            }
            Member::Unnamed(_) => {}
        }
        self.register_expr(&token.expr);
    }
    
}