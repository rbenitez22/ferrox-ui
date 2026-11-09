//! Build-time support for ferrox-icons: the rules an icon file must follow,
//! and the generated Rust that turns checked files into `IconData` consts.
//!
//! ferrox-icons' own `build.rs` uses it for the icons ferrox ships, so every
//! icon goes through one set of rules. App icons (defined in the app's code
//! with `ferrox_icons::icon!`) will use the same checks, at compile time.

#[doc(hidden)]
pub mod discover;
#[doc(hidden)]
pub mod emit;
mod markup;
mod naming;
#[doc(hidden)]
pub mod sources;
mod validate;

use std::fs;
use std::path::Path;

/// One icon file that passed every rule: its name (the file stem, e.g.
/// `"chevron-down"`) and its drawing (the SVG's children as compact markup,
/// safe to set with `inner_html`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckedIcon {
    pub name: String,
    pub body: String,
}

/// Reads and checks one icon file. The error names the rule it breaks; the
/// caller adds the file name.
pub fn check_file(file: &Path) -> Result<CheckedIcon, String> {
    let name = stem(file)?;
    naming::check_kebab(&name)?;
    let text = fs::read_to_string(file).map_err(|e| format!("cannot read: {e}"))?;
    let doc = roxmltree::Document::parse(&text).map_err(|e| e.to_string())?;
    validate::check(&doc)?;
    Ok(CheckedIcon { body: markup::body(doc.root_element()), name })
}

/// The file stem as UTF-8.
pub fn stem(file: &Path) -> Result<String, String> {
    file.file_stem()
        .and_then(|s| s.to_str())
        .map(str::to_string)
        .ok_or_else(|| "file name is not UTF-8".to_string())
}
