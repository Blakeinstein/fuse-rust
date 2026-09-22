#![warn(missing_docs)]

//! Derive macro for the [`Fuseable`] trait of the
//! [`fuse-rust`](https://docs.rs/fuse-rust) crate.
//!
//! This crate is re-exported by `fuse-rust` when its `derive` feature is enabled, so
//! there is normally no need to depend on it directly.
//!
//! [`Fuseable`]: https://docs.rs/fuse-rust/latest/fuse_rust/trait.Fuseable.html

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Lit};

/// Derives `Fuseable` for a struct with named fields.
///
/// Mark each field that should take part in the search with `#[fuse]`. Fields without
/// the attribute are ignored, so a struct may freely hold ids, counts or any other
/// non-textual data.
///
/// # Examples
///
/// A bare `#[fuse]` gives the field a weight of `1.0`:
///
/// ```ignore
/// #[derive(Fuseable)]
/// struct Tag {
///     #[fuse]
///     name: String,
/// }
/// ```
///
/// Use `#[fuse(weight = ...)]` to weigh fields against each other. Weights must be
/// greater than `0.0` and no greater than `1.0`:
///
/// ```ignore
/// #[derive(Fuseable)]
/// struct Book {
///     #[fuse(weight = 0.3)]
///     title: String,
///     #[fuse(weight = 0.7)]
///     author: String,
///     isbn: u64, // not searched
/// }
/// ```
///
/// Every annotated field must be usable as a `&str`, which covers `String` and `&str`.
#[proc_macro_derive(Fuseable, attributes(fuse))]
pub fn derive_fuseable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = named_fields(&input)?;

    let mut selected: Vec<(Ident, f64)> = Vec::new();
    for field in fields {
        if let Some(weight) = field_weight(field)? {
            let name = field
                .ident
                .clone()
                .expect("named fields always have an identifier");
            selected.push((name, weight));
        }
    }

    if selected.is_empty() {
        return Err(syn::Error::new(
            ident.span(),
            "`Fuseable` needs at least one searchable field: \
             mark one with `#[fuse]`, or `#[fuse(weight = 0.5)]` to weigh it",
        ));
    }

    let properties = selected.iter().map(|(name, weight)| {
        let key = name.to_string();
        quote! {
            ::fuse_rust::FuseProperty {
                value: ::std::string::String::from(#key),
                weight: #weight,
            }
        }
    });

    let arms = selected.iter().map(|(name, _)| {
        let key = name.to_string();
        quote! { #key => ::core::option::Option::Some(&self.#name) }
    });

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::fuse_rust::Fuseable for #ident #ty_generics #where_clause {
            fn properties(&self) -> ::std::vec::Vec<::fuse_rust::FuseProperty> {
                ::std::vec![#(#properties),*]
            }

            fn lookup(&self, key: &str) -> ::core::option::Option<&str> {
                match key {
                    #(#arms,)*
                    _ => ::core::option::Option::None,
                }
            }
        }
    })
}

/// `Fuseable` maps field names to values, so the struct must have named fields.
fn named_fields(
    input: &DeriveInput,
) -> syn::Result<&syn::punctuated::Punctuated<syn::Field, syn::token::Comma>> {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => Ok(&named.named),
            Fields::Unnamed(fields) => Err(syn::Error::new(
                fields.span(),
                "`Fuseable` cannot be derived for a tuple struct, because its fields have no names to look up",
            )),
            Fields::Unit => Err(syn::Error::new(
                input.ident.span(),
                "`Fuseable` cannot be derived for a unit struct, because it has no fields to search",
            )),
        },
        Data::Enum(data) => Err(syn::Error::new(
            data.enum_token.span(),
            "`Fuseable` can only be derived for structs",
        )),
        Data::Union(data) => Err(syn::Error::new(
            data.union_token.span(),
            "`Fuseable` can only be derived for structs",
        )),
    }
}

/// Returns the weight for a field, or `None` when the field carries no `#[fuse]`
/// attribute and so should not be searched.
fn field_weight(field: &syn::Field) -> syn::Result<Option<f64>> {
    let mut weight: Option<f64> = None;
    let mut marked = false;

    for attr in &field.attrs {
        if !attr.path().is_ident("fuse") {
            continue;
        }
        marked = true;

        if matches!(attr.meta, syn::Meta::Path(_)) {
            continue; // bare `#[fuse]`, weight defaults to 1.0
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("weight") {
                let literal: Lit = meta.value()?.parse()?;
                weight = Some(parse_weight(&literal)?);
                Ok(())
            } else {
                Err(meta.error("unknown `fuse` option, expected `weight`"))
            }
        })?;
    }

    Ok(marked.then(|| weight.unwrap_or(1.0)))
}

/// Weights are combined as `1.0 - weight`, so anything outside `(0.0, 1.0]` would score
/// nonsensically. Reject it here rather than at search time.
fn parse_weight(literal: &Lit) -> syn::Result<f64> {
    let value = match literal {
        Lit::Float(float) => float.base10_parse::<f64>()?,
        Lit::Int(int) => int.base10_parse::<i64>()? as f64,
        other => {
            return Err(syn::Error::new(
                other.span(),
                "`weight` must be a number, for example `#[fuse(weight = 0.3)]`",
            ))
        }
    };

    if !value.is_finite() || value <= 0.0 || value > 1.0 {
        return Err(syn::Error::new(
            literal.span(),
            "`weight` must be greater than 0.0 and at most 1.0",
        ));
    }

    Ok(value)
}
