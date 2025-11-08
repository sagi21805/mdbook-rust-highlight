use proc_macro2::{TokenStream, TokenTree};
use syn::{
    FnArg, ImplItem, Item, ItemEnum, ItemFn, ItemImpl, ItemMacro, ItemStruct, ItemUse, LitStr,
    Macro, Signature, UseTree, Visibility,
};

use crate::{
    highlighter::{
        RustHighlighter,
        macro_parsers::asm::{AsmArgs, AsmInput, AsmOptions, ExprOperand, RegOperand, RegSpec},
    },
    tokens::TokenTag,
};

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
            Item::Macro(token) => {
                self.register_macro_item(token);
            }
            Item::Impl(token) => {
                self.register_impl_item(token);
            }
            Item::Struct(token) => {
                self.register_struct_item(token);
            }
            Item::Static(token) => {
                self.register_static_item(token);
            }
            Item::Const(token) => {
                self.register_const_item(token);
            }
            _ => {}
        }
    }

    pub(crate) fn register_struct_item(&mut self, token: &'ast ItemStruct) {
        self.register_attributes(&token.attrs);
        self.register_visibility(&token.vis);
        self.register_keyword_tag(&token.struct_token);
        self.register_type_tag(&token.ident);
        self.register_struct_fields(&token.fields);
    }

    pub(crate) fn register_function_item(&mut self, token: &'ast ItemFn) {
        self.register_attributes(&token.attrs);
        self.register_visibility(&token.vis);
        self.register_function_sig(&token.sig);
        self.register_block(&token.block);
    }

    pub(crate) fn register_function_sig(&mut self, token: &'ast Signature) {
        self.try_register_keyword_tag(token.constness.as_ref());
        self.try_register_keyword_tag(token.asyncness.as_ref());
        self.try_register_keyword_tag(token.unsafety.as_ref());
        if let Some(abi) = &token.abi {
            self.register_keyword_tag(&abi.extern_token);
            self.try_register_litstr_tag(abi.name.as_ref());
        }
        self.register_keyword_tag(&token.fn_token);
        self.register_function_tag(&token.ident);

        for input in &token.inputs {
            match input {
                FnArg::Receiver(arg) => {
                    self.register_selftoken_tag(&arg.self_token);
                    self.try_register_keyword_tag(arg.mutability.as_ref());
                    self.try_register_lifetime_tag(arg.lifetime());
                }
                FnArg::Typed(type_pat) => {
                    self.register_type_pat(type_pat);
                    self.register_type(&type_pat.ty);
                }
            }
        }

        self.register_return_type(&token.output);
    }

    pub(crate) fn register_enum_item(&mut self, token: &'ast ItemEnum) {
        self.register_attributes(&token.attrs);
        self.register_visibility(&token.vis);
        self.register_keyword_tag(&token.enum_token);
        self.register_tag(&token.ident, Some(TokenTag::Type));
        // TODO REGISTER GENERICS AND FIELDS
        for variant in &token.variants {
            self.register_enum_tag(&variant.ident);
            if let Some((_, discriminant)) = &variant.discriminant {
                self.register_expr(discriminant, None);
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
                self.register_unidentified((&token.ident).into());
            }
            UseTree::Rename(token) => {
                self.register_segment_tag(&token.ident);
                self.register_keyword_tag(&token.as_token);
                self.register_segment_tag(&token.rename);
            }
        }
    }

    pub(crate) fn register_macro_item(&mut self, token: &'ast ItemMacro) {
        if let Some(name) = token.ident.as_ref() {
            self.register_ident(name.into(), Some(TokenTag::Macro));
        }
        self.register_macro(&token.mac);
    }

    pub(crate) fn register_macro(&mut self, token: &'ast Macro) {
        let mut tag = TokenTag::Macro;
        eprintln!("In regular macro");
        if let Some(ident) = token.path.get_ident() {
            match ident.to_string().as_str() {
                "macro_rules" => tag = TokenTag::Keyword,
                "asm" => {
                    eprintln!("{:?}", self.register_asm_macro(token.tokens.clone()));
                }
                _ => {}
            };
        };
        self.register_path(&token.path, Some(tag));
        self.register_macro_tag(&token.bang_token);
    }

    pub(crate) fn register_asm_macro(&mut self, token: TokenStream) -> syn::Result<()> {
        let args = syn::parse2::<AsmArgs>(token)?;
        for arg in &args.args {
            match arg {
                AsmInput::Instruction(instruction) => {
                    self.register_litstr_tag(instruction);
                }
                AsmInput::Options(options) => {
                    self.register_asm_options(options);
                }
                AsmInput::RegOperand(reg_operand) => {
                    self.register_asm_reg_operand(reg_operand);
                }
            }
        }
        Ok(())
    }

    pub(crate) fn register_asm_options(&mut self, token: &AsmOptions) {
        self.register_keyword_tag(&token.option_token);
        for option in &token.options.idents {
            self.register_keyword_tag(option);
        }
    }

    pub(crate) fn register_asm_reg_operand(&mut self, token: &RegOperand) {
        if let Some((param, _)) = &token.param_eq {
            self.register_ident_tag(param);
        }
        self.register_asm_expr_operand(&token.expr);
    }

    pub(crate) fn register_asm_expr_operand(&mut self, token: &ExprOperand) {
        match token {
            ExprOperand::Constant(constant) => {
                self.register_keyword_tag(&constant.const_token);
                // self.register_expr(&constant.expr, None);
            }
            ExprOperand::Label(label) => {
                self.register_keyword_tag(&label.label_token);
            }
            ExprOperand::Operand(op) => {
                self.register_keyword_tag(&op.directive);
                self.register_reg_spec(&op.reg);
            }
            ExprOperand::Symbol(sym) => {}
        }
    }

    pub(crate) fn register_reg_spec(&mut self, token: &RegSpec) {
        match token {
            RegSpec::ExplicitReg(s) => {
                self.register_litstr_tag(s);
            }
            RegSpec::RegClass(i) => {
                self.register_ident_tag(i);
            }
        }
    }

    // TODO COMPLETE
    pub(crate) fn register_impl_item(&mut self, token: &'ast ItemImpl) {
        self.try_register_keyword_tag(token.unsafety.as_ref());
        self.register_keyword_tag(&token.impl_token);
        if let Some((_, trait_name, for_token)) = &token.trait_ {
            self.register_path(trait_name, Some(TokenTag::Type));
            self.register_keyword_tag(for_token);
        }
        self.register_type(&token.self_ty);
        for item in &token.items {
            self.register_item_impl(item)
        }
    }

    pub(crate) fn register_item_impl(&mut self, token: &'ast ImplItem) {
        match token {
            ImplItem::Const(token) => {}
            ImplItem::Fn(token) => {
                self.register_attributes(&token.attrs);
                self.register_visibility(&token.vis);
                self.register_function_sig(&token.sig);
                self.register_block(&token.block);
            }
            ImplItem::Macro(token) => {
                self.register_macro(&token.mac);
            }
            ImplItem::Type(token) => {}
            ImplItem::Verbatim(_) => {}
            _ => {}
        }
    }

    pub(crate) fn register_visibility(&mut self, token: &'ast Visibility) {
        match token {
            Visibility::Inherited => {}
            _ => self.register_keyword_tag(token),
        }
    }
}
