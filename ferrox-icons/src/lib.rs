//! SVG icons for Leptos CSR apps. Each icon is a value ([`IconData`], a const
//! such as [`CLOSE`]); one component, [`Icon`], draws any of them. Because
//! icons are values, a component can take one as a prop and an app can pick
//! one at runtime.
//!
//! ```rust,ignore
//! use ferrox_icons::{Icon, CLOSE};
//!
//! view! {
//!     <IconButton title="Close" on_click=…><Icon icon=CLOSE/></IconButton>
//! }
//! ```
//!
//! Icons follow the text colour (`currentColor`) and the font size (`1em`).
//! They are decorative (`aria-hidden`): the control around one carries the
//! label. The drawings are Lucide's (ISC licence, `icons/LICENSE-lucide`).

// `icon!` expands to `::ferrox_icons::…`; this makes that path work inside
// this crate too (its tests).
extern crate self as ferrox_icons;

mod icon;

pub use icon::{icon_classes, Icon};
/// An app icon from the app's own SVG, checked at compile time by the same
/// rules as the icons here: `icon!("icons/pin.svg")` (a file, from the crate
/// root; named by its stem) or `icon!(name = "spark", "<svg …>")`.
pub use ferrox_macros::icon;

/// One icon: its name (the file stem, e.g. `"chevron-down"`) and its drawing
/// (the SVG's children, on a 24×24 grid). Only this crate's build script and
/// [`icon!`] make these, from checked markup, so the drawing is safe to set
/// as markup.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct IconData {
    pub name: &'static str,
    pub body: &'static str,
}

impl IconData {
    /// Only for [`icon!`]'s expansion, which has checked `body`. Not an API:
    /// calling it with unchecked markup defeats the safety above.
    #[doc(hidden)]
    pub const fn __checked(name: &'static str, body: &'static str) -> Self {
        IconData { name, body }
    }
}

include!(concat!(env!("OUT_DIR"), "/icons.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    /// The stems of `icons/*.svg`, sorted.
    fn icon_files() -> Vec<String> {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("icons");
        ferrox_icons_build::discover::svg_files(&dir).unwrap().iter()
            .map(|p| p.file_stem().unwrap().to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn all_matches_the_icon_files_in_order() {
        let names: Vec<&str> = ALL.iter().map(|i| i.name).collect();
        assert_eq!(names, icon_files());
    }

    #[test]
    fn all_has_no_duplicate_drawings() {
        for (i, a) in ALL.iter().enumerate() {
            for b in &ALL[i + 1..] {
                assert_ne!(a.body, b.body, "{} and {} draw the same", a.name, b.name);
            }
        }
    }

    const FILE_ICON: IconData = icon!("icons/close.svg");
    const INLINE_ICON: IconData = icon!(name = "dash", r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/></svg>"#);

    #[test]
    fn icon_macro_from_a_file_matches_the_built_in() {
        assert_eq!(FILE_ICON, CLOSE);
    }

    #[test]
    fn icon_macro_inline() {
        assert_eq!(INLINE_ICON.name, "dash");
        assert_eq!(INLINE_ICON.body, r#"<path d="M5 12h14"/>"#);
    }

    #[test]
    fn the_first_set_is_there() {
        let set = [BAN, CAMERA, CHECK, CHEVRON_DOWN, CHEVRON_RIGHT, CLOSE, DELETE, HELP, LOCK, MENU, PLUS, REFRESH, SEARCH, UNLOCK];
        assert!(set.iter().all(|icon| ALL.contains(icon)));
    }
}
