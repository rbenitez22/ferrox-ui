//! The CSS and fonts shipped with ferrox-theme, compiled into this crate so an
//! app's build script needs no path to them.
//!
//! Adding a built-in theme: put the file in `assets/themes/` and add a line to
//! [`THEMES`]. A test fails if the two disagree.

/// The default theme (Patina on `:root`), the palette, and the derived role slots.
pub(crate) const TOKENS_CSS: &str = include_str!("../assets/tokens.css");

/// `@font-face` rules for the shipped fonts (relative URLs into `fonts/`).
pub(crate) const FONTS_CSS: &str = include_str!("../assets/fonts.css");

/// The pre-mount loading screen (`.ferrox-loading`).
pub(crate) const LOADING_CSS: &str = include_str!("../assets/loading.css");

/// Built-in theme files: `(file name, css)`, sorted by name.
pub(crate) const THEMES: &[(&str, &str)] = &[
    ("classic.css", include_str!("../assets/themes/classic.css")),
    ("classic-dark.css", include_str!("../assets/themes/classic-dark.css")),
    ("foundry.css", include_str!("../assets/themes/foundry.css")),
    ("jamaica.css", include_str!("../assets/themes/jamaica.css")),
    ("jamaica-dark.css", include_str!("../assets/themes/jamaica-dark.css")),
    ("patina-dark.css", include_str!("../assets/themes/patina-dark.css")),
    ("pl-ex.css", include_str!("../assets/themes/pl-ex.css")),
    ("pl-ex-dark.css", include_str!("../assets/themes/pl-ex-dark.css")),
];

/// Font files and their licences: `(file name, bytes)`.
pub(crate) const FONT_FILES: &[(&str, &[u8])] = &[
    ("cormorant-garamond-var-italic-latin-ext.woff2", include_bytes!("../assets/fonts/cormorant-garamond-var-italic-latin-ext.woff2")),
    ("cormorant-garamond-var-italic-latin.woff2", include_bytes!("../assets/fonts/cormorant-garamond-var-italic-latin.woff2")),
    ("cormorant-garamond-var-normal-latin-ext.woff2", include_bytes!("../assets/fonts/cormorant-garamond-var-normal-latin-ext.woff2")),
    ("cormorant-garamond-var-normal-latin.woff2", include_bytes!("../assets/fonts/cormorant-garamond-var-normal-latin.woff2")),
    ("dm-mono-300-latin-ext.woff2", include_bytes!("../assets/fonts/dm-mono-300-latin-ext.woff2")),
    ("dm-mono-300-latin.woff2", include_bytes!("../assets/fonts/dm-mono-300-latin.woff2")),
    ("dm-mono-400-latin-ext.woff2", include_bytes!("../assets/fonts/dm-mono-400-latin-ext.woff2")),
    ("dm-mono-400-latin.woff2", include_bytes!("../assets/fonts/dm-mono-400-latin.woff2")),
    ("dm-mono-500-latin-ext.woff2", include_bytes!("../assets/fonts/dm-mono-500-latin-ext.woff2")),
    ("dm-mono-500-latin.woff2", include_bytes!("../assets/fonts/dm-mono-500-latin.woff2")),
    ("OFL-cormorant-garamond.txt", include_bytes!("../assets/fonts/OFL-cormorant-garamond.txt")),
    ("OFL-dm-mono.txt", include_bytes!("../assets/fonts/OFL-dm-mono.txt")),
];

/// The built-in theme files as `(file name, css)`, for tools that want to read
/// or check them.
pub fn builtin_sources() -> &'static [(&'static str, &'static str)] {
    THEMES
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::contract::{parse_theme, REQUIRED_TOKENS};
    use crate::scan::{declared_tokens, strip_comments};

    fn listed_dir(dir: &str, ext: &str) -> Vec<String> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir);
        let mut names: Vec<String> = std::fs::read_dir(path)
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .filter(|n| n.ends_with(ext))
            .collect();
        names.sort();
        names
    }

    #[test]
    fn theme_list_matches_the_assets_dir() {
        let mut listed: Vec<&str> = THEMES.iter().map(|(name, _)| *name).collect();
        listed.sort();
        assert_eq!(listed, listed_dir("assets/themes", ".css"));
    }

    #[test]
    fn font_list_matches_the_assets_dir() {
        let mut listed: Vec<&str> = FONT_FILES.iter().map(|(name, _)| *name).collect();
        listed.sort();
        let mut on_disk = listed_dir("assets/fonts", ".woff2");
        on_disk.extend(listed_dir("assets/fonts", ".txt"));
        on_disk.sort();
        assert_eq!(listed, on_disk);
    }

    #[test]
    fn every_builtin_theme_passes_the_contract() {
        for (file, css) in THEMES {
            parse_theme(file, css).unwrap_or_else(|e| panic!("{e}"));
        }
    }

    /// Keeps REQUIRED_TOKENS honest: the default theme in tokens.css must
    /// declare every token a theme is asked to restate.
    #[test]
    fn tokens_css_declares_every_required_token() {
        let css = strip_comments(TOKENS_CSS);
        let declared = declared_tokens(&css);
        let missing: Vec<_> = REQUIRED_TOKENS.iter().filter(|t| !declared.contains(t)).collect();
        assert!(missing.is_empty(), "tokens.css lacks {missing:?}");
    }

    #[test]
    fn every_font_file_is_referenced_by_fonts_css() {
        for (name, _) in FONT_FILES.iter().filter(|(n, _)| n.ends_with(".woff2")) {
            assert!(FONTS_CSS.contains(&format!("url('fonts/{name}')")), "{name} is not used");
        }
    }
}
