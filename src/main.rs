use mdbook::BookItem;
use mdbook::book::Book;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::{Regex, Replacer};
use ropey::Rope;
use serde_json::de;
use std::{collections::BTreeMap, io};
use strum_macros::AsRefStr;
use syn::{File, FnArg, spanned::Spanned, visit::Visit};

/// Token mapping with
#[derive(AsRefStr)]
pub enum TokenTag {
    Visibility,
    Abi,
    Fn,
    EndOfToken,
    Extern,
    Asyncness,
    Constness,
    Unsafety,
    FnName,
    FnArg,
    SelfToken,
    Variadic,
    FnType,
    LifeTime,
}

impl ToString for TokenTag {
    fn to_string(&self) -> String {
        match self {
            Self::EndOfToken => String::from("</span>"),
            _ => format!("<span class=\"hlrs-{}\">", self.as_ref()),
        }
    }
}

struct Highlighter {
    output: Rope,
    token_map: BTreeMap<usize, TokenTag>,
}

impl<'ast> Visit<'ast> for Highlighter {
    fn visit_signature(&mut self, i: &'ast syn::Signature) {
        if let Some(abi) = &i.abi {
            self.insert_token(abi.extern_token, TokenTag::Extern);
            self.try_insert_token(abi.name.clone(), TokenTag::Abi);
        }
        self.try_insert_token(i.asyncness, TokenTag::Asyncness);
        self.try_insert_token(i.constness, TokenTag::Constness);
        self.try_insert_token(i.unsafety, TokenTag::Unsafety);
        self.try_insert_token(i.variadic.clone(), TokenTag::Variadic);
        self.insert_token(i.fn_token, TokenTag::Fn);
        self.insert_token(i.ident.clone(), TokenTag::FnName);
        for input in &i.inputs {
            match input {
                FnArg::Receiver(arg) => {
                    self.insert_token(arg.self_token, TokenTag::SelfToken);
                    self.insert_token(arg.lifetime(), TokenTag::LifeTime);
                }
                FnArg::Typed(arg) => {
                    self.insert_token(arg.pat.clone(), TokenTag::FnArg);
                    self.insert_token(arg.ty.clone(), TokenTag::FnType);
                }
            }
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

    fn try_insert_token(&mut self, token: Option<impl Spanned>, tag: TokenTag) {
        if let Some(t) = token {
            self.insert_token(t, tag);
        }
    }

    fn highlight_rust_code(code: &str) -> String {
        let syntax_tree: File =
            syn::parse_str(code).expect(&format!("Failed to parse Rust code {}", code));

        let mut highlighter = Highlighter {
            output: Rope::from_str(code),
            token_map: BTreeMap::new(),
        };

        highlighter.visit_file(&syntax_tree);
        highlighter.write_tokens();
        highlighter.output.to_string()
    }
}

struct RustHighlighterPreprocessor;

impl Preprocessor for RustHighlighterPreprocessor {
    fn name(&self) -> &str {
        "rust-highlight"
    }
    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> mdbook::errors::Result<Book> {
        // Regex matches entire Rust code blocks including fences
        let rust_block_regex = Regex::new(r"```hlrs(?:,([^\n]*))?\n([\s\S]*?)\n```").unwrap();

        for item in &mut book.sections {
            if let BookItem::Chapter(chapter) = item {
                const FULL: usize = 0;
                const WHICHLANG_FEATURES: usize = 1;
                const CODE: usize = 2;

                // map from regex start pos, (before end, after str)
                let mut regex_map: BTreeMap<usize, (usize, String)> = BTreeMap::new();
                let mut chapter_rope = Rope::from_str(&chapter.content);
                for m in rust_block_regex.captures_iter(&chapter.content) {
                    let start_position = m.get(FULL).unwrap().start();
                    let end_position = m.get(FULL).unwrap().end();
                    let code_capture = match m.get(CODE) {
                        Some(c) => c,
                        None => continue,
                    };
                    let features = match self.whichlang_features(ctx, m.get(WHICHLANG_FEATURES)) {
                        Some(feature_map) => {
                            let mut feature_string = String::from("");
                            for (feature, value) in feature_map {
                                feature_string.push_str(&format!("{feature}={value} "));
                            }
                            eprintln!("{}", feature_string);
                            feature_string
                        }
                        None => String::from(""),
                    };

                    let code = code_capture.as_str();
                    let highlighted = Highlighter::highlight_rust_code(code);
                    let html = format!(
                        "<pre><code class=\"language-hlrs {}\">{}</code></pre>",
                        features, highlighted
                    );
                    regex_map.insert(start_position, (end_position, html));
                }

                let mut offset = 0;
                for (start, (end, after)) in regex_map {
                    chapter_rope.remove((start + offset)..(end + offset));
                    chapter_rope.insert(start + offset, &after);
                    offset += after.len() - (end - start);
                }

                chapter.content = chapter_rope.to_string();
            }
        }

        Ok(book)
    }
}

impl RustHighlighterPreprocessor {
    fn whichlang_features<'a>(
        &self,
        ctx: &PreprocessorContext,
        f: Option<regex::Match<'a>>,
    ) -> Option<BTreeMap<&'a str, String>> {
        if let Some(cfg) = ctx.config.get(&format!("preprocessor.{}", self.name())) {
            cfg.get("whichlang")?
                .as_bool()
                .expect("\nERROR: whichlang configuration should be a boolean");
        }
        let mut default: BTreeMap<&str, String> = BTreeMap::new();
        default.insert(
            "icon",
            String::from("@https://www.rust-lang.org/static/images/rust-logo-blk.svg"),
        );

        let features = match f {
            Some(feature) => feature.as_str().split(','),
            None => return Some(default),
        };
        for feature in features {
            let feature_parts: Vec<&str> = feature.split('=').collect();
            let feat = feature_parts[0];
            let val = String::from(feature_parts[1]);
            default.insert(feat, val);
        }
        Some(default)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let preprocessor = RustHighlighterPreprocessor;

    // Handle mdbook commands: `supports` or `preprocess
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "supports" {
        let renderer = &args[2];
        std::process::exit(if preprocessor.supports_renderer(renderer) {
            0
        } else {
            1
        });
    }

    // Read JSON book from stdin
    let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(io::stdin())?;
    let book = preprocessor.run(&ctx, book)?;

    // Write modified book to stdout
    serde_json::to_writer(io::stdout(), &book)?;
    Ok(())
}
