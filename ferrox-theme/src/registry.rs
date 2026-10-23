//! Which themes exist: the built-in files, the app's registered files, and
//! the families (light + dark halves) they group into.

use std::cell::RefCell;

/// The family whose light half is `tokens.css`'s `:root` (no `data-theme`).
pub const DEFAULT_FAMILY: &str = "patina";
const DEFAULT_LABEL: &str = "Patina";

/// One theme file's metadata, generated at build time by `ferrox-theme-build`.
/// Its CSS is in the stylesheet the app links, not in the WASM.
///
/// Built from the file name: `jamaica-dark.css` →
/// `ThemeFile::new("jamaica-dark", "jamaica", true, "Jamaica")`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct ThemeFile {
    /// The `data-theme` value, equal to the file name without `.css`.
    pub id: &'static str,
    /// The id without a trailing `-dark`.
    pub family: &'static str,
    /// `true` for a `<family>-dark.css` file.
    pub dark: bool,
    /// Display name: title-cased family, or the file's `@label` comment.
    pub label: &'static str,
}

impl ThemeFile {
    /// Used by the code `ferrox-theme-build` generates.
    pub const fn new(id: &'static str, family: &'static str, dark: bool, label: &'static str) -> Self {
        ThemeFile { id, family, dark, label }
    }
}

/// A theme as the user picks it: a light half, a dark half, or both.
///
/// `light: Some("")` means `:root` itself (the default family's light half).
/// A family missing one half uses the other in both modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct ThemeFamily {
    pub id: &'static str,
    pub label: &'static str,
    pub light: Option<&'static str>,
    pub dark: Option<&'static str>,
}

impl ThemeFamily {
    /// The `data-theme` id to show. `""` means no attribute (the `:root` default).
    pub fn resolve(&self, wants_dark: bool) -> &'static str {
        let (first, second) = if wants_dark { (self.dark, self.light) } else { (self.light, self.dark) };
        first.or(second).unwrap_or("")
    }
}

const BUILTIN_THEMES: &[ThemeFile] = include!(concat!(env!("OUT_DIR"), "/ferrox_builtin_themes.rs"));

thread_local! {
    static REGISTERED: RefCell<Vec<ThemeFile>> = const { RefCell::new(Vec::new()) };
}

/// The themes shipped with ferrox-theme, sorted by id.
pub fn builtin_themes() -> &'static [ThemeFile] {
    BUILTIN_THEMES
}

/// Adds the app's own theme files. A file with a built-in's id replaces it; a
/// file registered twice keeps the later one.
pub(crate) fn register(files: &'static [ThemeFile]) {
    REGISTERED.with_borrow_mut(|registered| add_files(registered, files));
}

fn add_files(registered: &mut Vec<ThemeFile>, files: &[ThemeFile]) {
    for file in files {
        registered.retain(|existing| existing.id != file.id);
        registered.push(*file);
    }
}

/// Every theme file in effect: built-ins not replaced by the app, then the app's.
pub fn theme_files() -> Vec<ThemeFile> {
    REGISTERED.with_borrow(|registered| merge_files(BUILTIN_THEMES, registered))
}

/// Every family, default first, the rest sorted by id.
pub fn families() -> Vec<ThemeFamily> {
    group_families(&theme_files())
}

/// The family with `id`, if it exists.
pub fn find_family<'a>(families: &'a [ThemeFamily], id: &str) -> Option<&'a ThemeFamily> {
    families.iter().find(|family| family.id == id)
}

fn merge_files(builtin: &[ThemeFile], registered: &[ThemeFile]) -> Vec<ThemeFile> {
    builtin
        .iter()
        .filter(|file| !registered.iter().any(|app| app.id == file.id))
        .chain(registered)
        .copied()
        .collect()
}

fn group_families(files: &[ThemeFile]) -> Vec<ThemeFamily> {
    let mut families = vec![ThemeFamily { id: DEFAULT_FAMILY, label: DEFAULT_LABEL, light: Some(""), dark: None }];
    for file in files {
        let index = families.iter().position(|family| family.id == file.family).unwrap_or_else(|| {
            families.push(ThemeFamily { id: file.family, label: file.label, light: None, dark: None });
            families.len() - 1
        });
        let family = &mut families[index];
        if file.dark {
            family.dark = Some(file.id);
        } else {
            family.light = Some(file.id);
            family.label = file.label;
        }
    }
    families[1..].sort_by_key(|family| family.id);
    families
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(id: &'static str, family: &'static str, dark: bool) -> ThemeFile {
        ThemeFile::new(id, family, dark, "L")
    }

    #[test]
    fn builtins_group_into_families() {
        let families = group_families(BUILTIN_THEMES);
        let ids: Vec<_> = families.iter().map(|f| f.id).collect();
        assert_eq!(ids, ["patina", "classic", "foundry", "jamaica", "pl-ex"]);
        assert_eq!(families[0], ThemeFamily { id: "patina", label: "Patina", light: Some(""), dark: Some("patina-dark") });
        let foundry = find_family(&families, "foundry").unwrap();
        assert_eq!((foundry.light, foundry.dark), (Some("foundry"), None));
    }

    #[test]
    fn resolve_falls_back_to_the_other_half() {
        let light_only = ThemeFamily { id: "f", label: "F", light: Some("f"), dark: None };
        let dark_only = ThemeFamily { id: "d", label: "D", light: None, dark: Some("d-dark") };
        assert_eq!(light_only.resolve(true), "f");
        assert_eq!(dark_only.resolve(false), "d-dark");
        let default = ThemeFamily { id: "p", label: "P", light: Some(""), dark: Some("p-dark") };
        assert_eq!((default.resolve(false), default.resolve(true)), ("", "p-dark"));
    }

    #[test]
    fn app_file_replaces_builtin_with_same_id() {
        let builtin = [file("sea", "sea", false), file("sea-dark", "sea", true)];
        let app = [ThemeFile { label: "App Sea", ..file("sea", "sea", false) }];
        let merged = merge_files(&builtin, &app);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged.iter().find(|f| f.id == "sea").unwrap().label, "App Sea");
    }

    #[test]
    fn registering_the_same_id_twice_keeps_the_later_file() {
        let mut registered = Vec::new();
        add_files(&mut registered, &[file("sea", "sea", false)]);
        add_files(&mut registered, &[ThemeFile { label: "Again", ..file("sea", "sea", false) }]);
        assert_eq!(registered.len(), 1);
        assert_eq!(registered[0].label, "Again");
    }

    #[test]
    fn app_can_add_a_dark_half_to_a_builtin_family() {
        let families = group_families(&[file("foundry", "foundry", false), file("foundry-dark", "foundry", true)]);
        assert_eq!(find_family(&families, "foundry").unwrap().dark, Some("foundry-dark"));
    }

    #[test]
    fn label_comes_from_the_light_half() {
        let files = [
            ThemeFile { label: "Dark Label", ..file("sea-dark", "sea", true) },
            ThemeFile { label: "Sea", ..file("sea", "sea", false) },
        ];
        assert_eq!(find_family(&group_families(&files), "sea").unwrap().label, "Sea");
    }
}
