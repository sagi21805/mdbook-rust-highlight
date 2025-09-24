use syn::{
    AngleBracketedGenericArguments, CapturedParam, GenericArgument, ParenthesizedGenericArguments,
    PreciseCapture, TraitBound, TypeParamBound,
};

use crate::highlighter::RustHighlighter;

impl<'ast> RustHighlighter<'ast> {
    pub(crate) fn register_capture_param(&mut self, token: &'ast CapturedParam) {
        match token {
            CapturedParam::Ident(token) => {
                self.register_ident_tag(token);
            }
            CapturedParam::Lifetime(token) => {
                self.register_lifetime_tag(token);
            }
            _ => {}
        }
    }
    pub(crate) fn register_trait_bound(&mut self, token: &'ast TraitBound) {}

    pub(crate) fn register_precise_capture(&mut self, token: &'ast PreciseCapture) {
        self.register_keyword_tag(&token.use_token);
        for param in &token.params {
            self.register_capture_param(param);
        }
    }

    pub(crate) fn register_bound(&mut self, token: &'ast TypeParamBound) {
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

    pub(crate) fn register_generic_argument(&mut self, token: &'ast GenericArgument) {
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

    pub(crate) fn register_parenthesized_arg(
        &mut self,
        token: &'ast ParenthesizedGenericArguments,
    ) {
        for input in &token.inputs {
            self.register_type(input);
        }
        self.register_return_type(&token.output);
    }

    pub(crate) fn register_angle_brackets_arg(
        &mut self,
        token: &'ast AngleBracketedGenericArguments,
    ) {
        for arg in &token.args {
            self.register_generic_argument(arg);
        }
    }
}
