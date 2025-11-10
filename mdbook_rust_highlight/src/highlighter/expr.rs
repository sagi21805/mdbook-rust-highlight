use syn::{
    Arm, Expr, ExprAssign, ExprBinary, ExprBlock, ExprCall, ExprCast, ExprField, ExprForLoop,
    ExprIf, ExprLit, ExprLoop, ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprReference,
    ExprReturn, ExprStruct, ExprTry, ExprTuple, ExprUnary, ExprUnsafe, Lit, Member,
    spanned::Spanned, token,
};

use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};

impl Register for Expr {
    fn register_as(&self, h: &mut RustHighlighter, tag: Option<Tag>) {
        match self {
            // Expr::Array(token)
            // | Expr::Assign(token)
            // | Expr::Async(token)
            // | Expr::Await(token)
            // | Expr::Binary(token)
            // | Expr::Block(token)
            // | Expr::Break(token)
            // | Expr::Call(token)
            // | Expr::Cast(token)
            // | Expr::Closure(token)
            // | Expr::Const(token)
            // | Expr::Continue(token)
            // | Expr::Field(token)
            // | Expr::ForLoop(token)
            // | Expr::Group(token)
            // | Expr::If(token)
            // | Expr::Index(token)
            // | Expr::Infer(token)
            // | Expr::Let(token)
            // | Expr::Lit(token)
            // | Expr::Loop(token)
            // | Expr::Macro(token)
            // | Expr::Match(token)
            // | Expr::MethodCall(token)
            // | Expr::Paren(token)
            // | Expr::Path(token)
            // | Expr::Range(token)
            // | Expr::RawAddr(token)
            // | Expr::Reference(token)
            // | Expr::Repeat(token)
            // | Expr::Return(token)
            // | Expr::Struct(token)
            // | Expr::Try(token)
            // | Expr::TryBlock(token)
            // | Expr::Tuple(token)
            // | Expr::Unary(token)
            // | Expr::Unsafe(token)
            // | Expr::While(token)
            // | Expr::Yield(token)
            // | Expr::Verbatim(token) => {
                // h.register_as(token, tag);
            }
    }
}

impl Register for ExprAssign {
    fn register_as(&self, h: &mut RustHighlighter, tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.left);
        h.register(&self.right);
    }
}

impl Register for ExprReturn {
    fn register_as(&self, h: &mut RustHighlighter, tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_keyword_tag(&self.return_token);
        if let Some(expr) = self.expr.as_ref().map(|v| &**v) {
            h.register(expr);
        }
    }
}

impl<'a> RustHighlighter<'a> {
    pub(crate) fn register_return_expr(&mut self, token: &ExprReturn) {}

    pub(crate) fn register_struct_expr(&mut self, token: &ExprStruct) {
        self.try_register_qself(token.qself.as_ref());
        self.register_path(&token.path, Some(Tag::Type));
        for field in &token.fields {
            self.register_field_value(field);
        }
        if let Some(expr) = token.rest.as_ref().map(|v| &**v) {
            self.register_expr(expr, None);
        }
    }

