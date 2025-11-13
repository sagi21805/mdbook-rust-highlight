use syn::{ItemConst, ItemStatic, StaticMutability};

use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};

impl Register for ItemStatic {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.vis);
        h.register_keyword_tag(&self.static_token);
        if let StaticMutability::Mut(mut_token) = &self.mutability {
            h.register_keyword_tag(mut_token);
        }
        h.register_variable_tag(&self.ident);
        h.register(&self.ty);
        h.register(&self.expr);
    }
}

impl Register for ItemConst {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.vis);
        h.register_keyword_tag(&self.const_token);
        h.register_const_tag(&self.ident);
        h.register(&self.generics);
        h.register(&self.ty);
        h.register(&self.expr);
    }
}
