use std::{
    any::Any,
    collections::{BTreeMap, HashMap},
    net::TcpListener,
};

use crate::tokens::TokenTag;
use proc_macro2::{Span, TokenTree};
use regex::Regex;
use ropey::Rope;
use syn::{
    AngleBracketedGenericArguments, Arm, Block, CapturedParam, Expr, ExprBinary, ExprBlock,
    ExprCall, ExprCast, ExprField, ExprForLoop, ExprIf, ExprLit, ExprMatch, ExprMethodCall,
    ExprParen, ExprPath, ExprReference, ExprTry, ExprTuple, ExprUnary, ExprUnsafe, File, FnArg,
    GenericArgument, Ident, Item, Lit, LitStr, Local, LocalInit, Member,
    ParenthesizedGenericArguments, Pat, PatIdent, PatOr, PatParen, PatReference, PatTuple,
    PatTupleStruct, PatType, Path, PathArguments, PathSegment, PreciseCapture, QSelf, ReturnType,
    Stmt, StmtMacro, TraitBound, Type, TypeImplTrait, TypeParamBound, TypePath, TypeReference,
    TypeTuple, spanned::Spanned, token::Token, visit::Visit,
};

use mdbook_rust_highlight_derive::add_try_method;

pub struct RustHighlighter<'ast> {
    token_map: BTreeMap<usize, TokenTag>,
    unidentified: HashMap<usize, &'ast PathSegment>,
}

impl<'ast> Visit<'ast> for RustHighlighter<'ast> {
    fn visit_signature(&mut self, i: &'ast syn::Signature) {
        self.try_register_keyword_tag(i.constness.as_ref());
        self.try_register_keyword_tag(i.asyncness.as_ref());
        self.try_register_keyword_tag(i.unsafety.as_ref());
        if let Some(abi) = &i.abi {
            self.register_keyword_tag(&abi.extern_token);
            self.try_register_litstr_tag(abi.name.as_ref());
        }
        self.register_keyword_tag(&i.fn_token);
        self.register_token(&i.ident, TokenTag::Function);
        for input in &i.inputs {
            match input {
                FnArg::Receiver(arg) => {
                    self.register_token(&arg.self_token, TokenTag::SelfToken);
                    self.try_register_keyword_tag(arg.mutability.as_ref());
                    self.try_register_lifetime_tag(arg.lifetime());
                }
                FnArg::Typed(type_pat) => {
                    self.register_type_pattern(type_pat);
                    self.register_type(&type_pat.ty);
                }
            }
        }

        self.register_return_type(&i.output);
    }

    fn visit_abi(&mut self, i: &'ast syn::Abi) {
        self.register_keyword_tag(&i.extern_token);
        self.try_register_litstr_tag(i.name.as_ref());
    }

    fn visit_expr_async(&mut self, i: &'ast syn::ExprAsync) {
        self.register_keyword_tag(&i.async_token);
    }

    fn visit_visibility(&mut self, i: &'ast syn::Visibility) {
        self.register_keyword_tag(i);
    }

    fn visit_block(&mut self, i: &'ast syn::Block) {
        self.register_block(i);
    }

    // fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {}

    // fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {}
}

impl<'ast> RustHighlighter<'ast> {
    fn write_tokens(self, output: &mut Rope) {
        // for (k, v) in self.unidentified {
        //     eprintln!("{:?} : {:?}", k, v.ident.to_string())
        // }

        let mut tok_offset: usize = 0;
        for (index, token) in self.token_map {
            match token {
                TokenTag::NeedIdentification => {
                    let ident_string = self.unidentified.get(&index).unwrap().ident.to_string();

                    let identified = match ident_string.as_str() {
                        "self" | "Self" => TokenTag::SelfToken,
                        "Ok" | "Err" | "NotATable" | "NoMapping" => TokenTag::Enum,
                        "new_unchecked" | "parse_str" => TokenTag::Function,
                        _ => TokenTag::Ident,
                    };

                    let tag = identified.to_string();
                    output.insert(index + tok_offset, tag.as_str());
                    tok_offset += tag.len();
                    // eprintln!("{}: {}", index, t);
                }
                _ => {
                    let tag = token.to_string();
                    output.insert(index + tok_offset, tag.as_str());
                    tok_offset += tag.len();
                }
            }
        }
    }

