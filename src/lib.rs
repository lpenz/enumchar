// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use quote::quote;
use std::collections::HashMap;
use syn::parse_macro_input;
use syn::Error;
use syn::Ident;

struct VariantData {
    pub ident: Ident,
    pub varchar: char,
}

fn variantdata_get(dataenum: &syn::DataEnum) -> Result<Vec<VariantData>, syn::Error> {
    dataenum
        .variants
        .iter()
        .filter_map(|variant| {
            variant
                .attrs
                .iter()
                .find_map(|attr| {
                    attr.path()
                        .is_ident("char")
                        .then(|| attr.parse_args().ok())
                        .flatten()
                })
                .map(|expr: syn::Expr| (&variant.ident, expr))
        })
        .map(|(ident, expr)| {
            let syn::Expr::Lit(lit) = expr else {
                return Err(syn::Error::new(
                    ident.span(),
                    "variant has invalid EnumChar char type",
                ));
            };
            let syn::Lit::Char(c) = lit.lit else {
                return Err(syn::Error::new(
                    ident.span(),
                    "variant has invalid EnumChar char type",
                ));
            };
            Ok::<VariantData, syn::Error>(VariantData {
                ident: ident.clone(),
                varchar: c.value(),
            })
        })
        .collect::<Result<Vec<VariantData>, syn::Error>>()
}

fn check_enumchars_unique(vdata: &[VariantData]) -> syn::Result<()> {
    let mut seen = HashMap::<char, &Ident>::new();
    for vd in vdata {
        if let Some(other) = seen.get(&vd.varchar) {
            return Err(Error::new(
                vd.ident.span(),
                format!(
                    "EnumChar variants {} and {} using the same char {}",
                    vd.ident, other, vd.varchar
                ),
            ));
        }
        seen.insert(vd.varchar, &vd.ident);
    }
    Ok(())
}

fn my_derive(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let syn::Data::Enum(ref dataenum) = input.data else {
        return Err(Error::new(
            input.ident.span(),
            "EnumChar can only be used with enums",
        ));
    };
    // eprintln!("{:#?}", dataenum);
    let name = &input.ident;
    let vdata = variantdata_get(dataenum)?;
    check_enumchars_unique(&vdata)?;
    let tryfrom_matches = vdata.iter().map(|vd| {
        let ident = &vd.ident;
        let chareq = vd.varchar;
        Some(quote! {
            #chareq => Ok(Self::#ident),
        })
    });
    let errmsg = format!("unable to convert {{}} into {}", name);
    let expanded = quote! {
        impl TryFrom<char> for #name {
            type Error = String;
            fn try_from(c: char) -> Result<Self, Self::Error> {
                match c {
                    #(#tryfrom_matches)*
                    _ => Err(format!(#errmsg, c))
                }
            }
        }
    };
    // eprintln!("{}", expanded);
    Ok(expanded)
}

#[proc_macro_derive(EnumChar, attributes(char))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    my_derive(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
