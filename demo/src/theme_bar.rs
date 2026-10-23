//! Family and mode pickers. Plain `<select>`s on purpose: a theme picker
//! component belongs to a later library branch, not to the demo.

use leptos::prelude::*;
use ferrox_theme::{use_theme, ThemeMode};

const MODES: [ThemeMode; 3] = [ThemeMode::System, ThemeMode::Light, ThemeMode::Dark];

#[component]
pub fn ThemeBar() -> impl IntoView {
    let theme = use_theme();
    let family_options = theme
        .families()
        .into_iter()
        .map(|family| {
            view! {
                <option value=family.id prop:selected=move || theme.choice.get().family == family.id>
                    {family.label}
                </option>
            }
        })
        .collect_view();
    let mode_options = MODES
        .map(|mode| {
            view! {
                <option value=mode.as_str() prop:selected=move || theme.choice.get().mode == mode>
                    {mode.as_str()}
                </option>
            }
        })
        .collect_view();

    view! {
        <div class="demo-theme-bar">
            <label class="demo-inline-label">
                "Theme "
                <select on:change=move |ev| theme.set_family(&event_target_value(&ev))>{family_options}</select>
            </label>
            <label class="demo-inline-label">
                "Mode "
                <select on:change=move |ev| {
                    if let Some(mode) = ThemeMode::parse(&event_target_value(&ev)) {
                        theme.set_mode(mode);
                    }
                }>{mode_options}</select>
            </label>
            <span class="demo-active">
                "data-theme: "
                <code>{move || active_label(theme.active.get())}</code>
            </span>
        </div>
    }
}

/// `""` is the `:root` default, which has no attribute.
fn active_label(active: &str) -> String {
    if active.is_empty() { "(none, :root)".to_string() } else { active.to_string() }
}
