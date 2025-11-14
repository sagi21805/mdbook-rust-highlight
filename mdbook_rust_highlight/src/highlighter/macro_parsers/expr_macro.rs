use crate::{
    highlighter::{Register, RustHighlighter, macro_parsers::ident_list::ExperList},
    tokens::Tag,
};

impl Register for ExperList {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.exprs);
    }
}
