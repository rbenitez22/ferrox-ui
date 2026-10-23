//! Buttons, icon buttons and toolbars: every family, emphasis, size and state,
//! so a theme (or a change to buttons.css / toolbar.css) can be checked at once.

use std::time::Duration;

use leptos::prelude::*;
use ferrox_ui::{Button, ButtonFamily, ButtonSize, IconButton, Toolbar, ToolbarGroup};

use crate::section::Section;

const SAVING_MS: u64 = 2000;

const FAMILIES: [(ButtonFamily, &str); 4] = [
    (ButtonFamily::Primary, "Primary"),
    (ButtonFamily::Secondary, "Secondary"),
    (ButtonFamily::Cancel, "Cancel"),
    (ButtonFamily::Danger, "Danger"),
];

#[component]
pub fn ButtonsPage() -> impl IntoView {
    view! {
        <Families />
        <Sizes />
        <States />
        <IconButtons />
        <Toolbars />
    }
}

#[component]
fn Families() -> impl IntoView {
    view! {
        <Section title="Families">
            <div class="demo-row">
                {FAMILIES.map(|(family, label)| view! { <Button family=family>{label}</Button> })}
            </div>
            <h3 class="demo-role-name">"quiet"</h3>
            <div class="demo-row">
                {FAMILIES.map(|(family, label)| view! { <Button family=family quiet=true>{label}</Button> })}
            </div>
        </Section>
    }
}

#[component]
fn Sizes() -> impl IntoView {
    view! {
        <Section title="Sizes">
            <div class="demo-row">
                <Button size=ButtonSize::Sm>"Small"</Button>
                <Button>"Medium"</Button>
                <Button size=ButtonSize::Lg>"Large"</Button>
            </div>
        </Section>
    }
}

/// Saving: click "Save" — it sweeps for 2s, stays focusable, and ignores
/// clicks (the counter does not move) until it is done.
#[component]
fn States() -> impl IntoView {
    let saving = RwSignal::new(false);
    let clicks = RwSignal::new(0u32);
    let save = move |_| {
        clicks.update(|n| *n += 1);
        saving.set(true);
        set_timeout(move || saving.set(false), Duration::from_millis(SAVING_MS));
    };
    view! {
        <Section title="States">
            <h3 class="demo-role-name">"disabled"</h3>
            <div class="demo-row">
                {FAMILIES.map(|(family, label)| view! { <Button family=family disabled=true>{label}</Button> })}
            </div>
            <h3 class="demo-role-name">"saving"</h3>
            <div class="demo-row">
                <Button saving=saving on_click=save>"Save"</Button>
                <Button family=ButtonFamily::Cancel>"Neighbour"</Button>
                <span>"clicks: " {move || clicks.get()}</span>
            </div>
        </Section>
    }
}

#[component]
fn IconButtons() -> impl IntoView {
    let pressed = RwSignal::new(true);
    view! {
        <Section title="Icon buttons">
            <div class="demo-row">
                <IconButton title="Search"><DemoIcon d=SEARCH /></IconButton>
                <IconButton title="Camera (toggle)" pressed=pressed on_click=move |_| pressed.update(|p| *p = !*p)>
                    <DemoIcon d=CAMERA />
                </IconButton>
                <IconButton title="Remove" danger=true><DemoIcon d=CROSS /></IconButton>
                <IconButton title="Disabled" disabled=true><DemoIcon d=BAN /></IconButton>
            </div>
        </Section>
    }
}

#[component]
fn Toolbars() -> impl IntoView {
    let camera = RwSignal::new(true);
    view! {
        <Section title="Toolbars">
            <h3 class="demo-role-name">"standard"</h3>
            <Toolbar title="Transactions">
                <Button family=ButtonFamily::Secondary quiet=true>"Export"</Button>
                <Button>"New transaction"</Button>
            </Toolbar>
            <h3 class="demo-role-name">"compact"</h3>
            <Toolbar compact=true>
                <ToolbarGroup>
                    <IconButton title="Search"><DemoIcon d=SEARCH /></IconButton>
                    <IconButton title="Camera" pressed=camera on_click=move |_| camera.update(|c| *c = !*c)>
                        <DemoIcon d=CAMERA />
                    </IconButton>
                    <IconButton title="Clear filter"><DemoIcon d=BAN /></IconButton>
                </ToolbarGroup>
                <ToolbarGroup>
                    <IconButton title="Help"><DemoIcon d=HELP /></IconButton>
                </ToolbarGroup>
            </Toolbar>
            <h3 class="demo-role-name">"compact, title and small buttons"</h3>
            <Toolbar compact=true title="Accounts">
                <IconButton title="Add"><DemoIcon d=PLUS /></IconButton>
                <Button size=ButtonSize::Sm family=ButtonFamily::Secondary>"Expand all"</Button>
            </Toolbar>
        </Section>
    }
}

// Demo-only SVGs until the SVG icon story lands: 24×24 strokes.
const SEARCH: &str = "M11 4a7 7 0 1 0 0 14a7 7 0 1 0 0-14z M21 21l-4.35-4.35";
const CAMERA: &str = "M4 8h3l2-3h6l2 3h3v11H4z M12 10a3.5 3.5 0 1 0 0 7a3.5 3.5 0 1 0 0-7z";
const BAN: &str = "M12 3a9 9 0 1 0 0 18a9 9 0 1 0 0-18z M5.6 5.6l12.8 12.8";
const HELP: &str = "M9.1 9a3 3 0 0 1 5.8 1c0 2-3 3-3 3 M12 17h.01";
const CROSS: &str = "M6 6l12 12 M18 6L6 18";
const PLUS: &str = "M12 5v14 M5 12h14";

#[component]
fn DemoIcon(d: &'static str) -> impl IntoView {
    view! {
        <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d=d />
        </svg>
    }
}
