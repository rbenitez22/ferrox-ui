//! Procedural macros for the ferrox-ui crates. Each family sits behind its
//! own feature and is re-exported by the crate it serves, so apps use
//! `ferrox_icons::icon!`, not this crate.
//!
//! | Feature | Macro | Re-exported as |
//! |---|---|---|
//! | `icons` | [`icon!`](macro@icon) | `ferrox_icons::icon!` |

#[cfg(feature = "icons")]
mod icon;

/// An app icon: an `IconData` const drawn from the app's own SVG, checked at
/// compile time by the same rules as ferrox's icons (24×24 grid, 2px round
/// `currentColor` strokes, no fill, drawing elements only). A file that
/// breaks a rule is a compile error naming the rule.
///
/// ```ignore
/// use ferrox_icons::{icon, IconData};
///
/// // A file, relative to the crate root (Cargo.toml's folder). Its name is
/// // the file stem: "pin".
/// pub const PIN: IconData = icon!("icons/pin.svg");
///
/// // Inline markup, with its name.
/// pub const SPARK: IconData = icon!(name = "spark", r#"<svg viewBox="0 0 24 24" …>…</svg>"#);
/// ```
#[cfg(feature = "icons")]
#[proc_macro]
pub fn icon(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    icon::expand(input.into()).unwrap_or_else(syn::Error::into_compile_error).into()
}
