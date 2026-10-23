//! The theme file contract: naming, scoping, required tokens, labels.

use std::fmt;

use crate::scan::{comment_value, declared_tokens, rule_selectors, strip_comments};

/// Tokens every theme must restate: the SURFACES block and each role's four
/// EXPLICIT slots from `tokens.css`. Derived slots (`--<role>-text|tint|…`)
/// are not listed — tokens.css derives them from `--<role>-base`.
/// `--fill-sheen` and `--scrim` are optional and inherit the default.
pub const REQUIRED_TOKENS: &[&str] = &[
    "--background", "--surface-0", "--surface-1", "--surface-2", "--surface-3",
    "--chart-in", "--chart-out",
    "--chart-cat-1", "--chart-cat-2", "--chart-cat-3", "--chart-cat-4", "--chart-cat-5",
    "--chart-other",
    "--text", "--text-muted", "--text-subtle", "--text-mix",
    "--line",
    "--focus-ring-color", "--focus-ring-width", "--focus-ring-offset",
    "--shadow-sm", "--shadow-md", "--shadow-lg",
    "--primary-base", "--primary-fill", "--primary-fill-hover", "--primary-on-fill",
    "--secondary-base", "--secondary-fill", "--secondary-fill-hover", "--secondary-on-fill",
    "--neutral-base", "--neutral-fill", "--neutral-fill-hover", "--neutral-on-fill",
    "--info-base", "--info-fill", "--info-fill-hover", "--info-on-fill",
    "--success-base", "--success-fill", "--success-fill-hover", "--success-on-fill",
    "--warning-base", "--warning-fill", "--warning-fill-hover", "--warning-on-fill",
    "--danger-base", "--danger-fill", "--danger-fill-hover", "--danger-on-fill",
];

const DARK_SUFFIX: &str = "-dark";
const LABEL_MARKER: &str = "@label";

/// What the build learns about one theme file.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ThemeMeta {
    pub id: String,
    pub family: String,
    pub dark: bool,
    pub label: String,
}

/// Why a theme file was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ThemeError {
    BadFileName { file: String },
    NoRules { id: String },
    UnscopedRule { id: String, selector: String },
    MissingTokens { id: String, missing: Vec<String> },
    Io { path: String, message: String },
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeError::BadFileName { file } => write!(
                f,
                "{file}: theme file names must be lowercase letters/digits separated by single hyphens, ending in .css"
            ),
            ThemeError::NoRules { id } => {
                write!(f, "{id}.css: no `:root[data-theme='{id}']` rule found")
            }
            ThemeError::UnscopedRule { id, selector } => write!(
                f,
                "{id}.css: rule `{selector}` must be scoped to `:root[data-theme='{id}']`"
            ),
            ThemeError::MissingTokens { id, missing } => {
                write!(f, "{id}.css: missing required tokens: {}", missing.join(", "))
            }
            ThemeError::Io { path, message } => write!(f, "{path}: {message}"),
        }
    }
}

impl std::error::Error for ThemeError {}

// ---- Naming ----

/// `true` when `id` is lowercase ASCII letters/digits separated by single hyphens.
pub fn is_valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .split('-')
            .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit()))
}

/// Splits an id into `(family, dark)`: `jamaica-dark` → `("jamaica", true)`.
pub fn family_of(id: &str) -> (&str, bool) {
    match id.strip_suffix(DARK_SUFFIX) {
        Some(family) if !family.is_empty() => (family, true),
        _ => (id, false),
    }
}

/// Title-cases each hyphen-separated part: `pl-ex` → `Pl-Ex`.
pub fn default_label(family: &str) -> String {
    family
        .split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join("-")
}

// ---- Validation ----

/// `true` when `selector` is `:root[data-theme='<id>']` (either quote style),
/// optionally followed by more selector text.
fn is_scoped(selector: &str, id: &str) -> bool {
    [format!(":root[data-theme='{id}']"), format!(":root[data-theme=\"{id}\"]")]
        .iter()
        .any(|root| selector.starts_with(root.as_str()))
}

/// Every comma-separated selector of every rule must be scoped to the theme.
fn check_scoping(id: &str, css: &str) -> Result<(), ThemeError> {
    let selectors = rule_selectors(css);
    if selectors.is_empty() {
        return Err(ThemeError::NoRules { id: id.to_string() });
    }
    for rule in &selectors {
        if let Some(bad) = rule.split(',').map(str::trim).find(|sel| !is_scoped(sel, id)) {
            return Err(ThemeError::UnscopedRule { id: id.to_string(), selector: bad.to_string() });
        }
    }
    Ok(())
}

/// Every [`REQUIRED_TOKENS`] entry must be declared.
fn check_tokens(id: &str, css: &str) -> Result<(), ThemeError> {
    let declared = declared_tokens(css);
    let missing: Vec<String> = REQUIRED_TOKENS
        .iter()
        .filter(|token| !declared.contains(token))
        .map(|token| token.to_string())
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(ThemeError::MissingTokens { id: id.to_string(), missing })
    }
}

