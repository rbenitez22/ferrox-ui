//! Build-time support for ferrox-icons: the rules an icon file must follow,
//! and the generated Rust that turns checked files into `IconData` consts.
//!
//! ferrox-icons' own `build.rs` uses it for the icons ferrox ships, so every
//! icon goes through one set of rules. App icons (defined in the app's code
//! with `ferrox_icons::icon!`) use the same checks, at compile time.

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

/// Reads and checks one icon file; its name is the file stem. The error names
/// the rule it breaks; the caller adds the file name.
pub fn check_file(file: &Path) -> Result<CheckedIcon, String> {
    let name = stem(file)?;
    let text = fs::read_to_string(file).map_err(|e| format!("cannot read: {e}"))?;
    check_markup(&name, &text)
}

/// Checks one icon given as markup (a whole `<svg>` document) under `name`
/// (kebab-case, e.g. `"chevron-down"`). The error names the rule it breaks.
pub fn check_markup(name: &str, svg: &str) -> Result<CheckedIcon, String> {
    naming::check_kebab(name)?;
    let doc = roxmltree::Document::parse(svg).map_err(|e| e.to_string())?;
    validate::check(&doc)?;
    Ok(CheckedIcon { name: name.to_string(), body: markup::body(doc.root_element()) })
}

/// The file stem as UTF-8.
pub fn stem(file: &Path) -> Result<String, String> {
    file.file_stem()
        .and_then(|s| s.to_str())
        .map(str::to_string)
        .ok_or_else(|| "file name is not UTF-8".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const STAR_LIKE: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M12 2 9 9H2l6 4-2 7 6-4 6 4-2-7 6-4h-7z"/>
</svg>"#;

    #[test]
    fn markup_is_checked_and_compacted() {
        let icon = check_markup("spark", STAR_LIKE).unwrap();
        assert_eq!(icon.name, "spark");
        assert_eq!(icon.body, r#"<path d="M12 2 9 9H2l6 4-2 7 6-4 6 4-2-7 6-4h-7z"/>"#);
    }

    #[test]
    fn markup_breaking_a_rule_is_rejected() {
        let red = STAR_LIKE.replace("<path ", "<path fill=\"red\" ");
        assert!(check_markup("spark", &red).unwrap_err().contains("currentColor"));
    }

    #[test]
    fn a_name_that_is_not_kebab_case_is_rejected() {
        assert!(check_markup("Spark", STAR_LIKE).is_err());
    }

    #[test]
    fn markup_that_is_not_xml_is_rejected() {
        assert!(check_markup("spark", "<svg").is_err());
    }
}
