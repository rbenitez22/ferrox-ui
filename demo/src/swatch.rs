//! A token chip: a sample painted with the token, its name, and the value the
//! browser computed for it (re-read whenever the active theme changes).

use leptos::prelude::*;
use ferrox_theme::use_theme;

/// The computed value of custom property `--{name}` on `<html>`.
fn computed_token(name: &str) -> String {
    web_sys::window()
        .and_then(|window| {
            let root = window.document()?.document_element()?;
            window.get_computed_style(&root).ok().flatten()
        })
        .and_then(|style| style.get_property_value(&format!("--{name}")).ok())
        .map(|value| value.trim().to_string())
        .unwrap_or_default()
}

/// `paint` is the inline style that shows the token (e.g. `background: var(--surface-0)`).
/// `sample` is optional text drawn inside the sample.
#[component]
pub fn Chip(token: String, paint: String, #[prop(optional)] sample: &'static str) -> impl IntoView {
    let theme = use_theme();
    let name = token.clone();
    let value = move || {
        theme.active.track();
        computed_token(&name)
    };
    view! {
        <figure class="demo-chip">
            <div class="demo-chip-sample" style=paint>{sample}</div>
            <figcaption>
                <code>{format!("--{token}")}</code>
                <span class="demo-chip-value">{value}</span>
            </figcaption>
        </figure>
    }
}
