use syn::{AttrStyle, Attribute, Ident, Meta, Token, parse::Parse, punctuated::Punctuated};

use crate::{highlighter::RustHighlighter, tokens::TokenTag};

impl<'a, 'ast> RustHighlighter<'a, 'ast> {
    pub(crate) fn register_attributes(&mut self, token: &'ast Vec<Attribute>) {
        for attr in token {
            self.register_attribute_tag(attr);
            match attr.style {
                AttrStyle::Outer => match &attr.meta {
                    Meta::Path(path) => {
                        self.register_path(path, Some(TokenTag::Function));
                    }
                    Meta::List(list) => {
                        if list.path.get_ident().unwrap().to_string() == "unsafe" {
                            self.register_keyword_tag(&list.path);
                        } else {
                            self.register_path(&list.path, Some(TokenTag::Function));
                        }
                        let ident_list = syn::parse2::<IdentList>(list.tokens.clone());
                        match ident_list {
                            Ok(list) => {
                                for item in &list.idents {
                                    self.register_type_tag(item);
                                }
                            }
                            _ => {}
                        }
                    }
                    Meta::NameValue(name_val) => {}
                },
                AttrStyle::Inner(_) => {}
            }
        }
    }
}

struct IdentList {
    idents: Punctuated<Ident, Token![,]>,
}

impl Parse for IdentList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let idents = input.parse_terminated(Ident::parse, Token![,])?;
        Ok(IdentList { idents })
    }
}
