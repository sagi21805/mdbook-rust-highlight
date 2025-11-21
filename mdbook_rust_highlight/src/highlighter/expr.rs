use syn::{
    Arm, Expr, ExprAssign, ExprBinary, ExprBlock, ExprCall, ExprCast, ExprConst, ExprField,
    ExprForLoop, ExprIf, ExprIndex, ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall,
    ExprParen, ExprPath, ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry, ExprTuple,
    ExprUnary, ExprUnsafe, Lit, Member, spanned::Spanned,
};

use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};

impl Register for Expr {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            // Expr::Array(token) => h.register_as(token, _tag),
            Expr::Assign(token) => h.register_as(token, _tag),
            // Expr::Async(token) => h.register_as(token, _tag),
            // Expr::Await(token) => h.register_as(token, _tag),
            Expr::Binary(token) => h.register_as(token, _tag),
            Expr::Block(token) => h.register_as(token, _tag),
            // Expr::Break(token) => h.register_as(token, _tag),
            Expr::Call(token) => h.register_as(token, _tag),
            Expr::Cast(token) => h.register_as(token, _tag),
            // Expr::Closure(token) => h.register_as(token, _tag),
            Expr::Const(token) => h.register_as(token, _tag),
            // Expr::Continue(token) => h.register_as(token, _tag),
            Expr::Field(token) => h.register_as(token, _tag),
            Expr::ForLoop(token) => h.register_as(token, _tag),
            // Expr::Group(token) => h.register_as(token, _tag),
            Expr::If(token) => h.register_as(token, _tag),
            Expr::Index(token) => h.register_as(token, _tag),
            // Expr::Infer(token) => h.register_as(token, _tag),
            // Expr::Let(token) => h.register_as(token, _tag),
            Expr::Lit(token) => h.register_as(token, _tag),
            Expr::Loop(token) => h.register_as(token, _tag),
            Expr::Macro(token) => h.register_as(token, _tag),
            Expr::Match(token) => h.register_as(token, _tag),
            Expr::MethodCall(token) => h.register_as(token, _tag),
            Expr::Paren(token) => h.register_as(token, _tag),
            Expr::Path(token) => h.register_as(token, _tag),
            // Expr::Range(token) => h.register_as(token, _tag),
            // Expr::RawAddr(token) => h.register_as(token, _tag),
            Expr::Reference(token) => h.register_as(token, _tag),
            Expr::Repeat(token) => h.register_as(token, _tag),
            Expr::Return(token) => h.register_as(token, _tag),
            Expr::Struct(token) => h.register_as(token, _tag),
            Expr::Try(token) => h.register_as(token, _tag),
            // Expr::TryBlock(token) => h.register_as(token, _tag),
            Expr::Tuple(token) => h.register_as(token, _tag),
            Expr::Unary(token) => h.register_as(token, _tag),
            // Expr::While(token) => h.register_as(token, _tag),
            // Expr::Yield(token) => h.register_as(token, _tag),
            Expr::Unsafe(token) => h.register_as(token, _tag),
            _ => {}
        }
    }
}

impl Register for ExprIndex {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.expr);
        h.register(&self.index);
    }
}

impl Register for ExprRepeat {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.expr);
        h.register(&self.len);
    }
}

impl Register for ExprMacro {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.mac);
    }
}

impl Register for ExprConst {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_keyword_tag(&self.const_token);
        h.register(&self.block);
    }
}

impl Register for ExprAssign {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.left);
        h.register(&self.right);
    }
}

impl Register for ExprReturn {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_keyword_tag(&self.return_token);
        if let Some(expr) = self.expr.as_ref().map(|v| &**v) {
            h.register(expr);
        }
    }
}

impl Register for ExprStruct {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.qself);
        h.register_as(&self.path, _tag);
        h.register(&self.fields);
        h.register(&self.rest);
    }
}

impl Register for ExprLit {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        match &self.lit {
            Lit::Int(_) | Lit::Float(_) => {
                h.register_litnum_tag(&self.lit);
            }
            Lit::Bool(_) => {
                h.register_litbool_tag(&self.lit);
            }
            Lit::Byte(_) | Lit::ByteStr(_) | Lit::CStr(_) | Lit::Char(_) | Lit::Str(_) => {
                h.register_litstr_tag(&self.lit);
            }
            _ => {}
        }
    }
}

impl Register for ExprForLoop {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_keyword_tag(&self.for_token);
        h.register(&self.pat);
        h.register_keyword_tag(&self.in_token);
        h.register_as(&self.expr, _tag);
        h.register(&self.body);
    }
}

impl Register for ExprUnsafe {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_keyword_tag(&self.unsafe_token);
        h.register(&self.block);
    }
}

impl Register for ExprMethodCall {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.receiver);
        h.register_function_tag(&self.method);
        h.register(&self.turbofish);
        h.register(&self.args);
    }
}

impl Register for ExprPath {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.qself);
        h.register_as(&self.path, _tag);
    }
}

impl Register for ExprReference {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.try_register_keyword_tag(self.mutability.as_ref());
        h.register_as(&self.expr, _tag);
    }
}

impl Register for ExprUnary {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_as(&self.expr, _tag);
    }
}

impl Register for ExprBinary {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.left);
        h.register(&self.right);
    }
}

impl Register for ExprTry {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_as(&self.expr, _tag);
    }
}

impl Register for ExprIf {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register_keyword_tag(&self.if_token);
        h.register(&self.cond);
        h.register(&self.then_branch);
        if let Some((else_token, block)) = &self.else_branch {
            h.register_keyword_tag(else_token);
            h.register(block);
        }
    }
}

impl Register for ExprCall {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_as(&self.func, Some(Tag::Function));
        h.register(&self.args);
        // let token_position = self.span().byte_range();
        // TODO Logic may be broken
        // for pos in token_position.start..token_position.end {
        //     if let Some(unidentified) = h.unidentified.remove(&pos) {
        //         if let Some(known) = h.ident_map.get(unidentified.to_string().as_str()) {
        //             h.register_ident(&unidentified, Some(known.clone()));
        //         } else {
        //             h.register_unidentified(&unidentified);
        //         }
        //     }
        // }
        h.register(&self.args);
    }
}

impl Register for ExprBlock {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.block);
    }
}

impl Register for ExprParen {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_as(&self.expr, _tag);
    }
}

impl Register for ExprCast {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_as(&self.expr, _tag);
        h.register_keyword_tag(&self.as_token);
        h.register(&self.ty);
    }
}

impl Register for ExprField {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.base);
        h.register(&self.member);
    }
}

impl Register for ExprMatch {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_keyword_tag(&self.match_token);
        h.register_as(&self.expr, _tag);
        for arm in &self.arms {
            h.register(arm);
        }
    }
}

impl Register for ExprTuple {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.elems);
    }
}

impl Register for Arm {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.pat);
        if let Some((if_token, guard_expr)) = &self.guard {
            h.register_keyword_tag(&if_token);
            h.register(guard_expr);
        }
        h.register(&self.body);
    }
}

impl Register for Member {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            Member::Named(token) => {
                h.register_variable_tag(token);
            }
            Member::Unnamed(token) => {
                h.register_litnum_tag(token);
            }
        }
    }
}

impl Register for ExprLoop {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register_keyword_tag(&self.loop_token);
        h.register(&self.body);
    }
}
