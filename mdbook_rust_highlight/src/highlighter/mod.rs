use crate::{
    highlighter::{self, error::IdentificationError},
    preprocessor::IdentMap,
    tokens::{PathToken, SpannedToken, Tag},
};
use regex::Regex;
use ropey::Rope;
use std::collections::{BTreeSet, HashMap};
use syn::{File, Ident, spanned::Spanned, token, visit::Visit};

pub mod attr;
pub mod error;
pub mod expr;
pub mod generics;
pub mod global;
pub mod item;
pub mod macro_parsers;
pub mod pat;
pub mod path;
pub mod statement;
pub mod structure;
pub mod ty;
pub mod visit;

pub trait Registerable: syn::spanned::Spanned + Sized {
    fn register(&self, h: &mut RustHighlighter);
}
pub struct RustHighlighter<'a, 'ast> {
    token_set: BTreeSet<SpannedToken>,
    unidentified: HashMap<usize, PathToken<'ast>>,
    ident_map: IdentMap<'a>,
}

impl<'a, 'ast> RustHighlighter<'a, 'ast> {
    fn highlight(&mut self, code: &'ast str) -> String {
        let code = self.register_boring(code);

        let mut output = Rope::from_str(&code);
        let syntax_tree: File =
            syn::parse_str(&code).expect(&format!("Failed to parse Rust code\n{}", code));

        self.visit_file(&syntax_tree);
        self.register_comments(&code);
        self.write_tokens(&mut output);

        output.to_string()
    }

    fn register(&mut self, token: &impl Registerable) {
        token.register(self);
    }

    fn register_at(&mut self, start: usize, stop: usize, t: Option<Tag>) {
        self.token_set.insert(SpannedToken {
            kind: t,
            start,
            end: stop,
        });
        self.token_set.insert(SpannedToken {
            kind: Some(Tag::EndOfToken),
            start: stop,
            end: usize::MAX,
        });
    }
}

impl<'a, 'ast> RustHighlighter<'a, 'ast> {
    pub(crate) fn write_tokens(&mut self, output: &mut Rope) {
        let mut tok_offset: usize = 0;
        let mut set_iterator = self.token_set.iter();
        while let Some(token) = set_iterator.next() {
            let identified = match self.identify_token(&token) {
                Ok(identified) => identified,
                Err(IdentificationError::AlreadyIdentified) => token.clone(),
                Err(IdentificationError::NoIdentificationNeeded) => {
                    continue;
                }
            };
            let tag = identified.kind.unwrap().to_string();
            output.insert(identified.start + tok_offset, tag.as_str());
            tok_offset += tag.len();
        }
        self.token_set.clear();
        self.unidentified.clear();
    }

    pub(crate) fn remember_as(&mut self, ident: &(impl Spanned + ToString), t: Tag) {
        self.ident_map.insert(ident.to_string().leak(), t);
    }

    /// Returns the identified token for ones the need identification, and for all others, None.
    pub(crate) fn identify_token(
        &self,
        token: &SpannedToken,
    ) -> Result<SpannedToken, IdentificationError> {
        match token.kind {
            None => {
                let unidentified = self.unidentified.get(&token.start);
                let ident_string = match unidentified {
                    Some(segment) => segment.ident().to_string(),
                    None => return Err(IdentificationError::NoIdentificationNeeded),
                };

                let identified = self
                    .ident_map
                    .get(ident_string.as_str())
                    .cloned()
                    .unwrap_or(Tag::Type);

                Ok(SpannedToken {
                    kind: Some(identified),
                    start: token.start,
                    end: token.end,
                })
            }
            Some(_) => Err(IdentificationError::AlreadyIdentified),
        }
    }

    pub(crate) fn register_ident(&mut self, token: &Ident) {
        self.register(token);
        if let Some(tag) = tag {
            self.remember_as(token, tag);
        } else {
            unimplemented!("Use register unidentified instead, later will require only TokenTag")
        }
    }

    pub(crate) fn register_unidentified(&mut self, token: PathToken<'ast>) {
        self.register(token.ident());
        self.unidentified
            .insert(token.span().byte_range().start, token);
    }

    pub(crate) fn register_comments(&mut self, code: &str) {
        let comment_regex: Regex = Regex::new(r"\/\/\/?.*\n?").unwrap();
        for comment in comment_regex.captures_iter(code) {
            let m = comment.get(0).unwrap();
            self.register_tag_at_index(m.start(), m.end(), Some(Tag::Comment));
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
                self.register_tag_at_index(start, end, Some(Tag::Boring));
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
