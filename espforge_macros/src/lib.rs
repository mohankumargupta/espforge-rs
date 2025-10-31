use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced, parse::{Parse, ParseStream}, parse_macro_input,
    Ident, Token,
};

struct ExampleConfig {
    name: Ident,
    entries: Vec<(String, Ident)>,
}

impl Parse for ExampleConfig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut entries = Vec::new();
        while !content.is_empty() {
            let key_ident: Ident = content.parse()?;
            content.parse::<Token![=>]>()?;
            let ty_ident: Ident = content.parse()?;

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }

            entries.push((key_ident.to_string(), ty_ident));
        }

        Ok(Self { name, entries })
    }
}

#[proc_macro]
pub fn generate_example_enum(input: TokenStream) -> TokenStream {
    let ExampleConfig { name, entries } = parse_macro_input!(input as ExampleConfig);

    // Generate PascalCase variant names
    let variant_names = entries.iter().map(|(key, _ty)| {
         to_pascal_case_ident(key)
     });    
    let variants = entries.iter().map(|(key, ty)| {
        let variant_name = to_pascal_case_ident(key);
        quote! { #variant_name(#ty) }
    });

    // Generate match arms
    let match_arms = entries.iter().map(|(key, ty)| {
        let variant_name = to_pascal_case_ident(key);
        let key_str = key;
        quote! {
            #key_str => {
                match #ty::deserialize(value.clone()) {
                    Ok(c) => Some(Self::#variant_name(c)),
                    Err(e) => {
                        eprintln!("Error parsing [example.{}]: {}", #key_str, e);
                        None
                    }
                }
            }
        }
    });

    // let expanded = quote! {
    //     #[derive(Debug, Clone)]
    //     pub enum #name {
    //         #( #variants, )*
    //     }

    //     impl #name {
    //         pub fn handle_example(name: &str, value: &toml::Value) -> Option<Self> {
    //             use serde::Deserialize;
    //             match name {
    //                 #( #match_arms, )*
    //                 _ => None,
    //             }
    //         }
    //     }


    // };

     let expanded = quote! {
         #[derive(Debug, Clone)]
         pub enum #name {
             #( #variants, )*
         }
 
         impl #name {
             pub fn handle_example(name: &str, value: &toml::Value) -> Option<Self> {
                 use serde::Deserialize;
                 match name {
                     #( #match_arms, )*
                     _ => None,
                 }
             }
         }
 
         impl crate::Example for #name {
             fn render(&self) -> Result<String, askama::Error> {
                 match self {
                     #(
                         Self::#variant_names(config) => config.render(),
                     )*
                 }
             }
         }
        };

    expanded.into()
}

fn to_pascal_case_ident(s: &str) -> Ident {
    let mut out = String::new();
    let mut uppercase = true;
    for c in s.chars() {
        if c == '_' {
            uppercase = true;
            continue;
        }
        if uppercase {
            out.push(c.to_ascii_uppercase());
            uppercase = false;
        } else {
            out.push(c);
        }
    }
    Ident::new(&out, proc_macro2::Span::call_site())
}

