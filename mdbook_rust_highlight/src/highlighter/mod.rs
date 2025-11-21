use crate::{
    highlighter::{error::IdentificationError, path_tracer::PathTracer},
    tokens::{SpannedToken, Tag},
};
use regex::Regex;
use ropey::Rope;
use std::collections::{BTreeSet, HashMap};
use syn::{File, Ident, Path, PathSegment, punctuated::Punctuated, spanned::Spanned, visit::Visit};

pub mod attr;
pub mod error;
pub mod expr;
pub mod generics;
pub mod global;
pub mod item;
pub mod macro_parsers;
pub mod pat;
pub mod path;
pub mod path_tracer;
pub mod statement;
pub mod structure;
pub mod ty;
pub mod visit;

pub trait Register: Sized {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>);

    fn register(&self, h: &mut RustHighlighter) {
        self.register_as(h, None);
    }
}

impl<T: Register> Register for Vec<T> {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        for item in self {
            item.register_as(h, _tag);
        }
    }
}

impl<T: Register> Register for Option<T> {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        if let Some(token) = self {
            token.register_as(h, _tag);
        }
    }
}

impl<T: Register> Register for Box<T> {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        self.as_ref().register_as(h, _tag);
    }
}

impl<T: Register, P> Register for Punctuated<T, P> {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        for item in self {
            item.register_as(h, _tag);
        }
    }
}
pub struct RustHighlighter {
    token_set: BTreeSet<SpannedToken>,
    unidentified: HashMap<usize, Path>,
    tracer: PathTracer,
}

impl RustHighlighter {
    pub fn highlight(&mut self, code: &str) -> String {
        let code = self.register_boring(code);

        let syntax_tree: File =
            syn::parse_str(&code).expect(&format!("Failed to parse Rust code\n{}", code));

        self.visit_file(&syntax_tree);
        self.register_comments(&code);
        self.write_tokens(code)
    }

    fn register(&mut self, token: &impl Register) {
        token.register(self);
    }

    fn register_as(&mut self, token: &impl Register, _tag: Option<Tag>) {
        token.register_as(self, _tag);
    }

    pub fn register_at(&mut self, start: usize, stop: usize, t: Option<Tag>) {
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

impl RustHighlighter {
    pub(crate) fn write_tokens(&mut self, code: String) -> String {
        let html_corrected = code; //.replace("<", "&lt;").replace(">", "&gt;");
        let mut output = Rope::from_str(&html_corrected);
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
        output.to_string()
    }

    pub(crate) fn remember_as(&mut self, p: &Path, tag: Tag) {
        self.tracer.map(p, tag);
    }

    /// Returns the identified token for ones the need identification, and for all others, None.
    pub(crate) fn identify_token(
        &self,
        token: &SpannedToken,
    ) -> Result<SpannedToken, IdentificationError> {
        match token.kind {
            None => {
                let unidentified = self.unidentified.get(&token.start);
                let p = match unidentified {
                    Some(segment) => segment,
                    None => return Err(IdentificationError::NoIdentificationNeeded),
                };

                let identified = self.tracer.get(p).unwrap_or(Tag::Type);

                Ok(SpannedToken {
                    kind: Some(identified),
                    start: token.start,
                    end: token.end,
                })
            }
            Some(_) => Err(IdentificationError::AlreadyIdentified),
        }
    }

    pub(crate) fn register_token(&mut self, token: &impl Spanned, _tag: Option<Tag>) {
        let range = token.span().byte_range();
        self.register_at(range.start, range.end, _tag);
    }

    pub(crate) fn register_ident(&mut self, token: &Ident, _tag: Option<Tag>) {
        if let Some(tag) = _tag {
            self.register_token(token, Some(tag));
            let mut segments = Punctuated::new();
            segments.push(PathSegment {
                ident: token.clone(),
                arguments: syn::PathArguments::None,
            });

            let p = Path {
                leading_colon: None,
                segments,
            };

            self.remember_as(&p, tag);
        }
    }

    pub(crate) fn register_unidentified_ident(&mut self, token: &Ident) {
        self.register_token(token, None);

        let mut segment = Punctuated::new();
        segment.push(PathSegment {
            ident: token.clone(),
            arguments: syn::PathArguments::None,
        });

        self.unidentified.insert(
            token.span().byte_range().start,
            Path {
                leading_colon: None,
                segments: segment,
            },
        );
    }

    pub(crate) fn register_comments(&mut self, code: &str) {
        let comment_regex: Regex = Regex::new(r"\/\/\/?.*\n?").unwrap();
        for comment in comment_regex.captures_iter(code) {
            let m = comment.get(0).unwrap();
            self.register_at(m.start(), m.end(), Some(Tag::Comment));
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
                self.register_at(start, end, Some(Tag::Boring));
            } else {
                output.push_str(line);
            }
            string_offset += line.len() - 2;
        }
        output
    }
}

impl RustHighlighter {
    pub fn new(tracer: PathTracer) -> Self {
        Self {
            token_set: BTreeSet::new(),
            unidentified: HashMap::new(),
            tracer,
        }
    }
}