    pub(crate) fn register_lit_expr(&mut self, token: &ExprLit) {
        self.register_attributes(&token.attrs);
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

    pub(crate) fn register_for_loop_expr(&mut self, token: &ExprForLoop, identifier: Option<Tag>) {
        self.register_attributes(&token.attrs);
        self.register_keyword_tag(&token.for_token);
        self.register_pat(&token.pat, None);
        self.register_keyword_tag(&token.in_token);
        self.register_expr(&token.expr, identifier);
        self.register_block(&token.body);
    }

    pub(crate) fn register_unsafe_expr(&mut self, token: &ExprUnsafe) {
        self.register_keyword_tag(&token.unsafe_token);
        self.register_block(&token.block);
    }

    /// Identifier for the reciever
    pub(crate) fn register_method_call_expr(
        &mut self,
        token: &ExprMethodCall,
        identifier: Option<Tag>,
    ) {
        self.register_expr(&token.receiver, identifier);
        self.register_function_tag(&token.method);
        for arg in &token.args {
            self.register_expr(arg, None);
        }
    }

    /// Identifier for the last token on the path
    pub(crate) fn register_path_expr(&mut self, token: &ExprPath, identifier: Option<Tag>) {
        self.try_register_qself(token.qself.as_ref());
        self.register_path(&token.path, identifier);
    }

    pub(crate) fn register_reference_expr(
        &mut self,
        token: &ExprReference,
        identifier: Option<Tag>,
    ) {
        self.try_register_keyword_tag(token.mutability.as_ref());
        self.register_expr(&token.expr, identifier);
    }

    pub(crate) fn register_unary_expr(&mut self, token: &ExprUnary, identifier: Option<Tag>) {
        self.register_expr(&token.expr, identifier);
    }

    /// TODO CONSIDER HANDLING CASE WHICH THERE ARE MULTIPLE EXPRS, MAYBE AS ENUM
    pub(crate) fn register_binary_expr(&mut self, token: &ExprBinary) {
        self.register_expr(&token.left, None);
        self.register_expr(&token.right, None);
    }

    pub(crate) fn register_try_expr(&mut self, token: &ExprTry, identifier: Option<Tag>) {
        self.register_expr(&token.expr, identifier);
    }

    /// SAME TWO EXPRS
    pub(crate) fn register_if_expr(&mut self, token: &ExprIf) {
        self.register_keyword_tag(&token.if_token);
        self.register_expr(&token.cond, None);
        self.register_block(&token.then_branch);
        if let Some(else_block) = &token.else_branch {
            self.register_keyword_tag(&else_block.0);
            self.register_expr(&else_block.1, None);
        }
    }

    pub(crate) fn register_call_expr(&mut self, token: &ExprCall) {
        self.register_expr(&token.func, Some(Tag::Function));
        let token_position = token.span().byte_range();
        // TODO Logic may be broken
        for pos in token_position.start..token_position.end {
            if let Some(identified) = self.unidentified.remove(&pos) {
                if let Some(known) = self.ident_map.get(identified.ident().to_string().as_str()) {
                    self.register_ident(identified.ident(), Some(known.clone()));
                } else {
                    self.register_unidentified(identified);
                }
            }
        }
        for arg in &token.args {
            self.register_expr(arg, None);
        }
    }

    pub(crate) fn register_block_expr(&mut self, token: &ExprBlock) {
        self.register_block(&token.block);
    }

    pub(crate) fn register_parentheses_expr(&mut self, token: &ExprParen, identifier: Option<Tag>) {
        self.register_expr(&token.expr, identifier);
    }

    pub(crate) fn register_cast_expr(&mut self, token: &ExprCast, identifier: Option<Tag>) {
        self.register_expr(&token.expr, identifier);
        self.register_keyword_tag(&token.as_token);
        self.register_type(&token.ty);
    }
    pub(crate) fn register_field_expr(&mut self, token: &ExprField, identifier: Option<Tag>) {
        self.register_expr(&token.base, identifier);
        self.register_member(&token.member);
    }

    pub(crate) fn register_match_expr(&mut self, token: &ExprMatch, identifier: Option<Tag>) {
        self.register_keyword_tag(&token.match_token);
        self.register_expr(&token.expr, identifier);
        for arm in &token.arms {
            self.register_arm(arm);
        }
    }

    pub(crate) fn register_tuple_expr(&mut self, token: &ExprTuple) {
        for arg in &token.elems {
            self.register_expr(arg, None);
        }
    }

    pub(crate) fn register_arm(&mut self, token: &Arm) {
        self.register_pat(&token.pat, None);
        if let Some(guard) = &token.guard {
            self.register_keyword_tag(&guard.0);
            self.register_expr(&guard.1, None);
        }
        self.register_expr(&token.body, None);
    }

    pub(crate) fn register_member(&mut self, token: &Member) {
        match token {
            Member::Named(token) => {
                self.register_variable_tag(token);
            }
            Member::Unnamed(token) => {
                self.register_litnum_tag(token);
            }
        }
    }

    pub(crate) fn register_loop_expr(&mut self, token: &ExprLoop) {
        self.register_keyword_tag(&token.loop_token);
        self.register_block(&token.body);
    }
}
