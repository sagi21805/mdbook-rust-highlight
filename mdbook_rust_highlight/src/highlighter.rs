use std::collections::BTreeMap;

use crate::tokens::TokenTag;
use proc_macro2::{TokenStream, TokenTree};
use ropey::Rope;
use syn::{
    Block, Expr, ExprForLoop, ExprLit, ExprUnsafe, File, FnArg, Item, Lit, LitStr, Local,
    LocalInit, Pat, PatIdent, PatReference, PatType, Stmt, StmtMacro, spanned::Spanned,
    visit::Visit,
};

use mdbook_rust_highlight_derive::make_register_wrappers;

pub struct RustHighlighter {
    output: Rope,
    token_map: BTreeMap<usize, TokenTag>,
}

impl<'ast> Visit<'ast> for RustHighlighter {
    fn visit_signature(&mut self, i: &'ast syn::Signature) {
        self.try_register_keyword(i.constness.as_ref());
        self.try_register_keyword(i.asyncness.as_ref());
        self.try_register_keyword(i.unsafety.as_ref());
        if let Some(abi) = &i.abi {
            self.register_keyword(&abi.extern_token);
            self.try_register_litstr(abi.name.as_ref());
        }
        self.register_keyword(&i.fn_token);
        self.register_token(&i.ident, TokenTag::Function);
        for input in &i.inputs {
            match input {
                FnArg::Receiver(arg) => {
                    self.register_token(&arg.self_token, TokenTag::SelfToken);
                    self.try_register_lifetime(arg.lifetime());
                }
                FnArg::Typed(type_pat) => {
                    self.register_type_pattern(type_pat);
                    self.register_token(&type_pat.ty, TokenTag::Type);
                }
            }
        }

        // TODO: Make right for all return types, some are more complicated
        match i.output.clone() {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, t) => self.register_token(&t, TokenTag::Type),
        }
    }

    fn visit_abi(&mut self, i: &'ast syn::Abi) {
        self.register_keyword(&i.extern_token);
        self.try_register_litstr(i.name.as_ref());
    }

    fn visit_expr_async(&mut self, i: &'ast syn::ExprAsync) {
        self.register_keyword(&i.async_token);
    }

    fn visit_visibility(&mut self, i: &'ast syn::Visibility) {
        self.register_keyword(i);
    }

    fn visit_block(&mut self, i: &'ast syn::Block) {
        self.register_block(i);
    }
}

impl RustHighlighter {
    fn write_tokens(&mut self) {
        let mut tok_offset: usize = 0;
        for (index, token) in &self.token_map {
            eprintln!("index: {}, token: {:?}", index, token as *const _ as u8);
            let tag = token.to_string();
            self.output.insert(index + tok_offset, tag.as_str());
            tok_offset += tag.len();
        }
    }

    /// Extract a span position in the rope.
    ///
    /// returns the (start_idx, end_idx) of the span
    ///
    /// TODO: assuming same line, create tests to assert this assumption
    fn span_position(&self, span: &impl Spanned) -> (usize, usize) {
        // lines are 1 indexed instead of zero.
        let span = span.span().byte_range();
        (span.start, span.end)
    }

    pub(crate) fn register_token(&mut self, token: &impl Spanned, tag: TokenTag) {
        let (start_idx, end_idx) = self.span_position(token);
        eprintln!("REGISTERD: s: {}, e: {}", start_idx, end_idx);
        self.token_map.insert(start_idx, tag);
        self.token_map.insert(end_idx, TokenTag::EndOfToken);
    }

    #[make_register_wrappers]
    fn register_statement(&mut self, token: &Stmt) {
        match token {
            syn::Stmt::Local(token) => {
                self.register_local(token);
            }
            syn::Stmt::Expr(token, _) => {
                self.register_expr(token);
            }
            syn::Stmt::Macro(token) => {
                self.register_macro_statement(token);
            }
            syn::Stmt::Item(i) => {}
        }
    }

