use std::collections::BTreeMap;

use ropey::Rope;
use syn::{File, FnArg, spanned::Spanned, visit::Visit};

use crate::tokens::TokenTag;

pub struct RustHighlighter {
    output: Rope,
    token_map: BTreeMap<usize, TokenTag>,
}

impl<'ast> Visit<'ast> for RustHighlighter {
    fn visit_signature(&mut self, i: &'ast syn::Signature) {
        if let Some(abi) = &i.abi {
            self.register_token(abi.extern_token, TokenTag::Extern);
            self.try_register_token(abi.name.clone(), TokenTag::Abi);
        }
        self.try_register_token(i.asyncness, TokenTag::Asyncness);
        self.try_register_token(i.constness, TokenTag::Constness);
        self.try_register_token(i.unsafety, TokenTag::Unsafety);
        self.try_register_token(i.variadic.clone(), TokenTag::Variadic);
        self.register_token(i.fn_token, TokenTag::Fn);
        self.register_token(i.ident.clone(), TokenTag::FnName);
        for input in &i.inputs {
            match input {
                FnArg::Receiver(arg) => {
                    self.register_token(arg.self_token, TokenTag::SelfToken);
                    self.register_token(arg.lifetime(), TokenTag::LifeTime);
                }
                FnArg::Typed(arg) => {
                    self.register_token(arg.pat.clone(), TokenTag::FnArg);
                    self.register_token(arg.ty.clone(), TokenTag::FnType);
                }
            }
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
    fn span_position(&self, span: impl Spanned) -> (usize, usize) {
        // lines are 1 indexed instead of zero.
        let start_line = self.output.line_to_char(span.span().start().line - 1);
        (
            start_line + span.span().start().column,
            start_line + span.span().end().column,
        )
    }

    fn register_token(&mut self, token: impl Spanned, tag: TokenTag) {
        let (start_idx, end_idx) = self.span_position(token);
        self.token_map.insert(start_idx, tag);
        self.token_map.insert(end_idx, TokenTag::EndOfToken);
    }

    fn try_register_token(&mut self, token: Option<impl Spanned>, tag: TokenTag) {
        if let Some(t) = token {
            self.register_token(t, tag);
        }
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
