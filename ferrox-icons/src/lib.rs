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

mod icon;

pub use icon::{icon_classes, Icon};

/// One icon: its name (the file stem, e.g. `"chevron-down"`) and its drawing
/// (the SVG's children, on a 24×24 grid). Only this crate makes these, from
/// files its build script has checked, so the drawing is safe to set as
/// markup.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct IconData {
    pub name: &'static str,
    pub body: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/icons.rs"));

#[cfg(test)]
#[path = "../build/mod.rs"]
mod build_logic;

#[cfg(test)]
mod tests {
    use super::*;

    /// The stems of `icons/*.svg`, sorted.
    fn icon_files() -> Vec<String> {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("icons");
        build_logic::discover::svg_files(&dir).unwrap().iter()
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

    #[test]
    fn the_first_set_is_there() {
        let set = [BAN, CAMERA, CHECK, CHEVRON_DOWN, CHEVRON_RIGHT, CLOSE, DELETE, HELP, LOCK, MENU, PLUS, REFRESH, SEARCH, UNLOCK];
        assert_eq!(set.len(), ALL.len());
        assert!(set.iter().all(|icon| ALL.contains(icon)));
    }
}
