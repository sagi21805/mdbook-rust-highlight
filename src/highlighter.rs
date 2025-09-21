use std::collections::BTreeMap;

use crate::tokens::TokenTag;
use paste::paste;
use ropey::Rope;
use syn::{
    Expr, File, FnArg, Ident, Item, Lifetime, LitStr, Local, LocalInit, Pat, Stmt, StmtMacro,
    spanned::Spanned, token::Token, visit::Visit,
};

macro_rules! make_register_wrappers {
    ($name:ident, $ty:ty) => {
        paste! {
            #[allow(dead_code)]
            fn [<register_ $name _box>](&mut self, v: &Box<$ty>) {
                self.[<register_ $name _ref>](v.as_ref());
            }

            #[allow(dead_code)]
            fn [<try_register_ $name>](&mut self, v: Option<&$ty>) {
                if let Some(v) = v {
                    self.[<register_ $name _ref>](v);
                }
            }
        }
    };
}
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
            self.register_keyword_ref(&abi.extern_token);
            self.try_register_string(abi.name.as_ref());
        }
        self.register_keyword_ref(&i.fn_token);
        self.register_token_ref(&i.ident, TokenTag::Function);
        for input in &i.inputs {
            match input {
                FnArg::Receiver(arg) => {
                    self.register_token_ref(&arg.self_token, TokenTag::SelfToken);
                    self.try_register_lifetime(arg.lifetime());
                }
                FnArg::Typed(type_pat) => {
                    self.register_type_pattern(type_pat);
                    self.register_token_ref(&type_pat.ty, TokenTag::Type);
                }
            }
        }

        // TODO: Make right for all return types, some are more complicated
        match i.output.clone() {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, t) => self.register_token_ref(&t, TokenTag::Type),
        }
    }

    fn visit_abi(&mut self, i: &'ast syn::Abi) {
        self.register_keyword_ref(&i.extern_token);
        self.try_register_string(i.name.as_ref());
    }

    fn visit_expr_async(&mut self, i: &'ast syn::ExprAsync) {
        self.register_keyword_ref(&i.async_token);
    }

    fn visit_visibility(&mut self, i: &'ast syn::Visibility) {
        self.register_keyword_ref(i);
    }

    fn visit_block(&mut self, i: &'ast syn::Block) {
        for statement in &i.stmts {
            self.register_statement(statement);
        }
    }
}

impl RustHighlighter {
    fn write_tokens(&mut self) {
        let mut tok_offset: usize = 0;
        for (key, val) in &self.token_map {
            let tag = val.to_string();
            self.output.insert(key + tok_offset, tag.as_str());
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
        let start_line = self.output.line_to_char(span.span().start().line - 1);
        (
            start_line + span.span().start().column,
            start_line + span.span().end().column,
        )
    }

    fn register_token_ref(&mut self, token: &impl Spanned, tag: TokenTag) {
        let (start_idx, end_idx) = self.span_position(token);
        self.token_map.insert(start_idx, tag);
        self.token_map.insert(end_idx, TokenTag::EndOfToken);
    }

    fn register_string_ref(&mut self, v: &LitStr) {
        self.register_token_ref(v, TokenTag::String);
    }

    make_register_wrappers!(string, LitStr);

    fn register_ident_ref(&mut self, v: &Ident) {
        self.register_token_ref(v, TokenTag::Ident);
    }

    fn register_statement(&mut self, v: &Stmt) {
        match v {
            syn::Stmt::Local(l) => {
                self.register_local_ref(l);
            }
            syn::Stmt::Expr(e, _) => {}
            syn::Stmt::Macro(m) => {}
            syn::Stmt::Item(i) => {}
        }
    }

    fn register_local_ref(&mut self, v: &Local) {
        self.register_keyword_ref(&v.let_token);
        self.register_pattern_ref(&v.pat);
        self.try_register_local_init(v.init.as_ref());
    }

    make_register_wrappers!(local, Local);

    fn register_expr_ref(&mut self, v: &Expr) {}

    fn register_macro(&mut self, v: &StmtMacro) {}

    fn register_item(&mut self, v: Item) {}

    fn register_pattern_ref(&mut self, pattern: &Pat) {
        match pattern {
            syn::Pat::Ident(i) => {
                self.register_pat_ident(i);
            }
            _ => {}
        }
    }

    make_register_wrappers!(pattern, Pat);

    fn register_local_init_ref(&mut self, init: &LocalInit) {}

    make_register_wrappers!(local_init, LocalInit);

    fn register_pat_ident(&mut self, pattern: &syn::PatIdent) {
        self.try_register_keyword(pattern.by_ref.as_ref());
        self.try_register_keyword(pattern.mutability.as_ref());
        self.register_token_ref(&pattern.ident, TokenTag::Ident);
    }

    fn register_type_pattern(&mut self, pattern: &syn::PatType) {
        self.register_pattern_box(&pattern.pat);
    }

    fn register_lifetime_ref(&mut self, lifetime: &Lifetime) {
        self.register_token_ref(lifetime, TokenTag::LifeTime);
    }

    make_register_wrappers!(lifetime, Lifetime);

    fn register_keyword_ref(&mut self, token: &impl Spanned) {
        self.register_token_ref(token, TokenTag::Keyword);
    }

    make_register_wrappers!(keyword, impl Spanned);

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
