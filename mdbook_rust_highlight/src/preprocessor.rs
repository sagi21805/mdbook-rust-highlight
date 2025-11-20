use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use crate::{highlighter::RustHighlighter, tokens::Tag};
use mdbook::{
    BookItem, Config,
    book::{Book, Chapter},
    preprocess::{Preprocessor, PreprocessorContext},
};
use regex::Regex;
use ropey::Rope;
use syn::Ident;

pub struct RustHighlighterPreprocessor;

const HLRS_CODEBLOCK_REGEX: &str = r"```rust(?:,?([^\n]+))?\n([\s\S]*?)\n?```";
const RUST_ICON_URL: &str = "@https://www.rust-lang.org/static/images/rust-logo-blk.svg";

pub type IdentMap = HashMap<&'static str, Tag>;

impl Preprocessor for RustHighlighterPreprocessor {
    fn name(&self) -> &str {
        "rust-highlight"
    }
    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> mdbook::errors::Result<Book> {
        let mut ident_map = self
            .process_configuration(ctx)
            .expect("Invalid Configuration");

        // Maybe turn into an initialize function.
        ident_map.insert("Ok", Tag::Enum);
        ident_map.insert("Some", Tag::Enum);
        ident_map.insert("None", Tag::Enum);
        ident_map.insert("Err", Tag::Enum);
        ident_map.insert("self", Tag::SelfToken);
        ident_map.insert("Self", Tag::SelfToken);

        let mut highlighter = RustHighlighter::new(ident_map);

        // Regex matches entire Rust code blocks including fences
        let block_pat = Regex::new(HLRS_CODEBLOCK_REGEX).unwrap();
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                let registered_blocks =
                    self.register_chapter(ctx, chapter, &block_pat, &mut highlighter);

                Self::write_codeblock(chapter, registered_blocks);
            }
        });
        Ok(book)
    }
}

impl RustHighlighterPreprocessor {
    fn register_chapter(
        &self,
        ctx: &PreprocessorContext,
        chapter: &Chapter,
        pattern: &Regex,
        highlighter: &mut RustHighlighter,
    ) -> BTreeMap<usize, (usize, String)> {
        const GROUP_FULL: usize = 0;
        const GROUP_FEATURES: usize = 1;
        const GROUP_CODE: usize = 2;

        let mut chap_replacement = BTreeMap::new();

        for caps in pattern.captures_iter(&chapter.content) {
            let full = caps.get(GROUP_FULL).unwrap();
            let code_match = match caps.get(GROUP_CODE) {
                Some(m) => m,
                None => continue,
            };
            let features = self.whichlang_features(ctx, caps.get(GROUP_FEATURES));
            let code = code_match.as_str();
            let highlighted = highlighter.highlight(code);
            let html =
                format!("<pre><code class=\"language-hlrs {features}\">{highlighted}</code></pre>");
            chap_replacement.insert(full.start(), (full.end(), html));
        }
        chap_replacement
    }

    fn write_codeblock(chapter: &mut Chapter, registered_blocks: BTreeMap<usize, (usize, String)>) {
        let mut chap_rope = Rope::from_str(&chapter.content);
        let mut offset = 0;
        for (start, (end, replacement)) in registered_blocks {
            chap_rope.remove((start + offset)..(end + offset));
            chap_rope.insert(start + offset, &replacement);
            offset += replacement.len() - (end - start);
        }
        chapter.content = chap_rope.to_string();
    }

    fn whichlang_features<'a>(
        &self,
        ctx: &PreprocessorContext,
        f: Option<regex::Match<'a>>,
    ) -> String {
        if let Some(cfg) = ctx.config.get(&format!("preprocessor.{}", self.name())) {
            match cfg.get("whichlang") {
                Some(feature) => feature
                    .as_bool()
                    .expect("\nERROR: `whichlang` configuration should be a boolean"),
                None => return String::from(""),
            };
        }

        let mut feature_string = match f {
            Some(feature) => feature.as_str().replace(',', " "),
            None => String::from(""),
        };
        if !feature_string.contains("icon=@https://") {
            feature_string.push_str(" icon=");
            feature_string.push_str(RUST_ICON_URL);
        }
        return feature_string;
    }

    fn process_configuration(&self, ctx: &PreprocessorContext) -> Option<IdentMap> {
        let cfg = ctx.config.get(&format!("preprocessor.{}", self.name()))?;
        let mapping = cfg.get("mapping")?.as_table()?;
        let ident_map = mapping
            .iter()
            .map(|(k, v)| {
                let leaked: &'static str = k.clone().leak();
                let tag =
                    Tag::from_str(v.as_str().expect("Tag in not string")).expect("Tag is no valid");
                (leaked, tag)
            })
            .collect::<IdentMap>();
        Some(ident_map)
    }
}
