use syn::{
    FnArg, ImplItem, Item, ItemEnum, ItemFn, ItemImpl, ItemMacro, ItemStruct, ItemUse, Macro,
    Signature, UseTree, Visibility,
};

use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};

impl Register for Item {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            Item::Fn(token) => h.register(token),
            Item::Enum(token) => h.register(token),
            Item::Use(token) => h.register(token),
            Item::Macro(token) => h.register(token),
            Item::Impl(token) => h.register(token),
            Item::Struct(token) => h.register(token),
            Item::Static(token) => h.register(token),
            Item::Const(token) => h.register(token),
            _ => {}
        }
    }
}

impl Register for ItemStruct {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.vis);
        h.register_keyword_tag(&self.struct_token);
        h.register_type_tag(&self.ident);
        h.register(&self.fields);
    }
}

impl Register for Signature {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.try_register_keyword_tag(self.constness.as_ref());
        h.try_register_keyword_tag(self.asyncness.as_ref());
        h.try_register_keyword_tag(self.unsafety.as_ref());
        if let Some(abi) = &self.abi {
            h.register_keyword_tag(&abi.extern_token);
            h.try_register_litstr_tag(abi.name.as_ref());
        }
        h.register_keyword_tag(&self.fn_token);
        h.register_function_tag(&self.ident);

        for input in &self.inputs {
            match input {
                FnArg::Receiver(arg) => {
                    h.register_selftoken_tag(&arg.self_token);
                    h.try_register_keyword_tag(arg.mutability.as_ref());
                    h.try_register_lifetime_tag(arg.lifetime());
                }
                FnArg::Typed(type_pat) => {
                    h.register(type_pat);
                    h.register(&type_pat.ty);
                }
            }
        }

        h.register(&self.output);

        eprintln!("at signature")
    }
}

impl Register for ItemFn {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        eprintln!("at fn");
        h.register(&self.attrs);
        h.register(&self.vis);
        h.register(&self.sig);
        h.register(&self.block);
    }
}

impl Register for ItemEnum {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register(&self.vis);
        h.register_keyword_tag(&self.enum_token);
        h.register_type_tag(&self.ident);
        // TODO REGISTER GENERICS AND FIELDS
        for variant in &self.variants {
            h.register_enum_tag(&variant.ident);
            if let Some((_, discriminant)) = &variant.discriminant {
                h.register(discriminant);
            }
        }
    }
}

impl Register for ItemUse {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.vis);
        h.register_keyword_tag(&self.use_token);
        h.register(&self.tree);
    }
}
impl Register for UseTree {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            UseTree::Glob(_) => {}
            UseTree::Group(token) => {
                for tree in &token.items {
                    h.register(tree);
                }
            }
            UseTree::Path(token) => {
                h.register_segment_tag(&token.ident);
                h.register(&token.tree);
            }
            UseTree::Name(token) => {
                h.register_unidentified((&token.ident).into());
            }
            UseTree::Rename(token) => {
                h.register_segment_tag(&token.ident);
                h.register_keyword_tag(&token.as_token);
                h.register_segment_tag(&token.rename);
            }
        }
    }
}

impl Register for ItemMacro {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        eprintln!("here at macro");
        if let Some(name) = self.ident.as_ref() {
            h.register_ident(name.into(), Some(Tag::Macro));
        }
        h.register(&self.mac);
    }
}

impl Register for Macro {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        let mut macro_tag = Tag::Macro;
        if let Some(ident) = self.path.get_ident() {
            match ident.to_string().as_str() {
                "macro_rules" => macro_tag = Tag::Keyword,
                "asm" => {
                    h.register_as(&self.tokens, Some(Tag::MacroAsm));
                }
                _ => {}
            }
        }
        h.register_as(&self.path, Some(macro_tag));
        h.register_macro_tag(&self.bang_token);
    }
}

impl Register for ItemImpl {
    // TODO COMPLETE
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.try_register_keyword_tag(self.unsafety.as_ref());
        h.register_keyword_tag(&self.impl_token);
        if let Some((_, trait_name, for_token)) = &self.trait_ {
            h.register_as(trait_name, Some(Tag::Type));
            h.register_keyword_tag(for_token);
        }
        h.register(&self.self_ty);
        h.register(&self.items);
    }
}

impl Register for ImplItem {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            ImplItem::Const(_token) => {}
            ImplItem::Fn(token) => {
                h.register(&token.attrs);
                h.register(&token.vis);
                h.register(&token.sig);
                h.register(&token.block);
            }
            ImplItem::Macro(token) => {
                h.register(&token.mac);
            }
            ImplItem::Type(_token) => {}
            ImplItem::Verbatim(_) => {}
            _ => {}
        }
    }
}

impl Register for Visibility {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            Visibility::Inherited => {}
            _ => h.register_keyword_tag(self),
        }
        eprintln!("here visibility");
    }
}
