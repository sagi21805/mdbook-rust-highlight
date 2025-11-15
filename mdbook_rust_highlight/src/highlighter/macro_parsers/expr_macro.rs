use crate::{
    highlighter::{Register, RustHighlighter, macro_parsers::ident_list::ExprList},
    tokens::Tag,
};

impl Register for ExprList {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.exprs);
    }
}
