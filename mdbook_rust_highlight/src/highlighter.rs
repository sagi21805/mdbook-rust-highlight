use std::collections::{BTreeMap, HashMap};

use crate::tokens::TokenTag;
use proc_macro2::{Span, TokenStream, TokenTree};
use regex::Regex;
use ropey::Rope;
use syn::{
    AngleBracketedGenericArguments, Block, Expr, ExprBinary, ExprBlock, ExprCall, ExprForLoop,
    ExprIf, ExprLit, ExprMethodCall, ExprPath, ExprReference, ExprTry, ExprUnary, ExprUnsafe, File,
    FnArg, GenericArgument, Item, Lit, LitStr, Local, LocalInit, ParenthesizedGenericArguments,
    Pat, PatIdent, PatReference, PatType, Path, PathArguments, PathSegment, QSelf, ReturnType,
    Stmt, StmtMacro, Type, TypePath, TypeReference, spanned::Spanned, visit::Visit,
};

use mdbook_rust_highlight_derive::add_try_method;

pub struct RustHighlighter<'ast> {
    token_map: BTreeMap<usize, TokenTag<'ast>>,
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
                    self.try_register_lifetime_tag(arg.lifetime());
                }
                FnArg::Typed(type_pat) => {
                    self.register_type_pattern(type_pat);
                    self.register_token(&type_pat.ty, TokenTag::Type);
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
}

impl<'ast> RustHighlighter<'ast> {
    fn write_tokens(self, output: &mut Rope) {
        for (k, v) in self.unidentified {
            eprintln!("{:?} : {:?}", k, v.ident.to_string())
        }

        let mut tok_offset: usize = 0;
        for (index, token) in self.token_map {
            match token {
                // TODO FIX COMMENTS TAGGING IN THE MIDDLE OF THE TOKEN STREAM
                TokenTag::TokenStream(stream) => {
                    for token in stream.clone() {
                        if let TokenTree::Literal(lit) = token {
                            if let Ok(_) = syn::parse_str::<LitStr>(&lit.to_string()) {
                                let tag = TokenTag::LitStr.to_string();
                                let (start, end) = Self::span_position(&lit);
                                output.insert(start + tok_offset, tag.as_str());
                                tok_offset += tag.len();
                                let end_tag = TokenTag::EndOfToken.to_string();
                                output.insert(end + tok_offset, end_tag.as_str());
                                tok_offset += end_tag.len();
                            }
                        }
                    }
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

    fn register_tag_on_index(&mut self, start: usize, end: usize, tag: TokenTag<'ast>) {
        self.token_map.insert(start, tag);
        self.token_map.insert(end, TokenTag::EndOfToken);
    }

    pub(crate) fn register_token(&mut self, token: &'ast impl Spanned, tag: TokenTag<'ast>) {
        let (start, end) = Self::span_position(token);
        self.register_tag_on_index(start, end, tag);
    }

    /// Register a tag with start index of t1 and end index of t2.
    pub(crate) fn register_merged_token(
        &mut self,
        t1: &'ast impl Spanned,
        t2: &'ast impl Spanned,
        tag: TokenTag<'ast>,
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

    fn register_literal_tag(&mut self, token: &'ast ExprLit) {
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
                self.register_literal_tag(token);
            }
            Expr::ForLoop(token) => {
                self.register_for_loop(token);
            }
            Expr::Unsafe(token) => {
                self.register_unsafe_expr(token);
            }
            Expr::MethodCall(token) => {
                self.register_method_call(token);
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
            _ => {}
        }
    }

    fn register_type(&mut self, token: &'ast Type) {
        match token {
            Type::Reference(token) => {
                self.register_type_reference(token);
            }
            Type::Path(token) => {
                self.register_type_path(token);
            }
            _ => {}
        }
    }

    fn register_item(&mut self, token: &'ast Item) {}

    #[add_try_method]
    fn register_macro_statement(&mut self, token: &'ast StmtMacro) {
        // TODO NEED CHANGE TO RENDER PATH CORRECTLY AND TO PARSE TOKEN TREE BETTER WITH SPECIFIC KEY WORD FOR BUILTIN MACROS
        self.register_merged_token(&token.mac.path, &token.mac.bang_token, TokenTag::Macro);
        self.register_tokenstream_tag(&token.mac.tokens);
    }

    fn register_path_argument(&mut self, token: &'ast PathArguments) {
        match token {
            PathArguments::Parenthesized(token) => {
                self.register_parenthesized(token);
            }
            PathArguments::AngleBracketed(token) => {
                self.register_angle_brackets(token);
            }
            PathArguments::None => {}
        }
    }

    fn register_pattern(&mut self, token: &'ast Pat) {
        match token {
            Pat::Ident(i) => {
                self.register_pat_ident(i);
            }
            Pat::Reference(r) => {
                self.register_reference_pat(r);
            }
            _ => {}
        }
    }
}

impl<'ast> RustHighlighter<'ast> {
    fn register_path_segment(&mut self, token: &'ast PathSegment) {
        self.register_segment_tag(&token.ident);
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

    fn register_parenthesized(&mut self, token: &'ast ParenthesizedGenericArguments) {
        for input in &token.inputs {
            self.register_type(input);
        }
        self.register_return_type(&token.output);
    }

    fn register_angle_brackets(&mut self, token: &'ast AngleBracketedGenericArguments) {
        for arg in &token.args {
            self.register_generic_argument(arg);
        }
    }

    fn register_type_reference(&mut self, token: &'ast TypeReference) {}

    fn register_type_path(&mut self, token: &'ast TypePath) {
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
    fn register_for_loop(&mut self, token: &'ast ExprForLoop) {
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
    fn register_path(&mut self, token: &'ast Path, last: Option<TokenTag<'ast>>) {
        let mut segment_iter = token.segments.iter().rev();
        let last_segment = segment_iter.next();
        for segment in &token.segments {
            self.register_path_segment(segment);
        }
        match last_segment {
            Some(segment) => match last {
                Some(tag) => self.register_token(segment, tag),
                None => {
                    self.register_token(segment, last.unwrap_or(TokenTag::NeedIdentification));
                    self.unidentified
                        .insert(segment.span().byte_range().start, &segment);
                }
            },
            None => {}
        }
    }

    fn register_method_call(&mut self, token: &'ast ExprMethodCall) {
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
    fn register_pat_ident(&mut self, token: &'ast PatIdent) {
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
            self.register_tag_on_index(m.start(), m.end() - 1, TokenTag::Comment);
        }
    }

    fn register_tokenstream_tag(&mut self, token: &'ast TokenStream) {
        let (start, _) = Self::span_position(token);
        self.token_map.insert(start, TokenTag::TokenStream(token));
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
