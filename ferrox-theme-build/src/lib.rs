//! Build-time support for ferrox-theme. Used from an app's `build.rs`:
//!
//! ```rust,ignore
//! // build.rs
//! fn main() {
//!     // The app's own themes, for ferrox_theme::register (optional).
//!     ferrox_theme_build::embed_themes("style/themes");
//!     // The CSS file and fonts index.html links.
//!     ferrox_theme_build::write_css("style/ferrox", Some("style/themes"));
//! }
//! ```
//!
//! ```rust,ignore
//! // main.rs, before mounting
//! ferrox_theme::register(include!(concat!(env!("OUT_DIR"), "/ferrox_themes.rs")));
//! ferrox_theme::init();
//! ```
//!
//! Trunk resolves `index.html`'s links before it runs cargo, so on a clean
//! checkout the generated files would not exist yet. Run the build script
//! first with a pre-build hook in `Trunk.toml`:
//!
//! ```toml
//! [[hooks]]
//! stage = "pre_build"
//! command = "cargo"
//! command_arguments = ["check", "--quiet", "--target", "wasm32-unknown-unknown"]
//! ```
//!
//! ```html
//! <!-- index.html -->
//! <link data-trunk rel="css" href="style/ferrox/ferrox.css" />
//! <link data-trunk rel="copy-dir" href="style/ferrox/fonts" />
//! ```
//!
//! # The theme file contract
//!
//! - The file name is the theme id: `jamaica-dark.css` → `data-theme="jamaica-dark"`.
//!   Lowercase ASCII letters and digits, separated by single hyphens.
//! - `<family>-dark.css` is the dark half of `<family>`; any other name is a light half.
//! - Every rule is scoped to `:root[data-theme='<id>']` (single or double quotes).
//! - Every token in [`REQUIRED_TOKENS`] is declared.
//! - The label defaults to the title-cased family (`pl-ex` → `Pl-Ex`) and can be
//!   overridden with a comment containing `@label <text>`.
//!
//! A file that breaks the contract fails the build, with every problem listed.
//!
//! Std only, so an app's build script never compiles ferrox-theme itself.

mod assets;
mod builtin;
mod contract;
mod embed;
mod scan;

pub use assets::{write_css, CSS_FILE, FONTS_DIR};
pub use builtin::builtin_sources;
pub use contract::{default_label, family_of, is_valid_id, parse_theme, ThemeError, ThemeMeta, REQUIRED_TOKENS};
pub use embed::{discover, embed_builtin_themes, embed_themes, embed_themes_for, render, ThemeSource, OUTPUT_FILE};

/// This crate's version. Generated code asks ferrox-theme to confirm it is
/// the same, so a build crate and runtime from different tags fail clearly.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
