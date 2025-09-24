use crate::tokens::TokenTag;
use regex::Regex;
use ropey::Rope;
use std::collections::{BTreeMap, HashMap};
use syn::{File, PathSegment, spanned::Spanned, visit::Visit};

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
    token_map: BTreeMap<usize, TokenTag>,
    unidentified: HashMap<usize, &'ast PathSegment>,
}

impl<'ast> RustHighlighter<'ast> {
    pub(crate) fn highlight(code: &str) -> String {
        let mut highlighter = RustHighlighter::<'ast> {
            token_map: BTreeMap::new(),
            unidentified: HashMap::new(),
        };

        let code = highlighter.register_boring(code);

        for (k,v) in &highlighter.token_map {
            eprintln!("{k}: {:?}",v)
        }

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
        for (index, token) in self.token_map {
            match token {
                TokenTag::NeedIdentification => {
                    let ident_string = self.unidentified.get(&index).unwrap().ident.to_string();

                    let identified = match ident_string.as_str() {
                        "self" | "Self" => TokenTag::SelfToken,
                        "Ok" | "Err" | "NotATable" | "NoMapping" => TokenTag::Enum,
                        "new_unchecked" | "parse_str" => TokenTag::Function,
                        _ => TokenTag::Ident,
                    };

                    let tag = identified.to_string();
                    output.insert(index + tok_offset, tag.as_str());
                    tok_offset += tag.len();
                }
                _ => {
                    if let TokenTag::Boring = token {
                        eprintln!("{}", index);
                    }
                    let tag = token.to_string();
                    output.insert(index + tok_offset, tag.as_str());
                    tok_offset += tag.len();
                }
            }
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

    pub(crate) fn register_tag_on_index(&mut self, start: usize, end: usize, tag: TokenTag) {
        self.token_map.insert(start, tag);
        self.token_map.insert(end, TokenTag::EndOfToken);
    }

    pub(crate) fn register_token(&mut self, token: &impl Spanned, tag: TokenTag) {
        let (start, end) = Self::span_position(token);
        self.register_tag_on_index(start, end, tag);
    }

    /// Register a tag with start index of t1 and end index of t2.
    pub(crate) fn register_merged_token(
        &mut self,
        t1: &'ast impl Spanned,
        t2: &'ast impl Spanned,
        tag: TokenTag,
    ) {
        let p1 = Self::span_position(t1);
        let p2 = Self::span_position(t2);
        self.token_map.insert(p1.0, tag);
        self.token_map.insert(p2.1, TokenTag::EndOfToken);
    }

    pub(crate) fn register_comments(&mut self, code: &str) {
        let comment_regex: Regex = Regex::new(r"\/\/\/?[^\n]*").unwrap();
        for comment in comment_regex.captures_iter(code) {
            let m = comment.get(0).unwrap();
            self.register_tag_on_index(m.start(), m.end(), TokenTag::Comment);
        }
    }

    pub(crate) fn register_boring(&mut self, code: &str) -> String {
        // FIX BUG THAT IT WILL NOT WORK ON THE END OR START, AND ADD A WAY TO PROCESS MULTIPLE TOKEN
        // MAYBE THE ORDERED SET INSTEAD OF MAP WILL SOLVE THIS.
        // #(\s*)([^\[\n][^\n]*)
        let boring_regex = Regex::new(r"(#\s)(.*\n)").unwrap();

        for boring in boring_regex.captures_iter(code) {
            let hashtag_len = boring.get(1).unwrap().as_str().len();
            let boring_code = boring.get(2).unwrap();
            self.register_tag_on_index(
                boring_code.start() - hashtag_len,
                boring_code.end() - hashtag_len,
                TokenTag::Boring,
            );
        }

        boring_regex.replace_all(code, "\n$2\n").to_string()
    }
}

