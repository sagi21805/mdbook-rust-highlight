use mdbook_rust_highlight_derive::add_try_method;
use syn::{
    Arm, Expr, ExprBinary, ExprBlock, ExprCall, ExprCast, ExprField, ExprForLoop, ExprIf, ExprLit,
    ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprReference, ExprTry, ExprTuple, ExprUnary,
    ExprUnsafe, Lit, Member,
};

use crate::highlighter::RustHighlighter;

impl<'ast> RustHighlighter<'ast> {
    #[add_try_method]
    pub(crate) fn register_expr(&mut self, token: &'ast Expr) {
        // MAKE A MACRO TO CREATE THIS AUTOMATICALLY
        match token {
            Expr::Lit(token) => {
                self.register_lit_expr(token);
            }
            Expr::ForLoop(token) => {
                self.register_for_loop_expr(token);
            }
            Expr::Unsafe(token) => {
                self.register_unsafe_expr(token);
            }
            Expr::MethodCall(token) => {
                self.register_method_call_expr(token);
            }
            Expr::Path(token) => {
                self.register_path_expr(token);
            }
            Expr::Reference(token) => {
                self.register_reference_expr(token);
            }
            Expr::Unary(token) => {
                self.register_unary_expr(token);
            }
            Expr::Binary(token) => {
                self.register_binary_expr(token);
            }
            Expr::Try(token) => {
                self.register_try_expr(token);
            }
            Expr::If(token) => {
                self.register_if_expr(token);
            }
            Expr::Call(token) => {
                self.register_call_expr(token);
            }
            Expr::Block(token) => {
                self.register_block_expr(token);
            }
            Expr::Paren(token) => {
                self.register_parentheses_expr(token);
            }
            Expr::Cast(token) => {
                self.register_cast_expr(token);
            }
            Expr::Field(token) => {
                self.register_field_expr(token);
            }
            Expr::Match(token) => {
                self.register_match_expr(token);
            }
            Expr::Tuple(token) => {
                self.register_tuple_expr(token);
            }
            _ => {}
        }
    }

    pub(crate) fn register_lit_expr(&mut self, token: &'ast ExprLit) {
        match &token.lit {
            Lit::Int(_) | Lit::Float(_) => {
                self.register_litnum_tag(&token.lit);
            }
            Lit::Bool(_) => {
                self.register_litbool_tag(&token.lit);
            }
            Lit::Byte(_) | Lit::ByteStr(_) | Lit::CStr(_) | Lit::Char(_) | Lit::Str(_) => {
                self.register_litstr_tag(&token.lit)
            }
            _ => {}
        }
    }

    pub(crate) fn register_for_loop_expr(&mut self, token: &'ast ExprForLoop) {
        self.register_keyword_tag(&token.for_token);
        self.register_pat(&token.pat);
        self.register_keyword_tag(&token.in_token);
        self.register_expr(&token.expr);
        self.register_block(&token.body);
    }

    pub(crate) fn register_unsafe_expr(&mut self, token: &'ast ExprUnsafe) {
        self.register_keyword_tag(&token.unsafe_token);
        self.register_block(&token.block);
    }

    pub(crate) fn register_method_call_expr(&mut self, token: &'ast ExprMethodCall) {
        self.register_expr(&token.receiver);
        self.register_function_tag(&token.method);
        for arg in &token.args {
            self.register_expr(arg);
        }
    }

    pub(crate) fn register_path_expr(&mut self, token: &'ast ExprPath) {
        self.try_register_qself(token.qself.as_ref());
        self.register_path(&token.path, None);
    }

    pub(crate) fn register_reference_expr(&mut self, token: &'ast ExprReference) {
        self.try_register_keyword_tag(token.mutability.as_ref());
        self.register_expr(&token.expr);
    }

    pub(crate) fn register_unary_expr(&mut self, token: &'ast ExprUnary) {
        self.register_expr(&token.expr);
    }

    pub(crate) fn register_binary_expr(&mut self, token: &'ast ExprBinary) {
        self.register_expr(&token.left);
        self.register_expr(&token.right);
    }

    pub(crate) fn register_try_expr(&mut self, token: &'ast ExprTry) {
        self.register_expr(&token.expr);
    }

    pub(crate) fn register_if_expr(&mut self, token: &'ast ExprIf) {
        self.register_keyword_tag(&token.if_token);
        self.register_expr(&token.cond);
        self.register_block(&token.then_branch);
        if let Some(else_block) = &token.else_branch {
            self.register_keyword_tag(&else_block.0);
            self.register_expr(&else_block.1);
        }
    }

    pub(crate) fn register_call_expr(&mut self, token: &'ast ExprCall) {
        // TODO UNDERSTAND HOW TO SIGNAL THIS PATH TO BE A FUNCTION
        self.register_expr(&token.func);
        for arg in &token.args {
            self.register_expr(arg);
        }
    }

    pub(crate) fn register_block_expr(&mut self, token: &'ast ExprBlock) {
        self.register_block(&token.block);
    }

    pub(crate) fn register_parentheses_expr(&mut self, token: &'ast ExprParen) {
        self.register_expr(&token.expr);
    }

    pub(crate) fn register_cast_expr(&mut self, token: &'ast ExprCast) {
        self.register_expr(&token.expr);
        self.register_keyword_tag(&token.as_token);
        self.register_type(&token.ty);
    }
    pub(crate) fn register_field_expr(&mut self, token: &'ast ExprField) {
        self.register_expr(&token.base);
        self.register_member(&token.member);
    }

    pub(crate) fn register_match_expr(&mut self, token: &'ast ExprMatch) {
        self.register_keyword_tag(&token.match_token);
        self.register_expr(&token.expr);
        for arm in &token.arms {
            self.register_arm(arm);
        }
    }

    pub(crate) fn register_tuple_expr(&mut self, token: &'ast ExprTuple) {
        for arg in &token.elems {
            self.register_expr(arg);
        }
    }

    pub(crate) fn register_arm(&mut self, token: &'ast Arm) {
        self.register_pat(&token.pat);
        if let Some(guard) = &token.guard {
            self.register_keyword_tag(&guard.0);
            self.register_expr(&guard.1);
        }
        self.register_expr(&token.body);
    }

    pub(crate) fn register_member(&mut self, token: &'ast Member) {
        match token {
            Member::Named(token) => {
                // TODO Can mark this as variable
                self.register_ident_tag(token);
            }
            Member::Unnamed(token) => {
                self.register_litnum_tag(token);
            }
        }
    }
}
