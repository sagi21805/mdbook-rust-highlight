use syn::{ReturnType, Type, TypeImplTrait, TypePath, TypePtr, TypeReference, TypeTuple};

use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};

impl Register for Type {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            Type::Reference(token) => h.register_as(token, _tag),
            Type::Path(token) => h.register_as(token, _tag),
            Type::Tuple(token) => h.register_as(token, _tag),
            Type::Ptr(token) => h.register_as(token, _tag),
            Type::ImplTrait(token) => h.register_as(token, _tag),
            _ => {}
        }
    }
}

impl Register for TypePtr {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register_keyword_tag(&self.star_token);
        h.try_register_keyword_tag(self.const_token.as_ref());
        h.try_register_keyword_tag(self.mutability.as_ref());
        h.register(&self.elem);
    }
}

impl Register for TypeReference {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.try_register_lifetime_tag(self.lifetime.as_ref());
        h.try_register_keyword_tag(self.mutability.as_ref());
        h.register(&self.elem);
    }
}

impl Register for TypePath {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.qself);
        h.register_as(&self.path, Some(Tag::Type));
    }
}

impl Register for TypeTuple {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.elems);
    }
}

impl Register for TypeImplTrait {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register_keyword_tag(&self.impl_token);
        h.register(&self.bounds);
    }
}

impl Register for ReturnType {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            ReturnType::Default => {}
            ReturnType::Type(_, token) => h.register_as(token, _tag),
        }
    }
}
