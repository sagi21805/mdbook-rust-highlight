use syn::{
    Expr, ExprPath, Ident, LitStr, Stmt, Token,
    parse::Parse,
    punctuated::Punctuated,
    token::{Brace, Paren},
};

use super::ident_list::IdentList;

pub struct AsmArgs {
    inputs: Punctuated<AsmInput, Token![,]>,
}

impl Parse for AsmArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

pub enum AsmInput {
    Instruction(LitStr),
    Options(AsmOptions),
    RegOperand(RegOperand),
}

impl Parse for AsmInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

/// Structs represents the options on asm! macro `options(nostack, noreturn)`
pub struct AsmOptions {
    pub option_token: Ident,
    pub paren: Paren,
    pub options: IdentList,
}

impl Parse for AsmOptions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let option_token: Ident = input.parse()?;
        if option_token.to_string() != "options" {
            return Err(syn::Error::new_spanned(
                option_token,
                "Expected ident to be options",
            ));
        }
        let content;
        let paren = syn::parenthesized!(content in input);
        let options: IdentList = content.parse()?;
        Ok(Self {
            option_token,
            paren,
            options,
        })
    }
}

/// The reg in `in(reg)` or the 'rax' in `out("rax")`
pub enum RegSpec {
    RegClass(Ident),
    ExplicitReg(LitStr),
}

impl Parse for RegSpec {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

/// Inline asm operand directive : `in(reg) 3`
pub struct OperandDirective {
    pub directive: Ident,
    pub paren: Paren,
    pub reg: RegSpec,
    pub expr: Expr,
}

impl Parse for OperandDirective {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

pub struct OperandDualDirective {
    pub directive: Ident,
    pub paren: Paren,
    pub reg: RegSpec,
    pub expr: Expr,
    pub expr2: Option<(Token![=>], Expr)>,
}

impl Parse for OperandDualDirective {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

pub struct ExprSym {
    pub sym_token: Ident,
    pub expr: ExprPath,
}

impl Parse for ExprSym {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

pub struct ExprConstAsm {
    pub const_token: Ident,
    pub expr: Expr,
}

impl Parse for ExprConstAsm {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

pub struct Label {
    pub label_token: Ident,
    pub brackets: Brace,
    pub stmt: Stmt,
}

impl Parse for Label {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

pub enum ExprOperand {
    Directive(OperandDirective),
    DualDirective(OperandDualDirective),
    Symbol(ExprSym),
    Constant(ExprConstAsm),
    Label(Label),
}

impl Parse for ExprOperand {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

pub struct RegOperand {
    pub param_name_eq: Option<(Ident, Token![=])>,
    pub expr: ExprOperand,
}

impl Parse for RegOperand {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}
