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
        Ok(Self {
            inputs: input.parse_terminated(AsmInput::parse, Token![,])?,
        })
    }
}

pub enum AsmInput {
    Instruction(LitStr),
    Options(AsmOptions),
    RegOperand(RegOperand),
}

impl Parse for AsmInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let initial_ident: Ident = match input.parse() {
            Ok(i) => i,
            Err(_) => {
                let instr: LitStr = input.parse()?;
                return Ok(AsmInput::Instruction(instr));
            }
        };
        if input.peek(Token![=]) {
            // Must be reg operand
            todo!();
        };
        if input.peek(Paren) {
            // Can be Options or some of the reg operands
            todo!()
        };
        // must be label
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
        Ok(Self {
            option_token,
            paren: syn::parenthesized!(content in input),
            options: content.parse()?,
        })
    }
}

/// The reg in `in(reg)` or the 'rax' in `out("rax")`
pub enum RegSpec {
    ExplicitReg(LitStr),
    RegClass(Ident),
}

impl Parse for RegSpec {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            return Ok(Self::ExplicitReg(input.parse()?));
        } else if input.peek(Ident) {
            return Ok(Self::RegClass(input.parse()?));
        } else {
            return Err(input.error("Expected LitStr or Ident for RegSpec"));
        }
    }
}

/// Inline asm operand directive : `in(reg) 3`
pub struct OperandDirective {
    pub directive: Ident,
    pub paren: Paren,
    pub reg: RegSpec,
    pub expr: Expr,
    pub dual: Option<(Token![=>], Expr)>,
}

impl Parse for OperandDirective {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let reg_buf;
        let directive = input.parse()?;
        let paren = syn::parenthesized!(reg_buf in input);
        let reg = reg_buf.parse()?;
        let expr = input.parse()?;
        let dual = input
            .parse::<Token![=>]>()
            .ok()
            .map(|fat_arrow| input.parse().map(|expr2| (fat_arrow, expr2)))
            .transpose()?;
        Ok(Self {
            directive,
            paren,
            reg,
            expr,
            dual,
        })
    }
}

pub struct ExprSym {
    pub sym_token: Ident,
    pub expr: ExprPath,
}

pub struct ExprConstAsm {
    pub const_token: Ident,
    pub expr: Expr,
}

pub struct Label {
    pub label_token: Ident,
    pub brackets: Brace,
    pub stmt: Stmt,
}

pub enum ExprOperand {
    Operand(OperandDirective),
    Symbol(ExprSym),
    Constant(ExprConstAsm),
    Label(Label),
}

impl Parse for ExprOperand {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(Paren) {
            return Ok(ExprOperand::Operand(input.parse::<OperandDirective>()?));
        } else if input.peek(Brace) {
            let content;
            let brace = syn::braced!(content in input);
        }
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
