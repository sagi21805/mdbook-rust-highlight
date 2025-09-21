use std::collections::BTreeMap;

use mdbook::{
    BookItem,
    book::{Book, Chapter},
    preprocess::{Preprocessor, PreprocessorContext},
};
use regex::Regex;
use ropey::Rope;

use crate::highlighter::RustHighlighter;

pub struct RustHighlighterPreprocessor;

const HLRS_CODEBLOCK_REGEX: &str = r"```hlrs(?:,([^\n]*))?\n([\s\S]*?)\n```";
const RUST_ICON_URL: &str = "@https://www.rust-lang.org/static/images/rust-logo-blk.svg";

impl Preprocessor for RustHighlighterPreprocessor {
    fn name(&self) -> &str {
        "rust-highlight"
    }
    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> mdbook::errors::Result<Book> {
        // Regex matches entire Rust code blocks including fences
        let block_pat = Regex::new(HLRS_CODEBLOCK_REGEX).unwrap();
        for item in &mut book.sections {
            if let BookItem::Chapter(chapter) = item {
                let registered_blocks = self.register_codeblock(ctx, chapter, &block_pat);
                Self::write_codeblock(chapter, registered_blocks);
            }
        }
        Ok(book)
    }
}

impl RustHighlighterPreprocessor {
    fn register_codeblock(
        &self,
        ctx: &PreprocessorContext,
        chapter: &Chapter,
        pattern: &Regex,
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
            let highlighted = RustHighlighter::highlight(code);
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
}
