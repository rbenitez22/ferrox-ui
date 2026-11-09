//! Derives for `ferrox_traits::{HasId, HasName, HasParentId}`: each returns
//! a clone of one field, by default `id`, `name`, `parent_id`, or the field
//! named by `#[has_id(field = "…")]` (and `has_name`, `has_parent_id`).
//!
//! The impl names the trait unqualified, so the trait must be in scope where
//! the derive is used. Importing the derive imports the trait with it
//! (`use ferrox_traits::HasId;`, or through a re-export such as
//! `webapp_lib::reactive::HasId`), so this holds whichever path brings them.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, LitStr};

/// One trait a derive implements: its name, method, return type, the field
/// read by default, and the attribute that overrides it.
pub struct Accessor {
    pub trait_name: &'static str,
    pub method: &'static str,
    pub returns: fn() -> TokenStream,
    pub default_field: &'static str,
    pub attribute: &'static str,
}

pub const HAS_ID: Accessor = Accessor {
    trait_name: "HasId",
    method: "get_id",
    returns: || quote! { String },
    default_field: "id",
    attribute: "has_id",
};

pub const HAS_NAME: Accessor = Accessor {
    trait_name: "HasName",
    method: "get_name",
    returns: || quote! { String },
    default_field: "name",
    attribute: "has_name",
};

pub const HAS_PARENT_ID: Accessor = Accessor {
    trait_name: "HasParentId",
    method: "get_parent_id",
    returns: || quote! { Option<String> },
    default_field: "parent_id",
    attribute: "has_parent_id",
};

pub fn expand(accessor: &Accessor, input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    let field = field_of(accessor, &input)?;
    let (ty, trait_name, method) = (&input.ident, ident(accessor.trait_name), ident(accessor.method));
    let returns = (accessor.returns)();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics #trait_name for #ty #ty_generics #where_clause {
            fn #method(&self) -> #returns {
                self.#field.clone()
            }
        }
    })
}

/// The field to read: the attribute's `field = "…"`, or the default.
fn field_of(accessor: &Accessor, input: &DeriveInput) -> syn::Result<Ident> {
    let mut field = None;
    for attr in input.attrs.iter().filter(|a| a.path().is_ident(accessor.attribute)) {
        attr.parse_nested_meta(|meta| {
            if !meta.path.is_ident("field") {
                return Err(meta.error("expected `field = \"…\"`"));
            }
            let name: LitStr = meta.value()?.parse()?;
            field = Some(Ident::new(&name.value(), name.span()));
            Ok(())
        })?;
    }
    Ok(field.unwrap_or_else(|| ident(accessor.default_field)))
}

fn ident(name: &str) -> Ident {
    Ident::new(name, proc_macro2::Span::call_site())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expand_str(accessor: &Accessor, item: &str) -> syn::Result<String> {
        expand(accessor, item.parse().unwrap()).map(|t| t.to_string())
    }

    #[test]
    fn has_id_reads_id_by_default() {
        let out = expand_str(&HAS_ID, "struct User { id: String }").unwrap();
        assert_eq!(out, "impl HasId for User { fn get_id (& self) -> String { self . id . clone () } }");
    }

    #[test]
    fn the_attribute_picks_another_field() {
        let out = expand_str(&HAS_NAME, r#"#[has_name(field = "display_name")] struct User { display_name: String }"#).unwrap();
        assert!(out.contains("impl HasName for User"), "{out}");
        assert!(out.contains("self . display_name . clone ()"), "{out}");
    }

    #[test]
    fn has_parent_id_returns_an_option() {
        let out = expand_str(&HAS_PARENT_ID, "struct Store { parent_id: Option<String> }").unwrap();
        assert!(out.contains("fn get_parent_id (& self) -> Option < String >"), "{out}");
        assert!(out.contains("self . parent_id . clone ()"), "{out}");
    }

    #[test]
    fn another_derives_attribute_is_ignored() {
        let out = expand_str(&HAS_ID, r#"#[has_name(field = "label")] struct Tag { id: String, label: String }"#).unwrap();
        assert!(out.contains("self . id . clone ()"), "{out}");
    }

    #[test]
    fn generics_are_carried() {
        let out = expand_str(&HAS_ID, "struct Row<T: Clone> { id: String, value: T }").unwrap();
        assert!(out.contains("impl < T : Clone > HasId for Row < T >"), "{out}");
    }

    #[test]
    fn an_unknown_key_is_an_error() {
        let err = expand_str(&HAS_ID, r#"#[has_id(column = "x")] struct User { x: String }"#).unwrap_err();
        assert!(err.to_string().contains("expected `field"), "{err}");
    }
}
