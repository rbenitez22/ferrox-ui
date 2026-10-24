//! The app's entry point: the theme and the components' CSS first, then
//! mount.

mod app;

fn main() {
    ferrox_theme::init();
    ferrox_ui::init();
    leptos::mount::mount_to_body(app::App);
    mark_ready();
}

/// `<body data-ready="true">` once mounted, for end-to-end tests to wait on.
fn mark_ready() {
    let body = web_sys::window().and_then(|w| w.document()).and_then(|d| d.body());
    if let Some(body) = body {
        let _ = body.set_attribute("data-ready", "true");
    }
}