    /// Extract a span position in the rope.
    ///
    /// returns the (start_idx, end_idx) of the span
    ///
    /// TODO: assuming same line, create tests to assert this assumption
    fn span_position(span: &impl Spanned) -> (usize, usize) {
        // lines are 1 indexed instead of zero.
        let span = span.span().byte_range();
        (span.start, span.end)
    }

    fn register_tag_on_index(&mut self, start: usize, end: usize, tag: TokenTag) {
        self.token_map.insert(start, tag);
        self.token_map.insert(end, TokenTag::EndOfToken);
    }

    pub(crate) fn register_token(&mut self, token: &impl Spanned, tag: TokenTag) {
        let (start, end) = Self::span_position(token);
        self.register_tag_on_index(start, end, tag);
    }

    /// Register a tag with start index of t1 and end index of t2.
    pub(crate) fn register_merged_token(
        &mut self,
        t1: &'ast impl Spanned,
        t2: &'ast impl Spanned,
        tag: TokenTag,
    ) {
        let p1 = Self::span_position(t1);
        let p2 = Self::span_position(t2);
        self.token_map.insert(p1.0, tag);
        self.token_map.insert(p2.1, TokenTag::EndOfToken);
    }

    fn identify_token(&self, index: usize) -> TokenTag {
        TokenTag::Ident
    }
}

impl<'ast> RustHighlighter<'ast> {
    #[add_try_method]
    fn register_statement(&mut self, token: &'ast Stmt) {
        match token {
            syn::Stmt::Local(token) => {
                self.register_local(token);
            }
            syn::Stmt::Expr(token, _) => {
                self.register_expr(token);
            }
            syn::Stmt::Macro(token) => {
                self.register_macro_statement(token);
            }
            syn::Stmt::Item(token) => {
                self.register_item(token);
            }
        }
    }

    fn register_lit_expr(&mut self, token: &'ast ExprLit) {
        match &token.lit {
            Lit::Int(_) | Lit::Float(_) => {
                self.register_litnum_tag(&token.lit);
            }
            Lit::Bool(_) => {
                self.register_litbool_tag(&token.lit);
            }
            Lit::Byte(_) | Lit::ByteStr(_) | Lit::CStr(_) | Lit::Char(_) | Lit::Str(_) => {
                self.register_litstr_tag(&token.lit)
            }
            _ => {}
        }
    }

    #[add_try_method]
    fn register_expr(&mut self, token: &'ast Expr) {
        // MAKE A MACRO TO CREATE THIS AUTOMATICALLY

        match token {
            Expr::Lit(token) => {
                self.register_lit_expr(token);
            }
            Expr::ForLoop(token) => {
                self.register_for_loop_expr(token);
            }
            Expr::Unsafe(token) => {
                self.register_unsafe_expr(token);
            }
            Expr::MethodCall(token) => {
                self.register_method_call_expr(token);
            }
            Expr::Path(token) => {
                self.register_path_expr(token);
            }
            Expr::Reference(token) => {
                self.register_reference_expr(token);
            }
            Expr::Unary(token) => {
                self.register_unary_expr(token);
            }
            Expr::Binary(token) => {
                self.register_binary_expr(token);
            }
            Expr::Try(token) => {
                self.register_try_expr(token);
            }
            Expr::If(token) => {
                self.register_if_expr(token);
            }
            Expr::Call(token) => {
                self.register_call_expr(token);
            }
            Expr::Block(token) => {
                self.register_block_expr(token);
            }
            Expr::Paren(token) => {
                self.register_parentheses_expr(token);
            }
            Expr::Cast(token) => {
                self.register_cast_expr(token);
            }
            Expr::Field(token) => {
                self.register_field_expr(token);
            }
            Expr::Match(token) => {
                self.register_match_expr(token);
            }
            Expr::Tuple(token) => {
                self.register_tuple_expr(token);
            }
            _ => {}
        }
    }

