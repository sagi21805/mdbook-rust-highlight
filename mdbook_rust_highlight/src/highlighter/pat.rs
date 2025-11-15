use syn::{Pat, PatIdent, PatOr, PatReference, PatTuple, PatTupleStruct, PatType};

use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};

impl Register for Pat {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            Pat::Ident(token) => h.register_as(token, _tag),
            Pat::Reference(token) => h.register_as(token, _tag),
            Pat::Type(token) => h.register_as(token, _tag),
            Pat::Path(token) => h.register_as(token, _tag),
            Pat::Tuple(token) => h.register_as(token, _tag),
            Pat::TupleStruct(token) => h.register_as(token, _tag),
            Pat::Or(token) => h.register_as(token, _tag),
            Pat::Const(token) => h.register_as(token, _tag),
            _ => {}
        }
    }
}

impl Register for PatIdent {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.try_register_keyword_tag(self.by_ref.as_ref());
        h.try_register_keyword_tag(self.mutability.as_ref());
        h.register_variable_tag(&self.ident);
    }
}

impl Register for PatReference {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.try_register_keyword_tag(self.mutability.as_ref());
        h.register_as(&self.pat, _tag);
    }
}

impl Register for PatType {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_as(&self.pat, Some(Tag::Variable));
        h.register(&self.ty);
    }
}

impl Register for PatTuple {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.elems);
    }
}

impl Register for PatTupleStruct {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.qself);
        h.register_as(&self.path, Some(Tag::Enum));
        h.register(&self.elems);
    }
}

impl Register for PatOr {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.cases);
    }
}
