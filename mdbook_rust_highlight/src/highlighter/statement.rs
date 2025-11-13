use crate::{highlighter::Register, tokens::Tag};
use syn::{Block, Local, LocalInit, Stmt, StmtMacro};

use crate::highlighter::RustHighlighter;

impl Register for Stmt {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            Stmt::Local(token) => h.register(token),
            Stmt::Expr(token, _) => h.register(token),
            Stmt::Macro(token) => h.register(token),
            Stmt::Item(token) => h.register(token),
        }
    }
}

impl Register for StmtMacro {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.mac);
    }
}

impl Register for Block {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.stmts);
    }
}

impl Register for Local {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register_keyword_tag(&self.let_token);
        h.register_as(&self.pat, Some(Tag::Variable));
        h.register(&self.init);
    }
}

impl Register for LocalInit {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.expr);
        if let Some((else_token, expr)) = &self.diverge {
            h.register_keyword_tag(else_token);
            h.register(expr);
        }
    }
}
