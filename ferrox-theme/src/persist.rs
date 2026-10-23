//! The localStorage cache shared with [`super::PREPAINT_SCRIPT`].
//!
//! Stored as `family:mode:light-id:dark-id`. The first two fields are the
//! choice. The last two are the resolved halves, so the pre-paint script picks
//! a theme without knowing which families exist. Both id fields are always
//! filled (a family missing a half repeats the other), and `""` is the `:root` default.

use super::choice::ThemeChoice;
use super::registry::ThemeFamily;

/// localStorage key. Changing it means changing [`super::PREPAINT_SCRIPT`] too.
pub const STORAGE_KEY: &str = "ferrox.theme";

/// The key webapp-lib 1.0 pre-releases used. Read once if [`STORAGE_KEY`] is
/// empty, so an app moving to ferrox-theme keeps the user's choice.
pub(crate) const LEGACY_STORAGE_KEY: &str = "webapp-lib.theme";

/// The stored form of `choice` showing `family`.
pub(crate) fn encode(choice: &ThemeChoice, family: &ThemeFamily) -> String {
    format!(
        "{}:{}:{}:{}",
        choice.family,
        choice.mode.as_str(),
        family.resolve(false),
        family.resolve(true),
    )
}

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

/// The stored value, if any, falling back to the legacy key. A blocked or
/// missing localStorage reads as `None`.
pub(crate) fn read() -> Option<String> {
    let storage = local_storage()?;
    let get = |key| storage.get_item(key).ok().flatten();
    get(STORAGE_KEY).or_else(|| get(LEGACY_STORAGE_KEY))
}

/// Stores `value`. Failure (private mode, quota) is ignored: the theme is
/// already applied, and only the next load's first paint is affected.
pub(crate) fn write(value: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.set_item(STORAGE_KEY, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ThemeMode;

    #[test]
    fn encodes_both_halves() {
        let jamaica = ThemeFamily { id: "jamaica", label: "J", light: Some("jamaica"), dark: Some("jamaica-dark") };
        let choice = ThemeChoice::new("jamaica", ThemeMode::System);
        assert_eq!(encode(&choice, &jamaica), "jamaica:system:jamaica:jamaica-dark");
    }

    #[test]
    fn default_light_half_is_empty() {
        let patina = ThemeFamily { id: "patina", label: "P", light: Some(""), dark: Some("patina-dark") };
        assert_eq!(encode(&ThemeChoice::default(), &patina), "patina:system::patina-dark");
    }

    #[test]
    fn missing_half_repeats_the_other() {
        let foundry = ThemeFamily { id: "foundry", label: "F", light: Some("foundry"), dark: None };
        let choice = ThemeChoice::new("foundry", ThemeMode::Dark);
        assert_eq!(encode(&choice, &foundry), "foundry:dark:foundry:foundry");
    }
}
