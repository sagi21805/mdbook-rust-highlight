use mdbook_rust_highlight_derive::add_try_method;
use proc_macro2::TokenTree;
use syn::{Block, LitStr, Local, LocalInit, Stmt, StmtMacro};

use crate::highlighter::RustHighlighter;

impl<'a, 'ast> RustHighlighter<'a, 'ast> {
    #[add_try_method]
    pub(crate) fn register_statement(&mut self, token: &'ast Stmt) {
        match token {
            Stmt::Local(token) => {
                self.register_local(token);
            }
            Stmt::Expr(token, _) => {
                self.register_expr(token);
            }
            Stmt::Macro(token) => {
                self.register_macro_statement(token);
            }
            Stmt::Item(token) => {
                self.register_item(token);
            }
        }
    }

    pub(crate) fn register_macro_statement(&mut self, token: &'ast StmtMacro) {
        // TODO NEED CHANGE TO RENDER PATH CORRECTLY AND TO PARSE TOKEN TREE BETTER WITH SPECIFIC KEY WORD FOR BUILTIN MACROS
        self.register_macro_tag(&token.mac.path);
        self.register_macro_tag(&token.mac.bang_token);
        for token in token.mac.tokens.clone() {
            if let TokenTree::Literal(lit) = token {
                if let Ok(_) = syn::parse_str::<LitStr>(&lit.to_string()) {
                    self.register_litstr_tag(&lit);
                }
            }
        }
    }

    pub(crate) fn register_block(&mut self, token: &'ast Block) {
        for statement in &token.stmts {
            self.register_statement(&statement);
        }
    }

    pub(crate) fn register_local(&mut self, token: &'ast Local) {
        self.register_keyword_tag(&token.let_token);
        self.register_pat(&token.pat);
        self.try_register_local_init(token.init.as_ref());
    }

    #[add_try_method]
    pub(crate) fn register_local_init(&mut self, token: &'ast LocalInit) {
        self.register_expr(&token.expr);
    }
}
