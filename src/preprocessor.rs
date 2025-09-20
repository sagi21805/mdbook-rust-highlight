use std::collections::BTreeMap;

use mdbook::{
    BookItem,
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
};
use regex::Regex;
use ropey::Rope;

use crate::highlighter::RustHighlighter;

pub struct RustHighlighterPreprocessor;

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
                    let highlighted = RustHighlighter::highlight_rust_code(code);
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
