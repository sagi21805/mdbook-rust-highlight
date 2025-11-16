use syn::{AttrStyle, Attribute, Meta, MetaNameValue};

use crate::{
    highlighter::{Register, RustHighlighter, macro_parsers::ident_list::ExprList},
    tokens::Tag,
};

impl Register for Attribute {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        // Distinguish between a comment or docs to an attribute.
        if self.pound_token.span.byte_range().len() == 1 {
            h.register_attribute_tag(&self.pound_token);
            h.register_attribute_tag(&self.bracket_token.span.join());
        } else {
            // If we are this means the attribute is a comment, so we return.
            return;
        }
        match self.style {
            AttrStyle::Outer => match &self.meta {
                Meta::Path(path) => {
                    h.register_as(path, Some(Tag::Const));
                }
                Meta::List(list) => {
                    if list.path.get_ident().unwrap().to_string() == "unsafe" {
                        h.register_keyword_tag(&list.path);
                    } else {
                        h.register_as(&list.path, Some(Tag::Const));
                    }
                    let ident_list = syn::parse2::<ExprList>(list.tokens.clone());
                    match ident_list {
                        Ok(list) => {
                            for item in &list.exprs {
                                h.register(item);
                            }
                        }
                        _ => {}
                    }
                    let name_val = syn::parse2::<MetaNameValue>(list.tokens.clone());
                    match name_val {
                        Ok(name_val) => {
                            h.register_variable_tag(&name_val.path.get_ident().unwrap());
                            h.register_litstr_tag(&name_val.value);
                        }
                        _ => {}
                    }
                }
                Meta::NameValue(name_val) => {
                    h.register_as(&name_val.path, Some(Tag::Variable));
                    h.register(&name_val.value);
                }
            },
            AttrStyle::Inner(not) => {
                h.register_attribute_tag(&not);
                match &self.meta {
                    Meta::Path(path) => {
                        h.register_as(path, Some(Tag::Const));
                    }
                    _ => {}
                }
            }
        }
    }
}
