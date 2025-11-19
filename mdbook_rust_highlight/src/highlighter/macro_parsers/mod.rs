use core::panic;

use proc_macro2::TokenStream;
use syn::{File, visit::Visit};

use crate::{
    highlighter::{
        Register, RustHighlighter,
        macro_parsers::{
            asm::AsmArgs,
            empty_macro_def::EmptyMacroDef,
            ident_list::{ExprList, IdentList},
        },
    },
    tokens::Tag,
};

pub mod asm;
pub mod empty_macro_def;
pub mod expr_macro;
pub mod ident_list;
pub mod ident_macro;

use proc_macro2::TokenTree;

pub fn remove_hash(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut output = TokenStream::new();

    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Punct(ref p) if p.as_char() == '#' => {
                let next_tt = iter
                    .next()
                    .expect("# should not be at the end of the stream");

                if let TokenTree::Ident(ident) = next_tt {
                    output.extend([TokenTree::Ident(ident)]);
                } else {
                    panic!("# should be followed by an Ident")
                }
            }
            other => output.extend([other]),
        }
    }

    output
}

impl Register for TokenStream {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        if let Some(tag) = _tag {
            match tag {
                Tag::MacroAsm => {
                    let args = syn::parse2::<AsmArgs>(self.clone()).unwrap();
                    h.register(&args)
                }
                Tag::MacroExpr => {
                    let args = syn::parse2::<ExprList>(self.clone()).unwrap();
                    h.register(&args)
                }
                Tag::MacroIdent => {
                    let args = syn::parse2::<IdentList>(self.clone()).unwrap();
                    h.register(&args)
                }
                Tag::MacroRulesCode => {
                    let args = syn::parse2::<EmptyMacroDef>(self.clone()).unwrap();
                    h.register(&args);
                }
                Tag::MacroCode => {
                    let args = syn::parse2::<File>(self.clone()).unwrap();
                    h.visit_file(&args);
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
