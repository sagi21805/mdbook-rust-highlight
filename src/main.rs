pub mod highlighter;
pub mod preprocessor;
pub mod tokens;

use mdbook::preprocess::Preprocessor;
use std::io;

use crate::preprocessor::RustHighlighterPreprocessor;

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
