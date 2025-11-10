use crate::tokens::Tag;
use mdbook_rust_highlight_derive::add_try_method;
use syn::{Block, Local, LocalInit, Stmt, StmtMacro};

use crate::highlighter::RustHighlighter;

impl<'a, 'ast> RustHighlighter<'a, 'ast> {
    #[add_try_method]
    pub(crate) fn register_statement(&mut self, token: &'ast Stmt) {
        match token {
            Stmt::Local(token) => {
                self.register_local(token);
            }
            Stmt::Expr(token, _) => {
                self.register_expr(token, None);
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
        self.register_macro(&token.mac);
    }

    pub(crate) fn register_block(&mut self, token: &'ast Block) {
        for statement in &token.stmts {
            self.register_statement(&statement);
        }
    }

    pub(crate) fn register_local(&mut self, token: &'ast Local) {
        self.register_keyword_tag(&token.let_token);
        self.register_pat(&token.pat, Some(Tag::Variable));
        self.try_register_local_init(token.init.as_ref());
    }

    #[add_try_method]
    pub(crate) fn register_local_init(&mut self, token: &'ast LocalInit) {
        self.register_expr(&token.expr, None);
        if let Some((else_token, expr)) = &token.diverge {
            self.register_keyword_tag(else_token);
            self.register_expr(expr, None);
        }
    }
}
