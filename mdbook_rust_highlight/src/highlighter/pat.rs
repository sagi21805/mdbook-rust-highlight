use syn::{Pat, PatIdent, PatOr, PatReference, PatTuple, PatTupleStruct, PatType};

use crate::{highlighter::RustHighlighter, tokens::TokenTag};

impl<'ast> RustHighlighter<'ast> {
    pub(crate) fn register_pat(&mut self, token: &'ast Pat) {
        match token {
            Pat::Ident(token) => {
                self.register_ident_pat(token);
            }
            Pat::Reference(token) => {
                self.register_reference_pat(token);
            }
            Pat::Type(token) => {
                self.register_type_pat(token);
            }
            Pat::Path(token) => {
                self.register_path_expr(token);
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
            _ => {
                self.register_ident_tag(token);
            }
        }
    }

    pub(crate) fn register_ident_pat(&mut self, token: &'ast PatIdent) {
        self.try_register_keyword_tag(token.by_ref.as_ref());
        self.try_register_keyword_tag(token.mutability.as_ref());
        self.register_ident_tag(&token.ident);
    }

    pub(crate) fn register_reference_pat(&mut self, token: &'ast PatReference) {
        self.try_register_keyword_tag(token.mutability.as_ref());
        self.register_pat(&token.pat);
    }

    pub(crate) fn register_type_pattern(&mut self, token: &'ast PatType) {
        self.register_pat(&token.pat);
    }

    pub(crate) fn register_type_pat(&mut self, token: &'ast PatType) {
        self.register_pat(&token.pat);
        self.register_type(&token.ty);
    }

    pub(crate) fn register_tuple_pat(&mut self, token: &'ast PatTuple) {
        for arg in &token.elems {
            self.register_pat(arg);
        }
    }

    pub(crate) fn register_tuple_struct_pat(&mut self, token: &'ast PatTupleStruct) {
        self.try_register_qself(token.qself.as_ref());
        self.register_path(&token.path, Some(TokenTag::Enum));
        for arg in &token.elems {
            self.register_pat(arg);
        }
    }

    pub(crate) fn register_or_pat(&mut self, token: &'ast PatOr) {
        for case in &token.cases {
            self.register_pat(case);
        }
    }
}
