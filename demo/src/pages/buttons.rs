//! Buttons, icon buttons and toolbars: every family, emphasis, size and state,
//! so a theme (or a change to buttons.css / toolbar.css) can be checked at once.

use std::time::Duration;

use leptos::prelude::*;
use ferrox_icons::{Icon, BAN, CAMERA, CLOSE, HELP, PLUS, SEARCH};
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
    let save = move || {
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
                <IconButton label="Search"><Icon icon=SEARCH /></IconButton>
                <IconButton label="Camera (toggle)" pressed=pressed on_click=move || pressed.update(|p| *p = !*p)>
                    <Icon icon=CAMERA />
                </IconButton>
                <IconButton label="Remove" danger=true><Icon icon=CLOSE /></IconButton>
                <IconButton label="Disabled" disabled=true><Icon icon=BAN /></IconButton>
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
                    <IconButton label="Search"><Icon icon=SEARCH /></IconButton>
                    <IconButton label="Camera" pressed=camera on_click=move || camera.update(|c| *c = !*c)>
                        <Icon icon=CAMERA />
                    </IconButton>
                    <IconButton label="Clear filter"><Icon icon=BAN /></IconButton>
                </ToolbarGroup>
                <ToolbarGroup>
                    <IconButton label="Help"><Icon icon=HELP /></IconButton>
                </ToolbarGroup>
            </Toolbar>
            <h3 class="demo-role-name">"compact, title and small buttons"</h3>
            <Toolbar compact=true title="Accounts">
                <IconButton label="Add"><Icon icon=PLUS /></IconButton>
                <Button size=ButtonSize::Sm family=ButtonFamily::Secondary>"Expand all"</Button>
            </Toolbar>
        </Section>
    }
}
