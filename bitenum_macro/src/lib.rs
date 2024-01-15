use proc_macro::TokenStream;

use quote::quote;
use syn::{Error, Expr};
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn bitenum(attrs: TokenStream, body: TokenStream) -> TokenStream {
    let scalar_type = match parse_scalar(attrs) {
        Ok(ident) => ident,
        Err(err) => { return err.to_compile_error().into(); }
    };

    let mut enum_type = match syn::parse::<syn::ItemEnum>(body) {
        Ok(item) => item,
        Err(err) => { return err.to_compile_error().into(); }
    };

    let enum_name = enum_type.ident.clone();

    let must_has_discriminant = enum_type.variants.iter().nth(0).map(|v| v.discriminant.is_some()).unwrap_or(false);

    for (idx, variant) in enum_type.variants.iter_mut().enumerate() {
        if must_has_discriminant != variant.discriminant.is_some() {
            return Error::new(
                variant.span(),
                "all variants must rather have or don't have values",
            ).to_compile_error().into();
        }

        if variant.discriminant.is_none() {
            variant.discriminant = Some((
                syn::token::Eq::default(),
                syn::parse_str::<Expr>(format!("1 << {}", idx).as_str()).unwrap()
            ));
        }
    }

    TokenStream::from(quote! {
        #[repr(#scalar_type)]
        #enum_type

        impl Into<#scalar_type> for #enum_name {
            fn into(self) -> #scalar_type { self as #scalar_type }
        }

        impl bitenum::BitEnum for #enum_name {
            type Scalar = #scalar_type;
        }
    })
}


fn parse_scalar(tokens: TokenStream) -> syn::Result<syn::Ident> {
    match syn::parse::<syn::Ident>(tokens) {
        Ok(ident) => match ident.to_string().as_str() {
            "u8" | "u16" | "u32" | "u64" | "u128" => Ok(ident),
            _ => Err(Error::new(ident.span(), "must be unsigned integer type"))
        },
        Err(err) => Err(err)
    }
}
