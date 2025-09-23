use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemEnum, ItemFn, Type, parse_macro_input, token::Continue};

#[proc_macro_attribute]
pub fn add_try_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    let fn_name = &sig.ident;
    let fn_name_str = fn_name.to_string();

    let base = fn_name_str
        .strip_prefix("register_")
        .expect("Function must be named `register_<name>`");

    let try_ident = format_ident!("try_register_{}", base);

    let arg_ty = if let Some(arg) = sig.inputs.iter().nth(1) {
        match arg {
            syn::FnArg::Typed(pat) => match &*pat.ty {
                Type::Reference(ref_ty) => &ref_ty.elem,
                _ => &pat.ty,
            },
            _ => panic!("Expected typed argument"),
        }
    } else {
        panic!("Function must have &mut self and one argument");
    };

    let expanded = quote! {
        #vis #sig #block

        #[allow(dead_code)]
        pub(crate) fn #try_ident(&mut self, token: Option<&'ast #arg_ty>) {
            if let Some(token) = token {
                self.#fn_name(token);
            }
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn register_variants(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    let enum_name = &input.ident;

    let methods = input.variants.iter().map(|v| {
        let variant_name = &v.ident;
        let variant_string = variant_name.to_string().to_lowercase();
        let method_name = format_ident!("register_{}_tag", variant_string);
        quote! {
            #[add_try_method]
            pub(crate) fn #method_name(&mut self, token: &(impl syn::spanned::Spanned)) {
                self.register_token(token, #enum_name::#variant_name);
            }
        }
    });

    let expanded = quote! {
        #input

        impl<'ast> RustHighlighter<'ast> {
            #(#methods)*
        }
    };

    expanded.into()
}
