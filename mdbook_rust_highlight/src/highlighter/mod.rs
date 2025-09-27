use crate::tokens::{SpannedToken, TokenTag};
use regex::Regex;
use ropey::Rope;
use std::{
    collections::{BTreeSet, HashMap},
    string, usize,
};
use syn::{File, PathSegment, spanned::Spanned, token::Token, visit::Visit};

pub mod expr;
pub mod generics;
pub mod item;
pub mod pat;
pub mod path;
pub mod statement;
pub mod ty;
pub mod visit;

pub struct RustHighlighter<'ast> {
    // TODO CONSIDER CHANGING INTO A SET AND THE TOKEN WILL HOLD THE USIZE IN IT
    token_set: BTreeSet<SpannedToken>,
    unidentified: HashMap<usize, &'ast PathSegment>,
    pub token_count: usize,
}

impl<'ast> RustHighlighter<'ast> {
    pub(crate) fn highlight(code: &str) -> String {
        let mut highlighter = RustHighlighter::<'ast> {
            token_set: BTreeSet::new(),
            unidentified: HashMap::new(),
            token_count: 0,
        };

        let code = highlighter.register_boring(code);

        let mut output = Rope::from_str(&code);

        let syntax_tree: File =
            syn::parse_str(&code).expect(&format!("Failed to parse Rust code\n{}", code));

        highlighter.visit_file(&syntax_tree);
        highlighter.register_comments(&code);
        highlighter.write_tokens(&mut output);

        output.to_string()
    }

    pub(crate) fn write_tokens(self, output: &mut Rope) {
        let mut tok_offset: usize = 0;
        for token in &self.token_set {
            eprintln!("{:?}", token);
            let identified = self.identify_token(&token).unwrap_or(token.clone());
            let tag = identified.kind.to_string();
            output.insert(identified.start + tok_offset, tag.as_str());
            tok_offset += tag.len();
        }
    }

    /// Returns the identified token for ones the need identification, and for all others, None.
    pub(crate) fn identify_token(&self, token: &SpannedToken) -> Option<SpannedToken> {
        match token.kind {
            TokenTag::NeedIdentification => {
                let ident_string = self
                    .unidentified
                    .get(&token.start)
                    .unwrap()
                    .ident
                    .to_string();
                let identified = match ident_string.as_str() {
                    "self" | "Self" => TokenTag::SelfToken,
                    "Ok" | "Err" | "NotATable" | "NoMapping" => TokenTag::Enum,
                    "new_unchecked" | "parse_str" => TokenTag::Function,
                    _ => TokenTag::Ident,
                };
                Some(SpannedToken {
                    kind: identified,
                    start: token.start,
                    end: token.end,
                })
            }
            _ => None,
        }
    }

    /// Extract a span position in the rope.
    ///
    /// returns the (start_idx, end_idx) of the span
    pub(crate) fn span_position(span: &impl Spanned) -> (usize, usize) {
        // lines are 1 indexed instead of zero.
        let span = span.span().byte_range();
        (span.start, span.end)
    }

    pub(crate) fn register_tag_at_index(&mut self, start: usize, end: usize, tag: TokenTag) {
        self.token_set.insert(SpannedToken {
            kind: tag,
            start,
            end,
        });
        self.token_set.insert(SpannedToken {
            kind: TokenTag::EndOfToken,
            start: end,
            end: usize::MAX,
        });
    }

    pub(crate) fn register_tag(&mut self, token: &impl Spanned, tag: TokenTag) {
        let (start, end) = Self::span_position(&token.span());
        self.register_tag_at_index(start, end, tag);
    }

    pub(crate) fn register_comments(&mut self, code: &str) {
        let comment_regex: Regex = Regex::new(r"\/\/\/?[^\n]*").unwrap();
        for comment in comment_regex.captures_iter(code) {
            let m = comment.get(0).unwrap();
            self.register_tag_at_index(m.start(), m.end(), TokenTag::Comment);
        }
    }

    pub(crate) fn register_boring(&mut self, code: &str) -> String {
        // FIX BUG THAT IT WILL NOT WORK ON THE END OR START, AND ADD A WAY TO PROCESS MULTIPLE TOKEN
        // MAYBE THE ORDERED SET INSTEAD OF MAP WILL SOLVE THIS.
        // #(\s*)([^\[\n][^\n]*)
        // let boring_regex = Regex::new(r"(?m)(#\s)(.*)$").unwrap();
        let mut string_offset = 0;
        let mut output = String::with_capacity(code.len());
        for line in code.split_inclusive('\n') {
            if let Some(hash_position) = line.find("# ") {
                let after_hash = &line[(hash_position + 2)..];
                let start = string_offset + hash_position;
                let end = string_offset + line.len() - 2;
                output.push_str(after_hash);
                self.register_tag_at_index(start, end, TokenTag::Boring);
            } else {
                output.push_str(line);
            }
            string_offset += line.len() - 2;
        }
        output
    }
}
