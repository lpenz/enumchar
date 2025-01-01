// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Data;
use syn::DeriveInput;
use syn::Expr;

#[proc_macro_derive(EnumChar, attributes(char))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let variants = if let Data::Enum(ref data) = input.data {
        data.variants.clone()
    } else {
        Default::default()
    };
    let tryfrom_matches = variants.iter().filter_map(|variant| {
        let chareq = variant.attrs.iter().find_map(|attr| {
            if !attr.path().is_ident("char") {
                return None;
            }
            let chareq: Expr = attr.parse_args().ok()?;
            Some(chareq)
        })?;
        let ident = variant.ident.clone();
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
    TokenStream::from(expanded)
}
