use syn::{
    FnArg, Item, ItemEnum, ItemFn, ItemUse, PathSegment, UseTree, Visibility,
    token::{self, Token},
};

use crate::{highlighter::RustHighlighter, tokens::TokenTag};

impl<'a, 'ast> RustHighlighter<'a, 'ast> {
    pub(crate) fn register_item(&mut self, token: &'ast Item) {
        match token {
            Item::Fn(token) => {
                self.register_function_item(token);
            }
            Item::Enum(token) => {
                self.register_enum_item(token);
            }
            Item::Use(token) => {
                self.register_use_item(token);
            }
            _ => {}
        }
    }

    pub(crate) fn register_function_item(&mut self, token: &'ast ItemFn) {
        self.register_visibility(&token.vis);
        self.try_register_keyword_tag(token.sig.constness.as_ref());
        self.try_register_keyword_tag(token.sig.asyncness.as_ref());
        self.try_register_keyword_tag(token.sig.unsafety.as_ref());
        if let Some(abi) = &token.sig.abi {
            self.register_keyword_tag(&abi.extern_token);
            self.try_register_litstr_tag(abi.name.as_ref());
        }
        self.register_keyword_tag(&token.sig.fn_token);
        self.register_function_tag(&token.sig.ident);

        for input in &token.sig.inputs {
            match input {
                FnArg::Receiver(arg) => {
                    self.register_selftoken_tag(&arg.self_token);
                    self.try_register_keyword_tag(arg.mutability.as_ref());
                    self.try_register_lifetime_tag(arg.lifetime());
                }
                FnArg::Typed(type_pat) => {
                    self.register_type_pattern(type_pat);
                    self.register_type(&type_pat.ty);
                }
            }
        }

        self.register_return_type(&token.sig.output);
        self.register_block(&token.block);
    }

    pub(crate) fn register_enum_item(&mut self, token: &'ast ItemEnum) {
        self.register_visibility(&token.vis);
        self.register_keyword_tag(&token.enum_token);
        self.register_tag(&token.ident, TokenTag::Type);
        // TODO REGISTER GENERICS AND FIELDS
        for variant in &token.variants {
            self.register_enum_tag(&variant.ident);
            if let Some((_, discriminant)) = &variant.discriminant {
                self.register_expr(discriminant);
            }
        }
    }

    pub(crate) fn register_use_item(&mut self, token: &'ast ItemUse) {
        self.register_visibility(&token.vis);
        self.register_keyword_tag(&token.use_token);
        self.register_use_tree(&token.tree);
    }

    pub(crate) fn register_use_tree(&mut self, token: &'ast UseTree) {
        match token {
            UseTree::Glob(_) => {}
            UseTree::Group(token) => {
                for tree in &token.items {
                    self.register_use_tree(tree);
                }
            }
            UseTree::Path(token) => {
                self.register_segment_tag(&token.ident);
                self.register_use_tree(&token.tree);
            }
            UseTree::Name(token) => {
                self.register_path_segment(, None);
            }
            UseTree::Rename(token) => {
                self.register_segment_tag(&token.ident);
                self.register_keyword_tag(&token.as_token);
                self.register_segment_tag(&token.rename);
            }
        }
    }

    pub(crate) fn register_visibility(&mut self, token: &'ast Visibility) {
        match token {
            Visibility::Inherited => {}
            _ => self.register_keyword_tag(token),
        }
    }
}
