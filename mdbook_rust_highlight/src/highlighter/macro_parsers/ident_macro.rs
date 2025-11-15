use crate::{
    highlighter::{Register, RustHighlighter, macro_parsers::ident_list::IdentList},
    tokens::Tag,
};

impl Register for IdentList {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        for ident in &self.idents {
            h.register_ident(ident, _tag);
        }
    }
}
