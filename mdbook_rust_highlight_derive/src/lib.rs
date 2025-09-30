use proc_macro::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, FnArg, ItemFn, ItemImpl, Type, TypeImplTrait, parse_macro_input, parse_quote,
};

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
            FnArg::Typed(pat) => match &*pat.ty {
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

#[proc_macro_derive(RegisterVariants)]
pub fn register_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    // Make sure it's an enum
    let data_enum = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => {
            return syn::Error::new_spanned(
                &input.ident,
                "RegisterVariants can only be applied to enums",
            )
            .to_compile_error()
            .into();
        }
    };

    // Generate methods for each variant
    let methods = data_enum.variants.iter().map(|v| {
        let variant_name = &v.ident;
        let variant_string = variant_name.to_string().to_lowercase();
        let token_method = format_ident!("register_{}_tag", variant_string);
        let (register_function, impls): (_, syn::Type) = match variant_string.as_str() {
            "function" | "type" | "enum" | "ident" => (
                format_ident!("register_ident"),
                parse_quote!(syn::spanned::Spanned + ToString),
            ),
            _ => (
                format_ident!("register_tag"),
                parse_quote!(syn::spanned::Spanned),
            ),
        };

        quote! {
            #[add_try_method]
            pub(crate) fn #token_method(&mut self, token: &(impl #impls)) {
                let (start, end) = Self::span_position(token);
                self.#register_function(token, #enum_name::#variant_name);
            }
        }
    });

    let expanded = quote! {
        impl<'a, 'ast> RustHighlighter<'a, 'ast> {
            #(#methods)*
        }
    };

    expanded.into()
}
