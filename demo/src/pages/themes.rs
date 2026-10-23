//! Every surface and role token of the active theme, painted and with values.

use leptos::prelude::*;

use crate::swatch::Chip;

const SURFACES: [&str; 5] = ["background", "surface-0", "surface-1", "surface-2", "surface-3"];
const TEXT: [&str; 3] = ["text", "text-muted", "text-subtle"];
const ROLES: [&str; 7] = ["primary", "secondary", "neutral", "info", "success", "warning", "danger"];
const CHARTS: [&str; 8] = [
    "chart-in", "chart-out", "chart-cat-1", "chart-cat-2", "chart-cat-3", "chart-cat-4", "chart-cat-5", "chart-other",
];

#[component]
pub fn ThemesPage() -> impl IntoView {
    view! {
        <section class="demo-section">
            <h2>"Surfaces"</h2>
            <div class="demo-chips">
                {SURFACES.map(|t| view! { <Chip token=t.to_string() paint=format!("background: var(--{t})") /> }).collect_view()}
            </div>
        </section>
        <section class="demo-section">
            <h2>"Text"</h2>
            <div class="demo-chips">
                {TEXT.map(|t| view! { <Chip token=t.to_string() paint=format!("color: var(--{t}); background: var(--surface-0)") sample="Aa" /> }).collect_view()}
                <Chip token="line".to_string() paint="border: 2px solid var(--line); background: var(--surface-0)".to_string() />
                <Chip token="focus-ring-color".to_string() paint="outline: var(--focus-ring-width) solid var(--focus-ring-color); outline-offset: var(--focus-ring-offset); background: var(--surface-0)".to_string() />
            </div>
        </section>
        <section class="demo-section">
            <h2>"Roles"</h2>
            {ROLES.map(|role| view! { <RoleRow role=role /> }).collect_view()}
        </section>
        <section class="demo-section">
            <h2>"Charts"</h2>
            <div class="demo-chips">
                {CHARTS.map(|t| view! { <Chip token=t.to_string() paint=format!("background: var(--{t})") /> }).collect_view()}
            </div>
        </section>
    }
}

/// One role's explicit slots (set by the theme) and derived slots (from tokens.css).
#[component]
fn RoleRow(role: &'static str) -> impl IntoView {
    let chip = move |slot: &str, paint: String, sample: &'static str| {
        view! { <Chip token=format!("{role}-{slot}") paint=paint sample=sample /> }
    };
    view! {
        <h3 class="demo-role-name">{role}</h3>
        <div class="demo-chips">
            {chip("base", format!("background: var(--{role}-base)"), "")}
            {chip("fill", format!("background: var(--{role}-fill); color: var(--{role}-on-fill)"), "on-fill")}
            {chip("fill-hover", format!("background: var(--{role}-fill-hover); color: var(--{role}-on-fill)"), "on-fill")}
            {chip("text", format!("color: var(--{role}-text); background: var(--surface-0)"), "Aa text")}
            {chip("tint", format!("background: var(--{role}-tint)"), "")}
            {chip("tint-strong", format!("background: var(--{role}-tint-strong)"), "")}
            {chip("line", format!("border: 2px solid var(--{role}-line); background: var(--surface-0)"), "")}
            {chip("glow", format!("box-shadow: var(--{role}-glow); background: var(--surface-0)"), "")}
        </div>
    }
}
