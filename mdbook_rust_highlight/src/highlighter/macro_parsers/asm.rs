use super::ident_list::IdentList;
// use proc_macro2::Span;
use syn::{
    Expr, ExprPath, Ident, LitStr, Stmt, Token,
    parse::Parse,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Brace, Paren},
};

pub struct AsmArgs {
    pub args: Punctuated<AsmInput, Token![,]>,
}

impl Parse for AsmArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            args: input.parse_terminated(AsmInput::parse, Token![,])?,
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
        if input.peek(LitStr) {
            return Ok(Self::Instruction(input.parse()?));
        }
        if input.peek(Ident) {
            match input.parse::<AsmOptions>() {
                Ok(options) => return Ok(Self::Options(options)),
                Err(_) => {}
            }
        }
        Ok(Self::RegOperand(input.parse()?))
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
        let option_token = input.parse::<Ident>()?;
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

        let directive: Ident = if input.peek(Token![in]) {
            let kw = input.parse::<Token![in]>()?;
            Ident::new("in", kw.span())
        } else if input.peek(Token![const]) {
            let kw = input.parse::<Token![const]>()?;
            Ident::new("const", kw.span())
        } else {
            eprintln!("here");
            input.parse()?
        };
        eprintln!("directive: {}", directive.to_string());

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

impl Parse for Label {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            label_token: input.parse()?,
            brackets: syn::braced!(content in input),
            stmt: content.parse()?,
        })
    }
}

pub enum ExprOperand {
    Operand(OperandDirective),
    Symbol(ExprSym),
    Constant(ExprConstAsm),
    Label(Label),
}

impl Parse for ExprOperand {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if (input.peek(Ident) || input.peek(Token![in])) && input.peek2(Paren) {
            return Ok(Self::Operand(input.parse::<OperandDirective>()?));
        }
        if input.peek(Ident) && input.peek2(Brace) {
            return Ok(Self::Label(input.parse::<Label>()?));
        }
        if input.peek(Token![const]) {
            let kw = input.parse::<Token![const]>()?;
            let const_ident = Ident::new("const", kw.span());
            let expr = input.parse()?;
            return Ok(Self::Constant(ExprConstAsm {
                const_token: const_ident,
                expr,
            }));
        }
        let ident = input.parse::<Ident>()?;
        if ident == "sym" {
            return Ok(Self::Symbol(ExprSym {
                sym_token: ident,
                expr: input.parse()?,
            }));
        }
        return Err(input.error("Expected Operand, Symbol, Const or Label"));
    }
}

pub struct RegOperand {
    pub param_eq: Option<(Ident, Token![=])>,
    pub expr: ExprOperand,
}

impl Parse for RegOperand {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // check first token is an Ident and second is '='
        let param_eq = if input.peek(Ident) && input.peek2(Token![=]) {
            let param = input.parse()?;
            let eq = input.parse()?;
            Some((param, eq))
        } else {
            None
        };
        Ok(RegOperand {
            param_eq: param_eq,
            expr: input.parse()?,
        })
    }
}
