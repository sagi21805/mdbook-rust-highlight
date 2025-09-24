use syn::{ReturnType, Type, TypeImplTrait, TypePath, TypeReference, TypeTuple};

use crate::{highlighter::RustHighlighter, tokens::TokenTag};

impl<'ast> RustHighlighter<'ast> {
    pub(crate) fn register_type(&mut self, token: &'ast Type) {
        match token {
            Type::Reference(token) => {
                self.register_reference_type(token);
            }
            Type::Path(token) => {
                self.register_path_type(token);
            }
            Type::Tuple(token) => {
                self.register_tuple_type(token);
            }
            Type::ImplTrait(token) => {
                self.register_impl_trait_type(token);
            }
            _ => {}
        }
    }

    pub(crate) fn register_reference_type(&mut self, token: &'ast TypeReference) {
        self.try_register_lifetime_tag(token.lifetime.as_ref());
        self.try_register_keyword_tag(token.mutability.as_ref());
        self.register_type(&token.elem);
    }

    pub(crate) fn register_path_type(&mut self, token: &'ast TypePath) {
        self.try_register_qself(token.qself.as_ref());
        self.register_path(&token.path, Some(TokenTag::Type));
    }

    pub(crate) fn register_tuple_type(&mut self, token: &'ast TypeTuple) {
        for arg in &token.elems {
            self.register_type(arg);
        }
    }

    pub(crate) fn register_impl_trait_type(&mut self, token: &'ast TypeImplTrait) {
        self.register_keyword_tag(&token.impl_token);
        for bound in &token.bounds {
            self.register_bound(bound);
        }
    }

    pub(crate) fn register_return_type(&mut self, token: &'ast ReturnType) {
        match token {
            ReturnType::Default => {}
            ReturnType::Type(_, token) => {
                self.register_type(token);
            }
        }
    }
}
