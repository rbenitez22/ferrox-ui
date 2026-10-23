//! Browser side effects: the `data-theme` attribute and the
//! `prefers-color-scheme` media query.

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

const THEME_ATTRIBUTE: &str = "data-theme";
const DARK_QUERY: &str = "(prefers-color-scheme: dark)";

/// Sets `data-theme` on `<html>`, or removes it for `""` (the `:root` default).
pub(crate) fn set_theme_attribute(id: &str) {
    let Some(root) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) else { return };
    let _ = if id.is_empty() {
        root.remove_attribute(THEME_ATTRIBUTE)
    } else {
        root.set_attribute(THEME_ATTRIBUTE, id)
    };
}

fn dark_query() -> Option<web_sys::MediaQueryList> {
    web_sys::window()?.match_media(DARK_QUERY).ok()?
}

/// Whether the device asks for dark now. `false` when `matchMedia` is unavailable.
pub(crate) fn prefers_dark() -> bool {
    dark_query().is_some_and(|query| query.matches())
}

/// Calls `on_change` with the new preference whenever the device flips.
/// The listener lives for the rest of the page's life.
pub(crate) fn watch_prefers_dark(on_change: impl Fn(bool) + 'static) {
    let Some(query) = dark_query() else { return };
    let listener = Closure::<dyn Fn(web_sys::Event)>::new(move |_| on_change(prefers_dark()));
    let _ = query.add_event_listener_with_callback("change", listener.as_ref().unchecked_ref());
    listener.forget();
}