/// Validates one theme's CSS (`file` is its file name) and derives its metadata.
pub fn parse_theme(file: &str, css: &str) -> Result<ThemeMeta, ThemeError> {
    let id = file
        .strip_suffix(".css")
        .filter(|id| is_valid_id(id))
        .ok_or_else(|| ThemeError::BadFileName { file: file.to_string() })?;

    let rules = strip_comments(css);
    check_scoping(id, &rules)?;
    check_tokens(id, &rules)?;

    let (family, dark) = family_of(id);
    Ok(ThemeMeta {
        id: id.to_string(),
        family: family.to_string(),
        dark,
        label: comment_value(css, LABEL_MARKER).unwrap_or_else(|| default_label(family)),
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn full_theme(id: &str) -> String {
        let body: String = REQUIRED_TOKENS.iter().map(|t| format!("  {t}: #000;\n")).collect();
        format!(":root[data-theme='{id}'] {{\n{body}}}\n")
    }

    #[test]
    fn id_validation() {
        assert!(is_valid_id("pl-ex-dark"));
        assert!(is_valid_id("theme2"));
        for bad in ["", "Pl-Ex", "pl--ex", "-pl", "pl-", "pl_ex", "pl ex"] {
            assert!(!is_valid_id(bad), "{bad:?} should be rejected");
        }
    }

    #[test]
    fn family_and_mode_from_name() {
        assert_eq!(family_of("jamaica-dark"), ("jamaica", true));
        assert_eq!(family_of("jamaica"), ("jamaica", false));
        assert_eq!(family_of("pl-ex-dark"), ("pl-ex", true));
        assert_eq!(family_of("dark"), ("dark", false));
    }

    #[test]
    fn labels() {
        assert_eq!(default_label("pl-ex"), "Pl-Ex");
        assert_eq!(default_label("foundry"), "Foundry");
        let css = format!("/* THEME\n   @label Plum Express\n*/\n{}", full_theme("pl-ex"));
        assert_eq!(parse_theme("pl-ex.css", &css).unwrap().label, "Plum Express");
        assert_eq!(parse_theme("pl-ex-dark.css", &full_theme("pl-ex-dark")).unwrap().label, "Pl-Ex");
    }

    #[test]
    fn valid_theme_metadata() {
        let meta = parse_theme("sea-dark.css", &full_theme("sea-dark")).unwrap();
        assert_eq!((meta.id.as_str(), meta.family.as_str(), meta.dark), ("sea-dark", "sea", true));
    }

    #[test]
    fn double_quoted_selector_is_accepted() {
        let css = full_theme("sea").replace("'sea'", "\"sea\"");
        assert!(parse_theme("sea.css", &css).is_ok());
    }

    #[test]
    fn bad_file_name() {
        assert!(matches!(parse_theme("Sea.css", &full_theme("Sea")), Err(ThemeError::BadFileName { .. })));
    }

    #[test]
    fn selector_must_match_file_name() {
        let err = parse_theme("sea.css", &full_theme("ocean")).unwrap_err();
        assert!(matches!(err, ThemeError::UnscopedRule { .. }), "{err}");
    }

    #[test]
    fn unscoped_extra_rule_is_rejected() {
        let css = full_theme("sea") + "body { color: red; }\n";
        assert_eq!(
            parse_theme("sea.css", &css).unwrap_err(),
            ThemeError::UnscopedRule { id: "sea".into(), selector: "body".into() }
        );
    }

    #[test]
    fn scoped_rule_inside_media_query_is_accepted() {
        let css = full_theme("sea") + "@media (prefers-reduced-motion: reduce) { :root[data-theme='sea'] { --react-grow: none; } }\n";
        assert!(parse_theme("sea.css", &css).is_ok());
    }

    #[test]
    fn selectors_in_comments_are_ignored() {
        let css = format!("/* body {{ }} */\n{}", full_theme("sea"));
        assert!(parse_theme("sea.css", &css).is_ok());
    }

    #[test]
    fn empty_file_is_rejected() {
        assert!(matches!(parse_theme("sea.css", "/* nothing */"), Err(ThemeError::NoRules { .. })));
    }

    #[test]
    fn missing_tokens_are_all_reported() {
        let css = full_theme("sea").replace("  --surface-3: #000;\n", "").replace("  --danger-on-fill: #000;\n", "");
        assert_eq!(
            parse_theme("sea.css", &css).unwrap_err(),
            ThemeError::MissingTokens { id: "sea".into(), missing: vec!["--surface-3".into(), "--danger-on-fill".into()] }
        );
    }

    #[test]
    fn token_named_only_in_a_comment_does_not_count() {
        let css = full_theme("sea").replace("  --line: #000;\n", "  /* --line: #000; */\n");
        assert!(matches!(parse_theme("sea.css", &css), Err(ThemeError::MissingTokens { .. })));
    }
}
