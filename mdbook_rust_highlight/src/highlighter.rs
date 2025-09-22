use std::collections::BTreeMap;

use crate::tokens::TokenTag;
use proc_macro2::{Span, TokenTree};
use regex::Regex;
use ropey::Rope;
use syn::{
    Block, Expr, ExprForLoop, ExprIf, ExprLit, ExprMethodCall, ExprPath, ExprReference, ExprTry,
    ExprUnary, ExprUnsafe, File, FnArg, Ident, Item, Lit, LitStr, Local, LocalInit, Pat, PatIdent,
    PatPath, PatReference, PatType, Path, QSelf, Stmt, StmtMacro, spanned::Spanned, token::Token,
    visit::Visit,
};

use mdbook_rust_highlight_derive::add_try_method;

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
            let identified = if let TokenTag::NeedIdentification = token {
                self.identify_token(index.clone())
            } else {
                token.clone()
            };
            let tag = identified.to_string();
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

    fn register_tag_on_index(&mut self, start: usize, end: usize, tag: TokenTag) {
        self.token_map.insert(start, tag);
        self.token_map.insert(end, TokenTag::EndOfToken);
    }

    pub(crate) fn register_token(&mut self, token: &impl Spanned, tag: TokenTag) {
        let (start, end) = self.span_position(token);
        self.register_tag_on_index(start, end, tag);
    }

    fn merge_tokens_span(t1: &impl Spanned, t2: &impl Spanned) -> Option<Span> {
        t1.span().join(t2.span())
    }

    fn identify_token(&self, index: usize) -> TokenTag {
        TokenTag::Ident
    }
}

impl RustHighlighter {
    #[add_try_method]
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

    #[add_try_method]
    fn register_expr(&mut self, token: &Expr) {
        // MAKE A MACRO TO CREATE THIS AUTOMATICALLY

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
            Expr::MethodCall(token) => {
                self.register_method_call(token);
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
            Expr::Try(token) => {
                self.register_try_expr(token);
            }
            Expr::If(token) => {
                self.register_if_expr(token);
            }
            _ => {}
        }
    }

    fn register_item(&mut self, token: &Item) {}

    #[add_try_method]
    fn register_macro_statement(&mut self, token: &StmtMacro) {
        // TODO NEED CHANGE TO RENDER PATH CORRECTLY AND TO PARSE TOKEN TREE BETTER WITH SPECIFIC KEY WORD FOR BUILTIN MACROS
        self.try_register_macro(
            Self::merge_tokens_span(&token.mac.path, &token.mac.bang_token).as_ref(),
        );
        for token in token.mac.tokens.clone() {
            if let TokenTree::Literal(lit) = token {
                if let Ok(_) = syn::parse_str::<LitStr>(&lit.to_string()) {
                    self.register_litstr(&lit);
                }
            }
        }
    }

    fn register_pattern(&mut self, token: &Pat) {
        match token {
            Pat::Ident(i) => {
                self.register_pat_ident(i);
            }
            Pat::Reference(r) => {
                self.register_reference_pat(r);
            }
            _ => {}
        }
    }
}

impl RustHighlighter {
    #[add_try_method]
    fn register_local(&mut self, token: &Local) {
        self.register_keyword(&token.let_token);
        self.register_pattern(&token.pat);
        self.try_register_local_init(token.init.as_ref());
    }

    #[add_try_method]
    fn register_block(&mut self, token: &Block) {
        for statement in &token.stmts {
            self.register_statement(&statement);
        }
    }

    #[add_try_method]
    fn register_for_loop(&mut self, token: &ExprForLoop) {
        self.register_keyword(&token.for_token);
        self.register_pattern(&token.pat);
        self.register_keyword(&token.in_token);
        self.register_expr(&token.expr);
        self.register_block(&token.body);
    }

    fn register_if_expr(&mut self, token: &ExprIf) {
        self.register_keyword(&token.if_token);
        self.register_expr(&token.cond);
        self.register_block(&token.then_branch);
        if let Some(else_block) = &token.else_branch {
            self.register_keyword(&else_block.0);
            self.register_expr(&else_block.1);
        }
    }

    fn register_try_expr(&mut self, token: &ExprTry) {
        self.register_expr(&token.expr);
    }

    fn register_unary_expr(&mut self, token: &ExprUnary) {
        self.register_expr(&token.expr);
    }

    fn register_reference_expr(&mut self, token: &ExprReference) {
        self.try_register_keyword(token.mutability.as_ref());
        self.register_expr(&token.expr);
    }

    fn register_path_expr(&mut self, token: &ExprPath) {
        self.try_register_qself(token.qself.as_ref());
        self.register_path(&token.path);
    }

    #[add_try_method]
    fn register_qself(&mut self, token: &QSelf) {
        self.register_type(&token.ty);
        self.try_register_keyword(token.as_token.as_ref());
    }

    fn register_path(&mut self, token: &Path) {
        let mut segment_iter = token.segments.iter().rev();
        let last_segment = segment_iter.next();
        for segment in &token.segments {
            self.register_segment(&segment.ident);
        }
        // TODO THINK OF A MECHANISM TO IDENTIFY THE TOKEN AT THE END
        if let Some(segment) = last_segment {
            // IMPROVE
            if segment.ident.to_string() == "self" || segment.ident == "Self" {
                self.register_selftoken(segment);
            } else {
                self.register_needidentification(segment);
            }
        }
    }

    fn register_method_call(&mut self, token: &ExprMethodCall) {
        self.register_expr(&token.receiver);
        self.register_function(&token.method);
        for arg in &token.args {
            self.register_expr(arg);
        }
    }

    fn register_unsafe_expr(&mut self, token: &ExprUnsafe) {
        self.register_keyword(&token.unsafe_token);
        self.register_block(&token.block);
    }

    #[add_try_method]
    fn register_reference_pat(&mut self, token: &PatReference) {
        self.try_register_keyword(token.mutability.as_ref());
        self.register_pattern(&token.pat);
    }

    #[add_try_method]
    fn register_local_init(&mut self, token: &LocalInit) {
        self.register_expr(&token.expr);
    }

    #[add_try_method]
    fn register_pat_ident(&mut self, token: &PatIdent) {
        self.try_register_keyword(token.by_ref.as_ref());
        self.try_register_keyword(token.mutability.as_ref());
        self.register_token(&token.ident, TokenTag::Ident);
    }

    #[add_try_method]
    fn register_type_pattern(&mut self, token: &PatType) {
        self.register_pattern(&token.pat);
    }

    fn register_comments(&mut self, code: &str) {
        let comment_regex: Regex = Regex::new(r"\/\/\/?.*").unwrap();
        for comment in comment_regex.captures_iter(code) {
            let m = comment.get(0).unwrap();
            self.register_tag_on_index(m.start(), m.end(), TokenTag::Comment);
        }
    }
}

impl RustHighlighter {
    pub fn highlight(code: &str) -> String {
        let mut highlighter = RustHighlighter {
            output: Rope::from_str(code),
            token_map: BTreeMap::new(),
        };

        highlighter.register_comments(code);

        let syntax_tree: File =
            syn::parse_str(code).expect(&format!("Failed to parse Rust code {}", code));

        highlighter.visit_file(&syntax_tree);
        highlighter.write_tokens();
        highlighter.output.to_string()
    }
}
