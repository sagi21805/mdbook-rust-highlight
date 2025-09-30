use crate::{
    highlighter::error::IdentificationError,
    preprocessor::IdentMap,
    tokens::{SpannedToken, TokenTag},
};
use regex::Regex;
use ropey::Rope;
use std::collections::{BTreeSet, HashMap};
use syn::{File, Ident, PathSegment, spanned::Spanned, visit::Visit};

pub mod error;
pub mod expr;
pub mod generics;
pub mod item;
pub mod pat;
pub mod path;
pub mod statement;
pub mod ty;
pub mod visit;

pub struct RustHighlighter<'a, 'ast> {
    token_set: BTreeSet<SpannedToken>,
    unidentified: HashMap<usize, &'ast PathSegment>,
    ident_map: IdentMap<'a>,
}

impl<'a, 'ast> RustHighlighter<'a, 'ast> {
    pub(crate) fn highlight(code: &str, ident_map: IdentMap<'a>) -> String {
        let mut highlighter = Self::new(ident_map);

        let code = highlighter.register_boring(code);

        let mut output = Rope::from_str(&code);

        let syntax_tree: File =
            syn::parse_str(&code).expect(&format!("Failed to parse Rust code\n{}", code));

        highlighter.visit_file(&syntax_tree);
        highlighter.register_comments(&code);
        highlighter.write_tokens(&mut output);

        output.to_string()
    }

    pub(crate) fn write_tokens(&mut self, output: &mut Rope) {
        let mut tok_offset: usize = 0;
        let mut set_iterator = self.token_set.iter();
        while let Some(token) = set_iterator.next() {
            let identified = match self.identify_token(&token) {
                Ok(identified) => identified,
                Err(IdentificationError::AlreadyIdentified) => token.clone(),
                Err(IdentificationError::NoIdentificationNeeded) => {
                    let _ = set_iterator.next();
                    continue;
                }
            };
            let tag = identified.kind.to_string();
            output.insert(identified.start + tok_offset, tag.as_str());
            tok_offset += tag.len();
        }
        self.token_set.clear();
        self.unidentified.clear();
    }

    pub(crate) fn remember_ident(&mut self, ident: &(impl Spanned + ToString), token: TokenTag) {
        self.ident_map.insert(ident.to_string().leak(), token);
    }

    /// Returns the identified token for ones the need identification, and for all others, None.
    pub(crate) fn identify_token(
        &self,
        token: &SpannedToken,
    ) -> Result<SpannedToken, IdentificationError> {
        match token.kind {
            TokenTag::NeedIdentification => {
                let unidentified = self.unidentified.get(&token.start);
                let ident_string = match unidentified {
                    Some(segment) => segment.ident.to_string(),
                    None => return Err(IdentificationError::NoIdentificationNeeded),
                };

                let identified = self
                    .ident_map
                    .get(ident_string.as_str())
                    .cloned()
                    .unwrap_or(TokenTag::Ident);

                Ok(SpannedToken {
                    kind: identified,
                    start: token.start,
                    end: token.end,
                })
            }
            _ => Err(IdentificationError::AlreadyIdentified),
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

    pub(crate) fn register_ident(&mut self, ident: &(impl Spanned + ToString), tag: TokenTag) {
        self.remember_ident(ident, tag);
        self.register_tag(ident, tag);
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

impl<'a, 'ast> RustHighlighter<'a, 'ast> {
    fn new(ident_map: IdentMap<'a>) -> Self {
        Self {
            token_set: BTreeSet::new(),
            unidentified: HashMap::new(),
            ident_map,
        }
    }
}
