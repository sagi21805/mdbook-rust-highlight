use syn::{
    AngleBracketedGenericArguments, BoundLifetimes, CapturedParam, ConstParam, GenericArgument,
    GenericParam, Generics, LifetimeParam, ParenthesizedGenericArguments, PreciseCapture,
    PredicateLifetime, PredicateType, TraitBound, TypeParam, TypeParamBound, WhereClause,
    WherePredicate,
};

use crate::{
    highlighter::{Register, RustHighlighter},
    tokens::Tag,
};

impl Register for Generics {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.params);
        h.register(&self.where_clause);
    }
}

impl Register for GenericParam {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            GenericParam::Const(token) => h.register_as(token, _tag),
            GenericParam::Lifetime(token) => h.register_as(token, _tag),
            GenericParam::Type(token) => h.register_as(token, _tag),
        }
    }
}

impl Register for LifetimeParam {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_lifetime_tag(&self.lifetime);
        for lifetime in &self.bounds {
            h.register_lifetime_tag(lifetime);
        }
    }
}

impl Register for TypeParam {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_type_tag(&self.ident);
        h.register(&self.bounds);
        h.register(&self.default);
    }
}

impl Register for ConstParam {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.attrs);
        h.register_keyword_tag(&self.const_token);
        h.register_const_tag(&self.ident);
        h.register(&self.ty);
        h.register(&self.default)
    }
}

impl Register for WhereClause {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register_keyword_tag(&self.where_token);
        h.register(&self.predicates);
    }
}

impl Register for PredicateLifetime {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register_lifetime_tag(&self.lifetime);
        for lifetime in &self.bounds {
            h.register_lifetime_tag(lifetime);
        }
    }
}

impl Register for BoundLifetimes {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register_keyword_tag(&self.for_token);
        h.register(&self.lifetimes);
    }
}

impl Register for PreciseCapture {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register_keyword_tag(&self.use_token);
        h.register(&self.params);
    }
}

impl Register for TypeParamBound {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            TypeParamBound::Lifetime(token) => h.register_lifetime_tag(token),
            TypeParamBound::PreciseCapture(token) => h.register_as(token, _tag),
            TypeParamBound::Trait(token) => h.register_as(token, _tag),
            TypeParamBound::Verbatim(_) => {}
            _ => todo!(),
        }
    }
}

impl Register for PredicateType {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.lifetimes);
        h.register(&self.bounded_ty);
        h.register(&self.bounds);
    }
}

impl Register for WherePredicate {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            WherePredicate::Lifetime(token) => h.register_as(token, _tag),
            WherePredicate::Type(token) => h.register_as(token, _tag),
            _ => {}
        }
    }
}

impl Register for CapturedParam {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            CapturedParam::Ident(token) => h.register_variable_tag(token),
            CapturedParam::Lifetime(token) => h.register_lifetime_tag(token),
            _ => {}
        }
    }
}

impl Register for TraitBound {
    fn register_as(&self, _h: &mut RustHighlighter, _tag: Option<Tag>) {
        todo!()
    }
}

impl Register for GenericArgument {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        match self {
            GenericArgument::Type(token) => h.register_as(token, _tag),
            GenericArgument::Lifetime(token) => h.register_lifetime_tag(token),
            _ => {}
        }
    }
}

impl Register for ParenthesizedGenericArguments {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.inputs);
        h.register(&self.output);
    }
}

impl Register for AngleBracketedGenericArguments {
    fn register_as(&self, h: &mut RustHighlighter, _tag: Option<Tag>) {
        h.register(&self.args);
    }
}
