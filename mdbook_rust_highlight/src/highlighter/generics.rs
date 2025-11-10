use mdbook_rust_highlight_derive::add_try_method;
use syn::{
    AngleBracketedGenericArguments, BoundLifetimes, CapturedParam, ConstParam, GenericArgument,
    GenericParam, Generics, LifetimeParam, ParenthesizedGenericArguments, PreciseCapture,
    PredicateLifetime, PredicateType, TraitBound, TypeParam, TypeParamBound, WhereClause,
    WherePredicate,
    token::{self, Type},
};

use crate::highlighter::RustHighlighter;

impl<'a> RustHighlighter<'a> {
    pub(crate) fn register_generics(&mut self, token: &Generics) {
        for param in &token.params {
            self.register_generic_param(param);
        }
        if let Some(where_cluase) = &token.where_clause {
            self.register_where_clause(where_cluase);
        }
    }

    pub(crate) fn register_generic_param(&mut self, token: &GenericParam) {
        match token {
            GenericParam::Const(token) => {
                self.register_const_param(token);
            }
            GenericParam::Lifetime(token) => {
                self.register_lifetime_param(token);
            }
            GenericParam::Type(token) => {
                self.register_type_param(token);
            }
        }
    }

    pub(crate) fn register_lifetime_param(&mut self, token: &LifetimeParam) {
        self.register_attributes(&token.attrs);
        self.register_lifetime_tag(&token.lifetime);
        for lifetime in &token.bounds {
            self.register_lifetime_tag(lifetime);
        }
    }

    pub(crate) fn register_type_param(&mut self, token: &TypeParam) {
        self.register_attributes(&token.attrs);
        self.register_type_tag(&token.ident);
        for bound in &token.bounds {
            self.register_type_param_bound(bound);
        }
        if let Some(default) = &token.default {
            self.register_type(default);
        }
    }

    pub(crate) fn register_const_param(&mut self, token: &ConstParam) {
        self.register_attributes(&token.attrs);
        self.register_keyword_tag(&token.const_token);
        self.register_const_tag(&token.ident);
        self.register_type(&token.ty);
        if let Some(default) = &token.default {
            self.register_expr(default, None);
        }
    }

    pub(crate) fn register_where_clause(&mut self, token: &WhereClause) {
        self.register_keyword_tag(&token.where_token);
        for item in &token.predicates {
            self.register_where_predicate(item);
        }
    }

    pub(crate) fn register_predicate_lifetime(&mut self, token: &PredicateLifetime) {
        self.register_lifetime_tag(&token.lifetime);
        for lifetime in &token.bounds {
            self.register_lifetime_tag(lifetime);
        }
    }

    #[add_try_method]
    pub(crate) fn register_bound_lifetimes(&mut self, token: &BoundLifetimes) {
        self.register_keyword_tag(&token.for_token);
        for lifetime in &token.lifetimes {
            self.register_generic_param(lifetime);
        }
    }

    pub(crate) fn register_percise_capture(&mut self, token: &PreciseCapture) {
        self.register_keyword_tag(&token.use_token);
        for param in &token.params {
            self.register_capture_param(param);
        }
    }

    pub(crate) fn register_type_param_bound(&mut self, token: &TypeParamBound) {
        match token {
            TypeParamBound::Lifetime(token) => {
                self.register_lifetime_tag(token);
            }
            TypeParamBound::PreciseCapture(token) => {
                self.register_percise_capture(token);
            }
            TypeParamBound::Trait(token) => {
                self.register_trait_bound(token);
            }
            TypeParamBound::Verbatim(_) => {}
            _ => {
                todo!()
            }
        }
    }

    pub(crate) fn register_predicate_type(&mut self, token: &PredicateType) {
        self.try_register_bound_lifetimes(token.lifetimes.as_ref());
        self.register_type(&token.bounded_ty);
        for bound in &token.bounds {
            self.register_type_param_bound(bound);
        }
    }

    pub(crate) fn register_where_predicate(&mut self, token: &WherePredicate) {
        match token {
            WherePredicate::Lifetime(token) => {
                self.register_predicate_lifetime(token);
            }
            WherePredicate::Type(token) => {
                self.register_predicate_type(token);
            }
            _ => {}
        }
    }

    pub(crate) fn register_capture_param(&mut self, token: &CapturedParam) {
        match token {
            CapturedParam::Ident(token) => {
                self.register_variable_tag(token);
            }
            CapturedParam::Lifetime(token) => {
                self.register_lifetime_tag(token);
            }
            _ => {}
        }
    }
    pub(crate) fn register_trait_bound(&mut self, token: &TraitBound) {}

    pub(crate) fn register_precise_capture(&mut self, token: &PreciseCapture) {
        self.register_keyword_tag(&token.use_token);
        for param in &token.params {
            self.register_capture_param(param);
        }
    }

    pub(crate) fn register_bound(&mut self, token: &TypeParamBound) {
        match token {
            TypeParamBound::Lifetime(token) => {
                self.register_lifetime_tag(token);
            }
            TypeParamBound::PreciseCapture(token) => {
                self.register_precise_capture(token);
            }
            TypeParamBound::Trait(token) => {
                self.register_trait_bound(token);
            }
            _ => {}
        }
    }

    pub(crate) fn register_generic_argument(&mut self, token: &GenericArgument) {
        match token {
            GenericArgument::Type(token) => {
                self.register_type(token);
            }
            GenericArgument::Lifetime(token) => {
                self.register_lifetime_tag(token);
            }
            _ => {}
        }
    }

    pub(crate) fn register_parenthesized_arg(&mut self, token: &ParenthesizedGenericArguments) {
        for input in &token.inputs {
            self.register_type(input);
        }
        self.register_return_type(&token.output);
    }

    pub(crate) fn register_angle_brackets_arg(&mut self, token: &AngleBracketedGenericArguments) {
        for arg in &token.args {
            self.register_generic_argument(arg);
        }
    }
}
