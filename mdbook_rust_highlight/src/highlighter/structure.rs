use syn::{Field, FieldValue, Fields, FieldsNamed, FieldsUnnamed, Member};

use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};

impl Register for Fields {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            Fields::Unit => {}
            Fields::Named(token) => h.register(token),
            Fields::Unnamed(token) => h.register(token),
        }
    }
}

impl Register for FieldsNamed {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.named);
    }
}

impl Register for FieldsUnnamed {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.unnamed);
    }
}

impl Register for Field {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.vis);
        h.try_register_variable_tag(self.ident.as_ref());
        h.register(&self.ty);
    }
}

impl Register for FieldValue {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match &self.member {
            Member::Named(token) => h.register_variable_tag(token),
            Member::Unnamed(_) => {}
        }
        h.register(&self.expr);
    }
}