    fn register_type(&mut self, token: &'ast Type) {
        match token {
            Type::Reference(token) => {
                self.register_reference_type(token);
            }
            Type::Path(token) => {
                self.register_path_type(token);
            }
            Type::Tuple(token) => {
                self.register_tuple_type(token);
            }
            Type::ImplTrait(token) => {
                self.register_impl_trait_type(token);
            }
            _ => {}
        }
    }

    fn register_item(&mut self, token: &'ast Item) {}

    #[add_try_method]
    fn register_macro_statement(&mut self, token: &'ast StmtMacro) {
        // TODO NEED CHANGE TO RENDER PATH CORRECTLY AND TO PARSE TOKEN TREE BETTER WITH SPECIFIC KEY WORD FOR BUILTIN MACROS
        self.register_merged_token(&token.mac.path, &token.mac.bang_token, TokenTag::Macro);
        for token in token.mac.tokens.clone() {
            if let TokenTree::Literal(lit) = token {
                if let Ok(_) = syn::parse_str::<LitStr>(&lit.to_string()) {
                    self.register_litstr_tag(&lit);
                }
            }
        }
    }

    fn register_path_argument(&mut self, token: &'ast PathArguments) {
        match token {
            PathArguments::Parenthesized(token) => {
                self.register_parenthesized_arg(token);
            }
            PathArguments::AngleBracketed(token) => {
                self.register_angle_brackets_arg(token);
            }
            PathArguments::None => {}
        }
    }

    fn register_pattern(&mut self, token: &'ast Pat) {
        match token {
            Pat::Ident(token) => {
                self.register_ident_pat(token);
            }
            Pat::Reference(token) => {
                self.register_reference_pat(token);
            }
            Pat::Type(token) => {
                self.register_type_pat(token);
            }
            Pat::Path(token) => {
                self.register_path_expr(token);
            }
            Pat::Tuple(token) => {
                self.register_tuple_pat(token);
            }
            Pat::TupleStruct(token) => {
                self.register_tuple_struct_pat(token);
            }
            Pat::Or(token) => {
                self.register_or_pat(token);
            }
            Pat::Lit(token) => {
                self.register_lit_expr(token);
            }
            _ => {
                self.register_ident_tag(token);
            }
        }
    }
}

impl<'ast> RustHighlighter<'ast> {
    fn register_or_pat(&mut self, token: &'ast PatOr) {
        for case in &token.cases {
            self.register_pattern(case);
        }
    }

