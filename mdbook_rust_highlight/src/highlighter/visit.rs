use syn::{FnArg, visit::Visit};

use crate::{highlighter::RustHighlighter, tokens::TokenTag};

impl<'ast> Visit<'ast> for RustHighlighter<'ast> {
    fn visit_signature(&mut self, i: &'ast syn::Signature) {
        self.try_register_keyword_tag(i.constness.as_ref());
        self.try_register_keyword_tag(i.asyncness.as_ref());
        self.try_register_keyword_tag(i.unsafety.as_ref());
        if let Some(abi) = &i.abi {
            self.register_keyword_tag(&abi.extern_token);
            self.try_register_litstr_tag(abi.name.as_ref());
        }
        self.register_keyword_tag(&i.fn_token);
        self.register_token(&i.ident, TokenTag::Function);
        for input in &i.inputs {
            match input {
                FnArg::Receiver(arg) => {
                    self.register_token(&arg.self_token, TokenTag::SelfToken);
                    self.try_register_keyword_tag(arg.mutability.as_ref());
                    self.try_register_lifetime_tag(arg.lifetime());
                }
                FnArg::Typed(type_pat) => {
                    self.register_type_pattern(type_pat);
                    self.register_type(&type_pat.ty);
                }
            }
        }

        self.register_return_type(&i.output);
    }

    fn visit_abi(&mut self, i: &'ast syn::Abi) {
        self.register_keyword_tag(&i.extern_token);
        self.try_register_litstr_tag(i.name.as_ref());
    }

    fn visit_expr_async(&mut self, i: &'ast syn::ExprAsync) {
        self.register_keyword_tag(&i.async_token);
    }

    fn visit_visibility(&mut self, i: &'ast syn::Visibility) {
        self.register_keyword_tag(i);
    }

    fn visit_block(&mut self, i: &'ast syn::Block) {
        self.register_block(i);
    }

    // pub(crate) visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {}

    // pub(crate) visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {}
}
