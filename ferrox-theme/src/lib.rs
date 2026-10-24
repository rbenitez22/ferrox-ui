//! Token themes for Leptos CSR apps, grouped into families with light and dark
//! halves, shown in `system`, `light`, or `dark` mode.
//!
//! # Usage
//!
//! The CSS (tokens, themes, fonts, loading screen) is a file the app links in
//! `index.html`, written by `ferrox-theme-build` from the app's `build.rs`.
//! This crate is the runtime: which themes exist, the user's choice, and the
//! `data-theme` attribute on `<html>`. Call once at startup, before mounting:
//!
//! ```rust,ignore
//! // Optional: the app's own themes, embedded by `ferrox_theme_build::embed_themes`.
//! ferrox_theme::register(include!(concat!(env!("OUT_DIR"), "/ferrox_themes.rs")));
//! ferrox_theme::init();
//!
//! // Later, e.g. from a settings page:
//! use ferrox_theme::{ThemeChoice, ThemeMode};
//! ferrox_theme::set_choice(ThemeChoice::new("jamaica", ThemeMode::System));
//! ```
//!
//! `ferrox-theme-build` also writes `prepaint.js` beside the stylesheet;
//! `index.html` inlines it (`<link data-trunk rel="inline"
//! href="style/ferrox/prepaint.js" />`) so the first paint is already themed.
//!
//! With the `leptos` feature, [`provide_theme`] / [`use_theme`] give Leptos
//! signals for the same state. Other code can watch changes with [`on_change`].
//!
//! The theme is applied as `data-theme="<id>"` on `<html>`. The default
//! family's light half is `:root` itself, so it has no attribute.

mod choice;
#[cfg(feature = "leptos")]
mod context;
mod dom;
mod listeners;
mod persist;
mod registry;
mod version;

use std::cell::{Cell, RefCell};

pub use choice::{ThemeChoice, ThemeMode};
#[cfg(feature = "leptos")]
pub use context::{provide_theme, use_theme, ThemeContext};
pub use listeners::ListenerId;
pub use persist::STORAGE_KEY;
pub use registry::{builtin_themes, families, theme_files, ThemeFamily, ThemeFile, DEFAULT_FAMILY};
pub use version::{check_build_version, VERSION};

use registry::find_family;

/// What is shown now: the user's choice and the theme id it resolved to.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ThemeState {
    choice: ThemeChoice,
    active: &'static str,
}

thread_local! {
    static STATE: RefCell<Option<ThemeState>> = const { RefCell::new(None) };
    static STARTING_FAMILY: Cell<&'static str> = const { Cell::new(DEFAULT_FAMILY) };
}

fn initialized() -> bool {
    STATE.with_borrow(Option::is_some)
}

/// Adds the app's own theme files (from `ferrox_theme_build::embed_themes`).
/// Call before [`init`]. A file with a built-in's id replaces it; registering
/// the same id twice keeps the later file.
pub fn register(files: &'static [ThemeFile]) {
    debug_assert!(!initialized(), "ferrox_theme::register() after init(): call it first");
    registry::register(files);
}

/// The family shown to a user who has never chosen one (default: Patina).
/// Call before [`init`]. A stored choice always wins.
pub fn set_starting_family(family: &'static str) {
    debug_assert!(!initialized(), "ferrox_theme::set_starting_family() after init(): call it first");
    STARTING_FAMILY.set(family);
}

/// Restores the stored choice (or the starting family), applies it, and starts
/// following the device's dark-mode setting. Call [`register`] first. A second
/// call does nothing.
pub fn init() {
    if initialized() {
        return;
    }
    let families = families();
    let choice = persist::read()
        .map(|raw| ThemeChoice::parse(&raw, &families))
        .unwrap_or_else(|| ThemeChoice::new(STARTING_FAMILY.get(), ThemeMode::System).normalized(&families));
    show(choice, &families);
    dom::watch_prefers_dark(|_| reapply_if_system());
}

/// The user's current choice (the default before [`init`]).
pub fn choice() -> ThemeChoice {
    STATE.with_borrow(|state| state.as_ref().map(|s| s.choice.clone())).unwrap_or_default()
}

/// The `data-theme` id shown now. `""` is the `:root` default.
pub fn active() -> &'static str {
    STATE.with_borrow(|state| state.as_ref().map_or("", |s| s.active))
}

/// Shows and stores `choice`. An unknown family falls back to the default family.
pub fn set_choice(choice: ThemeChoice) {
    let families = families();
    change(choice.normalized(&families), &families);
}

/// Switches family, keeping the current mode.
pub fn set_family(family: &str) {
    set_choice(ThemeChoice::new(family, choice().mode));
}

/// Switches mode, keeping the current family.
pub fn set_mode(mode: ThemeMode) {
    set_choice(ThemeChoice { mode, ..choice() });
}

/// Runs `listener(choice, active_id)` after every change: [`set_choice`] (and
/// its shorthands), or the device flipping light/dark in `system` mode.
/// Not called for the initial apply in [`init`].
pub fn on_change(listener: impl Fn(&ThemeChoice, &'static str) + 'static) -> ListenerId {
    listeners::add(listener)
}

/// Stops a listener added with [`on_change`].
pub fn remove_listener(id: ListenerId) {
    listeners::remove(id);
}

/// Resolves `choice` against the device preference, sets the attribute,
/// caches it for the pre-paint script, and records it as current.
fn show(choice: ThemeChoice, families: &[ThemeFamily]) -> ThemeState {
    let family = find_family(families, &choice.family).copied().unwrap_or(families[0]);
    let active = family.resolve(choice.mode.wants_dark(dom::prefers_dark()));
    dom::set_theme_attribute(active);
    persist::write(&persist::encode(&choice, &family));
    let state = ThemeState { choice, active };
    STATE.set(Some(state.clone()));
    state
}

/// [`show`], then tell listeners. State is recorded first, so a listener
/// reading [`choice`] or [`active`] sees the new values.
fn change(choice: ThemeChoice, families: &[ThemeFamily]) {
    let state = show(choice, families);
    listeners::notify(&state.choice, state.active);
}

/// Re-resolves after the device flips between light and dark. Only `system` mode cares.
fn reapply_if_system() {
    let current = choice();
    if current.mode == ThemeMode::System {
        change(current, &families());
    }
}
