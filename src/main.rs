use std::collections::BTreeMap;

use ropey::Rope;
use syn::{File, spanned::Spanned, visit::Visit};

/// Token mapping with

pub enum TokenTag {
    Visibility,
    Abi,
    Fn,
    EndOfToken,
    Extern,
}

macro_rules! token_string {
    ($name:ident) => {
        String::from(concat!("<span class=\"hlrs-", stringify!($name), "\">").to_lowercase())
    };
}

impl ToString for TokenTag {
    fn to_string(&self) -> String {
        match self {
            Self::EndOfToken => String::from("</span>"),
            Self::Visibility => token_string!(Visibility),
            Self::Extern => token_string!(Extern),
            Self::Fn => token_string!(Fn),
            Self::Abi => token_string!(Abi),
        }
    }
}

struct Highlighter {
    output: Rope,
    token_map: BTreeMap<usize, TokenTag>,
}

impl<'ast> Visit<'ast> for Highlighter {
    fn visit_signature(&mut self, i: &'ast syn::Signature) {
        match &i.abi {
            Some(a) => {
                self.insert_token(a.extern_token, TokenTag::Extern);
                self.insert_token(a.name.clone().unwrap(), TokenTag::Abi);
            }
            None => {}
        }
    }
}

impl Highlighter {
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

    fn insert_token(&mut self, token: impl Spanned, tag: TokenTag) {
        let (start_idx, end_idx) = self.span_position(token);
        self.token_map.insert(start_idx, tag);
        self.token_map.insert(end_idx, TokenTag::EndOfToken);
    }

    fn highlight_rust_code(code: &str) -> Self {
        let syntax_tree: File = syn::parse_str(code).expect("Failed to parse Rust code");

        let mut highlighter = Highlighter {
            output: Rope::from_str(code),
            token_map: BTreeMap::new(),
        };

        highlighter.visit_file(&syntax_tree);
        highlighter.write_tokens();
        highlighter
    }
}

fn main() {
    let code = r#"
        const unsafe fn test(a: B, c: D) {

        }

        pub const unsafe extern "testing" fn another(x: i32) {
            let y = x + 1;
        }

        fn testing(&self, x: i32) {
        }  
    "#;

    let h = Highlighter::highlight_rust_code(code);

    println!("{}", h.output.to_string());
}