    #[make_register_wrappers]
    fn register_local(&mut self, token: &Local) {
        self.register_keyword(&token.let_token);
        self.register_pattern(&token.pat);
        self.try_register_local_init(token.init.as_ref());
    }

    fn register_literal(&mut self, token: &ExprLit) {
        match &token.lit {
            Lit::Int(_) | Lit::Float(_) => {
                self.register_litnum(&token.lit);
            }
            Lit::Bool(_) => {
                self.register_litbool(&token.lit);
            }
            Lit::Byte(_) | Lit::ByteStr(_) | Lit::CStr(_) | Lit::Char(_) | Lit::Str(_) => {
                self.register_litstr(&token.lit)
            }
            _ => {}
        }
    }

    #[make_register_wrappers]
    fn register_block(&mut self, token: &Block) {
        for statement in &token.stmts {
            self.register_statement(&statement);
        }
    }

    #[make_register_wrappers]
    fn register_for_loop(&mut self, token: &ExprForLoop) {
        self.register_keyword(&token.for_token);
        self.register_pattern(&token.pat);
        self.register_keyword(&token.in_token);
        self.register_expr_box(&token.expr);
        self.register_block(&token.body);
    }

    #[make_register_wrappers]
    fn register_expr(&mut self, token: &Expr) {
        match token {
            Expr::Lit(token) => {
                self.register_literal(token);
            }
            Expr::ForLoop(token) => {
                self.register_for_loop(token);
            }
            Expr::Unsafe(token) => {
                self.register_unsafe_expr(token);
            }
            _ => self.register_ident(&token),
        }
    }

    fn register_unsafe_expr(&mut self, token: &ExprUnsafe) {
        self.register_keyword(&token.unsafe_token);
        self.register_block(&token.block);
    }

    fn register_macro_statement(&mut self, token: &StmtMacro) {
        // TODO NEED CHANGE TO RENDER PATH CORRECTLY AND TO PARSE TOKEN TREE BETTER WITH SPECIFIC KEY WORD FOR BUILTIN MACROS
        self.register_macro(&token.mac.path);
        // self.register_macro(&token.mac.bang_token);
        for token in token.mac.tokens.clone() {
            if let TokenTree::Literal(lit) = token {
                if let Ok(_) = syn::parse_str::<LitStr>(&lit.to_string()) {
                    self.register_litstr(&lit);
                }
            }
        }
    }

    fn register_item(&mut self, token: &Item) {}

    #[make_register_wrappers]
    fn register_pattern(&mut self, token: &Pat) {
        match token {
            Pat::Ident(i) => {
                self.register_pat_ident(i);
            }
            Pat::Reference(r) => {
                self.register_reference(r);
            }
            _ => {}
        }
    }

    #[make_register_wrappers]
    fn register_reference(&mut self, token: &PatReference) {
        self.try_register_keyword(token.mutability.as_ref());
        self.register_pattern_box(&token.pat);
    }

    #[make_register_wrappers]
    fn register_local_init(&mut self, token: &LocalInit) {
        self.register_expr_box(&token.expr);
    }

    #[make_register_wrappers]
    fn register_pat_ident(&mut self, token: &PatIdent) {
        self.try_register_keyword(token.by_ref.as_ref());
        self.try_register_keyword(token.mutability.as_ref());
        self.register_token(&token.ident, TokenTag::Ident);
    }

    #[make_register_wrappers]
    fn register_type_pattern(&mut self, token: &PatType) {
        self.register_pattern_box(&token.pat);
    }

    pub fn highlight(code: &str) -> String {
        let syntax_tree: File =
            syn::parse_str(code).expect(&format!("Failed to parse Rust code {}", code));

        let mut highlighter = RustHighlighter {
            output: Rope::from_str(code),
            token_map: BTreeMap::new(),
        };

        highlighter.visit_file(&syntax_tree);
        highlighter.write_tokens();
        highlighter.output.to_string()
    }
}
