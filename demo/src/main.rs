//! Demo app for ferrox-ui: every theme and component, wired the way an app is,
//! and the target of the Playwright e2e tests. One page per area.

mod app;
mod menu;
mod pages;
mod recipes;
mod section;
mod swatch;
mod theme_bar;

use ferrox_theme::ThemeChoice;

fn main() {
    ferrox_theme::register(include!(concat!(env!("OUT_DIR"), "/ferrox_themes.rs")));
    ferrox_theme::init();
    ferrox_ui::init();
    apply_theme_from_url();
    leptos::mount::mount_to_body(app::App);
    mark_ready();
}

/// `?theme=jamaica:dark` picks a theme on load, for links and screenshots.
fn apply_theme_from_url() {
    let Some(search) = web_sys::window().and_then(|w| w.location().search().ok()) else { return };
    let Some(raw) = theme_param(&search) else { return };
    ferrox_theme::set_choice(ThemeChoice::parse(raw, &ferrox_theme::families()));
}

/// The `theme` value in a `?a=b&theme=x` query string.
fn theme_param(search: &str) -> Option<&str> {
    search
        .trim_start_matches('?')
        .split('&')
        .find_map(|pair| pair.strip_prefix("theme="))
}

/// `<body data-ready="true">` once the app is mounted. e2e tests wait for it
/// instead of sleeping until the WASM bundle has loaded.
fn mark_ready() {
    let body = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.query_selector("body").ok().flatten());
    if let Some(body) = body {
        let _ = body.set_attribute("data-ready", "true");
    }
}
