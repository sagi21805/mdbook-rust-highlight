use syn::{Pat, PatIdent, PatOr, PatReference, PatTuple, PatTupleStruct, PatType};

use crate::{highlighter::RustHighlighter, tokens::Tag};

impl<'a> RustHighlighter<'a> {
    pub(crate) fn register_pat(&mut self, token: &Pat, identifier: Option<Tag>) {
        match token {
            Pat::Ident(token) => {
                self.register_ident_pat(token);
            }
            Pat::Reference(token) => {
                self.register_reference_pat(token, identifier);
            }
            Pat::Type(token) => {
                self.register_type_pat(token);
            }
            Pat::Path(token) => {
                self.register_path_expr(token, identifier);
            }
            Pat::Tuple(token) => {
                self.register_tuple_pat(token);
            }
            Pat::TupleStruct(token) => {
                self.register_tuple_struct_pat(token);
            }
            Pat::Or(token) => {
                self.register_or_pat(token);
            }
            Pat::Lit(token) => {
                self.register_lit_expr(token);
            }
            Pat::Const(token) => {
                self.register_attributes(&token.attrs);
                self.register_keyword_tag(&token.const_token);
                self.register_block(&token.block);
            }
            _ => {}
        }
    }

    pub(crate) fn register_ident_pat(&mut self, token: &PatIdent) {
        self.try_register_keyword_tag(token.by_ref.as_ref());
        self.try_register_keyword_tag(token.mutability.as_ref());
        self.register_variable_tag(&token.ident);
    }

    pub(crate) fn register_reference_pat(&mut self, token: &PatReference, identifier: Option<Tag>) {
        self.try_register_keyword_tag(token.mutability.as_ref());
        self.register_pat(&token.pat, identifier);
    }

    pub(crate) fn register_type_pat(&mut self, token: &PatType) {
        self.register_pat(&token.pat, Some(Tag::Variable));
        self.register_type(&token.ty);
    }

    pub(crate) fn register_tuple_pat(&mut self, token: &PatTuple) {
        for arg in &token.elems {
            self.register_pat(arg, None);
        }
    }

    pub(crate) fn register_tuple_struct_pat(&mut self, token: &PatTupleStruct) {
        self.try_register_qself(token.qself.as_ref());
        self.register_path(&token.path, Some(Tag::Enum));
        for arg in &token.elems {
            self.register_pat(arg, None);
        }
    }

    pub(crate) fn register_or_pat(&mut self, token: &PatOr) {
        for case in &token.cases {
            self.register_pat(case, None);
        }
    }
}
