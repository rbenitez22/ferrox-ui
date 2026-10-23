//! What the user asked for (family + mode), and its `family:mode` string form —
//! the same format auth-api will sync.

use super::registry::{find_family, ThemeFamily, DEFAULT_FAMILY};

/// Light, dark, or follow the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

impl ThemeMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ThemeMode::System => "system",
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "system" => Some(ThemeMode::System),
            "light" => Some(ThemeMode::Light),
            "dark" => Some(ThemeMode::Dark),
            _ => None,
        }
    }

    /// Whether this mode shows the dark half, given the device preference.
    pub fn wants_dark(self, prefers_dark: bool) -> bool {
        match self {
            ThemeMode::System => prefers_dark,
            ThemeMode::Light => false,
            ThemeMode::Dark => true,
        }
    }
}

/// A family and a mode, as the user chose them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeChoice {
    pub family: String,
    pub mode: ThemeMode,
}

impl Default for ThemeChoice {
    fn default() -> Self {
        ThemeChoice { family: DEFAULT_FAMILY.to_string(), mode: ThemeMode::System }
    }
}

impl ThemeChoice {
    pub fn new(family: impl Into<String>, mode: ThemeMode) -> Self {
        ThemeChoice { family: family.into(), mode }
    }

    /// `family:mode`, e.g. `jamaica:system`.
    pub fn format(&self) -> String {
        format!("{}:{}", self.family, self.mode.as_str())
    }

    /// Reads `family:mode` (extra `:` fields are ignored). Tolerant: a value
    /// from an older build, another app, or a corrupted store never fails.
    /// Unknown family → default family; unknown mode → `System`. They fall
    /// back independently, so a family this app lacks still keeps the mode.
    pub fn parse(raw: &str, families: &[ThemeFamily]) -> Self {
        let mut fields = raw.split(':');
        let family = fields.next().unwrap_or("");
        let mode = fields.next().and_then(ThemeMode::parse).unwrap_or_default();
        ThemeChoice::new(family, mode).normalized(families)
    }

    /// Replaces a family not in `families` with the default family.
    pub fn normalized(self, families: &[ThemeFamily]) -> Self {
        if find_family(families, &self.family).is_some() {
            self
        } else {
            ThemeChoice { family: DEFAULT_FAMILY.to_string(), ..self }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn families() -> Vec<ThemeFamily> {
        vec![
            ThemeFamily { id: "patina", label: "Patina", light: Some(""), dark: Some("patina-dark") },
            ThemeFamily { id: "jamaica", label: "Jamaica", light: Some("jamaica"), dark: Some("jamaica-dark") },
        ]
    }

    #[test]
    fn format_and_parse_round_trip() {
        let choice = ThemeChoice::new("jamaica", ThemeMode::Dark);
        assert_eq!(choice.format(), "jamaica:dark");
        assert_eq!(ThemeChoice::parse(&choice.format(), &families()), choice);
    }

    #[test]
    fn parse_ignores_extra_fields() {
        let parsed = ThemeChoice::parse("jamaica:light:jamaica:jamaica-dark", &families());
        assert_eq!(parsed, ThemeChoice::new("jamaica", ThemeMode::Light));
    }

    #[test]
    fn unknown_family_keeps_mode() {
        assert_eq!(ThemeChoice::parse("ocean:dark", &families()), ThemeChoice::new("patina", ThemeMode::Dark));
    }

    #[test]
    fn unknown_mode_keeps_family() {
        assert_eq!(ThemeChoice::parse("jamaica:sepia", &families()), ThemeChoice::new("jamaica", ThemeMode::System));
    }

    #[test]
    fn garbage_is_the_default() {
        for raw in ["", ":", "nonsense", "::::"] {
            assert_eq!(ThemeChoice::parse(raw, &families()), ThemeChoice::default(), "{raw:?}");
        }
    }

    #[test]
    fn modes() {
        assert!(ThemeMode::System.wants_dark(true));
        assert!(!ThemeMode::System.wants_dark(false));
        assert!(ThemeMode::Dark.wants_dark(false));
        assert!(!ThemeMode::Light.wants_dark(true));
    }
}
