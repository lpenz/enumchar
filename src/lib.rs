// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../doc.md")]

use quote::quote;
use std::collections::HashMap;
use syn::parse_macro_input;
use syn::Error;
use syn::Ident;
use syn::Result;

struct VariantData {
    pub ident: Ident,
    pub varchar: Option<char>,
}

fn variantdata_get(dataenum: &syn::DataEnum) -> Result<Vec<VariantData>> {
    dataenum
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let Some(expr) = variant.attrs.iter().find_map(|attr| {
                attr.path()
                    .is_ident("char")
                    .then(|| attr.parse_args().ok())
                    .flatten()
            }) else {
                return Ok::<VariantData, syn::Error>(VariantData {
                    ident: ident.clone(),
                    varchar: None,
                });
            };
            let syn::Expr::Lit(lit) = expr else {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("variant {} has invalid EnumChar char type", ident),
                ));
            };
            let syn::Lit::Char(c) = lit.lit else {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("variant {} has invalid EnumChar char type", ident),
                ));
            };
            Ok::<VariantData, syn::Error>(VariantData {
                ident: ident.clone(),
                varchar: Some(c.value()),
            })
        })
        .collect::<Result<Vec<VariantData>>>()
}

fn has_variant_without_char(dataenum: &syn::DataEnum) -> bool {
    !dataenum.variants.iter().all(|variant| {
        variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("char"))
    })
}

fn check_enumchars_unique(vdata: &[VariantData]) -> syn::Result<()> {
    let mut seen = HashMap::<char, &Ident>::new();
    for vd in vdata {
        let Some(ref varchar) = vd.varchar else {
            continue;
        };
        if let Some(other) = seen.get(varchar) {
            return Err(Error::new(
                vd.ident.span(),
                format!(
                    "EnumChar variants {} and {} using the same char {}",
                    vd.ident, other, varchar
                ),
            ));
        }
        seen.insert(*varchar, &vd.ident);
    }
    Ok(())
}

fn tryfrom_char_gen(topid: &Ident, vdata: &[VariantData]) -> proc_macro2::TokenStream {
    let tryfrom_char_matches = vdata.iter().map(|vd| {
        let ident = &vd.ident;
        let chareq = vd.varchar?;
        Some(quote! {
            #chareq => Ok(Self::#ident),
        })
    });
    let errmsg = format!("unable to convert {{}} into {}", topid);
    quote! {
        impl TryFrom<char> for #topid {
            type Error = String;
            fn try_from(c: char) -> Result<Self, Self::Error> {
                match c {
                    #(#tryfrom_char_matches)*
                    _ => Err(format!(#errmsg, c))
                }
            }
        }
    }
}

fn tryinto_char_gen(topid: &Ident, vdata: &[VariantData]) -> proc_macro2::TokenStream {
    // If there's a variant without char, we create a TryFrom impl
    let tryfrom_enum_matches = vdata
        .iter()
        .map(|vd| {
            let ident = &vd.ident;
            if let Some(chareq) = vd.varchar {
                Some(quote! {
                    #topid::#ident => Ok(#chareq),
                })
            } else {
                let errmsg = format!("variant {}::{} has no char representation", topid, ident);
                Some(quote! {
                    #topid::#ident => Err(#errmsg.to_string())
                })
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl TryFrom<#topid> for char {
            type Error = String;
            fn try_from(e: #topid) -> Result<Self, Self::Error> {
                match e {
                    #(#tryfrom_enum_matches)*
                }
            }
        }
        impl TryFrom<&#topid> for char {
            type Error = String;
            fn try_from(e: &#topid) -> Result<Self, Self::Error> {
                match e {
                    #(#tryfrom_enum_matches)*
                }
            }
        }
    }
}

fn into_char_gen(topid: &Ident, vdata: &[VariantData]) -> proc_macro2::TokenStream {
    // If all variants have chars, we can just impl From
    let from_enum_matches = vdata
        .iter()
        .map(|vd| {
            let ident = &vd.ident;
            let chareq = vd.varchar;
            quote! {
                #topid::#ident => #chareq,
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl From<#topid> for char {
            fn from(e: #topid) -> Self {
                match e {
                    #(#from_enum_matches)*
                }
            }
        }
        impl From<&#topid> for char {
            fn from(e: &#topid) -> Self {
                match e {
                    #(#from_enum_matches)*
                }
            }
        }
    }
}

fn display_gen(topid: &Ident) -> proc_macro2::TokenStream {
    quote! {
        impl std::fmt::Display for #topid {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let c = char::try_from(self).map_err(|_| std::fmt::Error)?;
                write!(f, "{}", c)
            }
        }
    }
}

fn fromstr_gen(topid: &Ident, vdata: &[VariantData]) -> proc_macro2::TokenStream {
    let fromstr_matches = vdata.iter().filter_map(|vd| {
        let ident = &vd.ident;
        let chareq = vd.varchar?.to_string();
        Some(quote! {
            #chareq => Ok(#topid::#ident),
        })
    });
    let errmsg = format!("unable to parse str into {}", topid);
    quote! {
        impl std::str::FromStr for #topid {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#fromstr_matches)*
                    _ => Err(format!(#errmsg))
                }
            }
        }
    }
}

fn my_derive(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    // eprintln!("{:#?}", input);
    let syn::Data::Enum(ref dataenum) = input.data else {
        return Err(Error::new(
            input.ident.span(),
            "EnumChar can only be used with enums",
        ));
    };
    // eprintln!("{:#?}", dataenum);
    let topid = &input.ident;
    let vdata = variantdata_get(dataenum)?;
    check_enumchars_unique(&vdata)?;
    let tryfrom_char_code = tryfrom_char_gen(topid, &vdata);
    let into_char_code = if has_variant_without_char(dataenum) {
        tryinto_char_gen(topid, &vdata)
    } else {
        into_char_gen(topid, &vdata)
    };
    let display_code = if !has_variant_without_char(dataenum) {
        display_gen(topid)
    } else {
        quote! {}
    };
    let fromstr_code = fromstr_gen(topid, &vdata);
    let expanded = quote! {
        #tryfrom_char_code
        #into_char_code
        #display_code
        #fromstr_code
    };
    // eprintln!("{}", expanded);
    Ok(expanded)
}

/// The `EnumChar` derive macro allows the usage of the `char`
/// attribute to define an equivalent character for each enum
/// variant. The macro then automatically creates `TryFrom<char>`,
/// `TryInto<char>`, `std::fmt::Display` and other default `impl`.
///
/// See also the top-level [crate documentation](crate) for more
/// information and examples.
#[proc_macro_derive(EnumChar, attributes(char))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    my_derive(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
