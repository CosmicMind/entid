/* Copyright © 2025, CosmicMind, Inc. */

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Lit};

/// Attribute for setting the prefix for an entity
#[proc_macro_attribute]
pub fn prefix(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // This is just a marker attribute, so we return the item unchanged
    item
}

/// Attribute for setting the delimiter for an entity
#[proc_macro_attribute]
pub fn delimiter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // This is just a marker attribute, so we return the item unchanged
    item
}

/// Derive macro for implementing the `Prefix` trait
///
/// # Attributes
///
/// - `#[entid(prefix = "...")]` - Sets the prefix for the entity (required)
/// - `#[entid(delimiter = "...")]` - Sets the delimiter for the entity (optional, defaults to "_")
///
/// # Example
///
/// ```rust
/// use entid::Prefix;
///
/// #[derive(Prefix)]
/// #[entid(prefix = "user", delimiter = "_")]
/// pub struct User;
///
/// #[derive(Prefix)]
/// #[entid(prefix = "post", delimiter = "-")]
/// pub struct Post;
///
/// // The delimiter is optional and defaults to "_"
/// #[derive(Prefix)]
/// #[entid(prefix = "comment")]
/// pub struct Comment;
/// ```
#[proc_macro_derive(Prefix, attributes(entid))]
pub fn derive_prefix(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Default values
    let mut prefix = format!("{}_", name).to_lowercase();
    let mut delimiter = String::from("_");

    // Parse attributes
    for attr in &input.attrs {
        if attr.path().is_ident("entid") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("prefix") {
                    let value = meta.value()?;
                    let expr: Expr = value.parse()?;
                    if let Expr::Lit(expr_lit) = expr {
                        if let Lit::Str(lit_str) = expr_lit.lit {
                            prefix = lit_str.value();
                        }
                    }
                } else if meta.path.is_ident("delimiter") {
                    let value = meta.value()?;
                    let expr: Expr = value.parse()?;
                    if let Expr::Lit(expr_lit) = expr {
                        if let Lit::Str(lit_str) = expr_lit.lit {
                            delimiter = lit_str.value();
                        }
                    }
                }
                Ok(())
            })
            .ok();
        }
    }

    let gen = quote! {
        impl entid::Prefix for #name {
            fn prefix() -> &'static str {
                #prefix
            }

            fn delimiter() -> &'static str {
                #delimiter
            }
        }
    };

    gen.into()
}
