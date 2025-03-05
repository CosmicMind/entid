/* Copyright © 2025, CosmicMind, Inc. */

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for implementing the `Prefix` trait
///
/// # Attributes
///
/// - `#[prefix = "..."]` - Sets the prefix for the entity (required)
/// - `#[delimiter = "..."]` - Sets the delimiter for the entity (optional, defaults to "_")
///
/// # Example
///
/// ```
/// use entid::Prefix;
///
/// #[derive(Prefix)]
/// #[prefix = "user"]
/// struct User;
///
/// #[derive(Prefix)]
/// #[prefix = "post"]
/// #[delimiter = "-"]
/// struct Post;
/// ```
#[proc_macro_derive(Prefix, attributes(prefix, delimiter))]
pub fn derive_prefix(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name
    let name = &input.ident;

    // Default values
    let mut prefix = None;
    let mut delimiter = None;

    // Parse attributes
    for attr in &input.attrs {
        if attr.path().is_ident("prefix") {
            match attr.meta {
                syn::Meta::NameValue(ref meta) => {
                    if let syn::Expr::Lit(ref expr_lit) = meta.value {
                        if let syn::Lit::Str(ref lit_str) = expr_lit.lit {
                            prefix = Some(lit_str.value());
                        }
                    }
                }
                _ => {}
            }
        } else if attr.path().is_ident("delimiter") {
            match attr.meta {
                syn::Meta::NameValue(ref meta) => {
                    if let syn::Expr::Lit(ref expr_lit) = meta.value {
                        if let syn::Lit::Str(ref lit_str) = expr_lit.lit {
                            delimiter = Some(lit_str.value());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Ensure prefix is provided
    let prefix = match prefix {
        Some(prefix) => prefix,
        None => {
            return syn::Error::new_spanned(&input.ident, "Missing #[prefix = \"...\"] attribute")
                .to_compile_error()
                .into();
        }
    };

    // Generate the implementation
    let impl_prefix = if let Some(delimiter) = delimiter {
        quote! {
            impl ::entid::Prefix for #name {
                fn prefix() -> &'static str {
                    #prefix
                }

                fn delimiter() -> &'static str {
                    #delimiter
                }
            }
        }
    } else {
        quote! {
            impl ::entid::Prefix for #name {
                fn prefix() -> &'static str {
                    #prefix
                }
            }
        }
    };

    // Return the generated implementation
    impl_prefix.into()
}
