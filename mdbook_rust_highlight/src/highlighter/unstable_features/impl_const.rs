use syn::{
    Attribute, Generics, ImplItem, Path, Token, Type, WhereClause,
    parse::{Parse, ParseStream},
    token,
};

pub struct ItemImplConst {
    pub attrs: Vec<Attribute>,
    pub defaultness: Option<Token![default]>,
    pub impl_token: Token![impl],
    pub constness: Option<Token![const]>,
    pub generics: Generics,
    /// Trait this impl implements.
    pub trait_: Option<(Option<Token![!]>, Path, Token![for])>,
    /// The Self type of the impl.
    pub self_ty: Box<Type>,
    pub brace_token: token::Brace,
    pub items: Vec<ImplItem>,
}

impl Parse for ItemImplConst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let defaultness: Option<Token![default]> = if input.peek(Token![default]) {
            Some(input.parse()?)
        } else {
            None
        };
        let impl_token: Token![impl] = input.parse()?;
        let constness: Option<Token![const]> = if input.peek(Token![const]) {
            Some(input.parse()?)
        } else {
            None
        };
        let mut generics: Generics = input.parse()?;
        let trait_;
        let self_ty;
        let polarity: Option<Token![!]> = if input.peek(Token![!]) {
            Some(input.parse()?)
        } else {
            None
        };
        let path_or_ty: Type = input.parse()?;

        if input.peek(Token![for]) {
            // CASE: impl Trait for Type
            let for_token: Token![for] = input.parse()?;
            let path = match path_or_ty {
                Type::Path(type_path) if type_path.qself.is_none() => type_path.path,
                _ => {
                    return Err(syn::Error::new_spanned(path_or_ty, "expected a trait path"));
                }
            };

            trait_ = Some((polarity, path, for_token));
            self_ty = Box::new(input.parse()?);
        } else {
            // CASE: impl Type (Inherent impl)
            if let Some(bang) = polarity {
                return Err(syn::Error::new_spanned(
                    bang,
                    "inherent impls cannot be negative",
                ));
            }
            trait_ = None;
            self_ty = Box::new(path_or_ty);
        }
        generics.where_clause = input.parse::<Option<WhereClause>>()?;
        let content;
        let brace_token = syn::braced!(content in input);
        let mut items = Vec::new();

        while !content.is_empty() {
            items.push(content.parse()?);
        }

        Ok(ItemImplConst {
            attrs,
            defaultness,
            constness,
            impl_token,
            generics,
            trait_,
            self_ty,
            brace_token,
            items,
        })
    }
}
