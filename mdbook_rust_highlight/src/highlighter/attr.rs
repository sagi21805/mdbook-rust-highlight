use syn::{AttrStyle, Attribute, Ident, Meta, Token, parse::Parse, punctuated::Punctuated};

use crate::{highlighter::RustHighlighter, tokens::TokenTag};

impl<'a, 'ast> RustHighlighter<'a, 'ast> {
    pub(crate) fn register_attributes(&mut self, token: &'ast Vec<Attribute>) {
        for attr in token {
            self.register_attribute(attr);
        }
    }

    pub(crate) fn register_attribute(&mut self, token: &'ast Attribute) {
        // Distinguish between a comment or docs to an attribute.
        if token.pound_token.span.byte_range().len() == 1 {
            self.register_attribute_tag(&token.pound_token);
            self.register_attribute_tag(&token.bracket_token.span.join());
        }
        match token.style {
            AttrStyle::Outer => match &token.meta {
                Meta::Path(path) => {
                    self.register_path(path, Some(TokenTag::Const));
                }
                Meta::List(list) => {
                    if list.path.get_ident().unwrap().to_string() == "unsafe" {
                        self.register_keyword_tag(&list.path);
                    } else {
                        self.register_path(&list.path, Some(TokenTag::Const));
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
                Meta::NameValue(name_val) => {
                    self.register_path(&name_val.path, Some(TokenTag::Ident));
                    self.register_expr(&name_val.value);
                }
            },
            AttrStyle::Inner(not) => {
                self.register_attribute_tag(&not);
                match &token.meta {
                    Meta::Path(path) => {
                        self.register_path(path, Some(TokenTag::Const));
                    }
                    _ => {}
                }
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