    fn register_trait_bound(&mut self, token: &'ast TraitBound) {}

    fn register_capture_param(&mut self, token: &'ast CapturedParam) {
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

    fn register_precise_capture(&mut self, token: &'ast PreciseCapture) {
        self.register_keyword_tag(&token.use_token);
        for param in &token.params {
            self.register_capture_param(param);
        }
    }

    fn register_bound(&mut self, token: &'ast TypeParamBound) {
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

    fn register_impl_trait_type(&mut self, token: &'ast TypeImplTrait) {
        self.register_keyword_tag(&token.impl_token);
        for bound in &token.bounds {
            self.register_bound(bound);
        }
    }

    fn register_tuple_type(&mut self, token: &'ast TypeTuple) {
        for arg in &token.elems {
            self.register_type(arg);
        }
    }

    fn register_tuple_expr(&mut self, token: &'ast ExprTuple) {
        for arg in &token.elems {
            self.register_expr(arg);
        }
    }

    fn register_tuple_pat(&mut self, token: &'ast PatTuple) {
        for arg in &token.elems {
            self.register_pattern(arg);
        }
    }

    fn register_tuple_struct_pat(&mut self, token: &'ast PatTupleStruct) {
        self.try_register_qself(token.qself.as_ref());
        self.register_path(&token.path, Some(TokenTag::Enum));
        for arg in &token.elems {
            self.register_pattern(arg);
        }
    }

    fn register_arm(&mut self, token: &'ast Arm) {
        self.register_pattern(&token.pat);
        if let Some(guard) = &token.guard {
            self.register_keyword_tag(&guard.0);
            self.register_expr(&guard.1);
        }
        self.register_expr(&token.body);
    }

    fn register_match_expr(&mut self, token: &'ast ExprMatch) {
        self.register_keyword_tag(&token.match_token);
        self.register_expr(&token.expr);
        for arm in &token.arms {
            self.register_arm(arm);
        }
    }

    fn register_member(&mut self, token: &'ast Member) {
        match token {
            Member::Named(token) => {
                // TODO Can mark this as variable
                self.register_ident_tag(token);
            }
            Member::Unnamed(token) => {
                self.register_litnum_tag(token);
            }
        }
    }

    fn register_field_expr(&mut self, token: &'ast ExprField) {
        self.register_expr(&token.base);
        self.register_member(&token.member);
    }

    fn register_cast_expr(&mut self, token: &'ast ExprCast) {
        self.register_expr(&token.expr);
        self.register_keyword_tag(&token.as_token);
        self.register_type(&token.ty);
    }

    fn register_parentheses_expr(&mut self, token: &'ast ExprParen) {
        self.register_expr(&token.expr);
    }

    fn register_type_pat(&mut self, token: &'ast PatType) {
        self.register_pattern(&token.pat);
        self.register_type(&token.ty);
    }

    fn register_path_segment(&mut self, token: &'ast PathSegment, tag: TokenTag) {
        self.register_token(&token.ident, tag);
        self.register_path_argument(&token.arguments);
    }

    fn register_generic_argument(&mut self, token: &'ast GenericArgument) {
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

    fn register_return_type(&mut self, token: &'ast ReturnType) {
        match token {
            ReturnType::Default => {}
            ReturnType::Type(_, token) => {
                self.register_type(token);
            }
        }
    }

    fn register_parenthesized_arg(&mut self, token: &'ast ParenthesizedGenericArguments) {
        for input in &token.inputs {
            self.register_type(input);
        }
        self.register_return_type(&token.output);
    }

    fn register_angle_brackets_arg(&mut self, token: &'ast AngleBracketedGenericArguments) {
        for arg in &token.args {
            self.register_generic_argument(arg);
        }
    }

    fn register_reference_type(&mut self, token: &'ast TypeReference) {
        self.try_register_lifetime_tag(token.lifetime.as_ref());
        self.try_register_keyword_tag(token.mutability.as_ref());
        self.register_type(&token.elem);
    }

    fn register_path_type(&mut self, token: &'ast TypePath) {
        self.try_register_qself(token.qself.as_ref());
        self.register_path(&token.path, Some(TokenTag::Type));
    }

    fn register_block_expr(&mut self, token: &'ast ExprBlock) {
        self.register_block(&token.block);
    }

    fn register_call_expr(&mut self, token: &'ast ExprCall) {
        // TODO UNDERSTAND HOW TO SIGNAL THIS PATH TO BE A FUNCTION
        self.register_expr(&token.func);
        for arg in &token.args {
            self.register_expr(arg);
        }
    }

    fn register_binary_expr(&mut self, token: &'ast ExprBinary) {
        self.register_expr(&token.left);
        self.register_expr(&token.right);
    }

    #[add_try_method]
    fn register_local(&mut self, token: &'ast Local) {
        self.register_keyword_tag(&token.let_token);
        self.register_pattern(&token.pat);
        self.try_register_local_init(token.init.as_ref());
    }

    #[add_try_method]
    fn register_block(&mut self, token: &'ast Block) {
        for statement in &token.stmts {
            self.register_statement(&statement);
        }
    }

    #[add_try_method]
    fn register_for_loop_expr(&mut self, token: &'ast ExprForLoop) {
        self.register_keyword_tag(&token.for_token);
        self.register_pattern(&token.pat);
        self.register_keyword_tag(&token.in_token);
        self.register_expr(&token.expr);
        self.register_block(&token.body);
    }

    fn register_if_expr(&mut self, token: &'ast ExprIf) {
        self.register_keyword_tag(&token.if_token);
        self.register_expr(&token.cond);
        self.register_block(&token.then_branch);
        if let Some(else_block) = &token.else_branch {
            self.register_keyword_tag(&else_block.0);
            self.register_expr(&else_block.1);
        }
    }

    fn register_try_expr(&mut self, token: &'ast ExprTry) {
        self.register_expr(&token.expr);
    }

    fn register_unary_expr(&mut self, token: &'ast ExprUnary) {
        self.register_expr(&token.expr);
    }

    fn register_reference_expr(&mut self, token: &'ast ExprReference) {
        self.try_register_keyword_tag(token.mutability.as_ref());
        self.register_expr(&token.expr);
    }

    fn register_path_expr(&mut self, token: &'ast ExprPath) {
        self.try_register_qself(token.qself.as_ref());
        self.register_path(&token.path, None);
    }

    #[add_try_method]
    fn register_qself(&mut self, token: &'ast QSelf) {
        self.register_type(&token.ty);
        self.try_register_keyword_tag(token.as_token.as_ref());
    }

    /// Register a path token
    ///
    /// # Parameters
    ///
    /// - `token:` - The path segment
    /// - `last:` - Optional tag to put for the last item of the path.
    ///
    /// TODO ADD DOCUMENTATION AND PLAN WHAT HAPPENS IF NONE IS GIVEN
    fn register_path(&mut self, token: &'ast Path, last: Option<TokenTag>) {
        let mut segment_iter = token.segments.iter().rev();
        let last_segment = segment_iter.next();
        for segment in &token.segments {
            self.register_path_segment(segment, TokenTag::Segment);
        }
        match last_segment {
            Some(segment) => match last {
                Some(tag) => self.register_path_segment(segment, tag),
                None => {
                    self.register_path_segment(segment, TokenTag::NeedIdentification);
                    self.unidentified
                        .insert(segment.span().byte_range().start, &segment);
                }
            },
            None => {}
        }
    }

    fn register_method_call_expr(&mut self, token: &'ast ExprMethodCall) {
        self.register_expr(&token.receiver);
        self.register_function_tag(&token.method);
        for arg in &token.args {
            self.register_expr(arg);
        }
    }

    fn register_unsafe_expr(&mut self, token: &'ast ExprUnsafe) {
        self.register_keyword_tag(&token.unsafe_token);
        self.register_block(&token.block);
    }

    #[add_try_method]
    fn register_reference_pat(&mut self, token: &'ast PatReference) {
        self.try_register_keyword_tag(token.mutability.as_ref());
        self.register_pattern(&token.pat);
    }

    #[add_try_method]
    fn register_local_init(&mut self, token: &'ast LocalInit) {
        self.register_expr(&token.expr);
    }

    #[add_try_method]
    fn register_ident_pat(&mut self, token: &'ast PatIdent) {
        self.try_register_keyword_tag(token.by_ref.as_ref());
        self.try_register_keyword_tag(token.mutability.as_ref());
        self.register_token(&token.ident, TokenTag::Ident);
    }

    #[add_try_method]
    fn register_type_pattern(&mut self, token: &'ast PatType) {
        self.register_pattern(&token.pat);
    }

    fn register_comments(&mut self, code: &str) {
        let comment_regex: Regex = Regex::new(r"\/\/\/?[^\n]*").unwrap();
        for comment in comment_regex.captures_iter(code) {
            let m = comment.get(0).unwrap();
            self.register_tag_on_index(m.start(), m.end(), TokenTag::Comment);
        }
    }
}

impl<'ast> RustHighlighter<'ast> {
    pub fn highlight(code: &str) -> String {
        let mut output = Rope::from_str(code);
        let mut highlighter = RustHighlighter {
            token_map: BTreeMap::new(),
            unidentified: HashMap::new(),
        };

        let syntax_tree: File =
            syn::parse_str(code).expect(&format!("Failed to parse Rust code {}", code));

        highlighter.visit_file(&syntax_tree);
        highlighter.register_comments(code);
        highlighter.write_tokens(&mut output);

        output.to_string()
    }
}
