use proc_macro2::TokenStream;

use crate::{
    highlighter::{Register, RustHighlighter, macro_parsers::asm::AsmArgs},
    tokens::Tag,
};

pub mod asm;
pub mod expr_macro;
pub mod ident_list;
pub mod ident_macro;

impl Register for TokenStream {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        if let Some(tag) = _tag {
            match tag {
                Tag::MacroAsm => {
                    let args = syn::parse2::<AsmArgs>(self.clone()).unwrap();
                    h.register(&args)
                }
                _ => {
                    eprintln!("{:?} is unsupported tag for TokenStream registration.", tag);
                }
            }
        } else {
            eprintln!("No macro was provided for TokenStream registration.");
        }
    }
}
