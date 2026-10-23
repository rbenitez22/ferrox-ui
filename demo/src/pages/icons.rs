//! Every icon in ferrox-icons (`ALL`), icons at several font sizes, and one
//! icon chosen at runtime, so a theme or a new icon can be checked at once.

use ferrox_icons::{Icon, ALL, CHEVRON_DOWN, CHEVRON_RIGHT, SEARCH};
use ferrox_ui::Button;
use leptos::prelude::*;

use crate::section::Section;

#[component]
pub fn IconsPage() -> impl IntoView {
    view! {
        <Gallery />
        <Sizes />
        <ChosenAtRuntime />
    }
}

#[component]
fn Gallery() -> impl IntoView {
    view! {
        <Section title="All icons">
            <ul class="demo-icons">
                {ALL.iter().map(|icon| view! {
                    <li class="demo-icon">
                        <Icon icon=*icon />
                        <code>{icon.name}</code>
                    </li>
                }).collect_view()}
            </ul>
        </Section>
    }
}

/// The same icon at three font sizes: it is 1em, so it follows each.
#[component]
fn Sizes() -> impl IntoView {
    view! {
        <Section title="Size follows the font">
            <div class="demo-row">
                {["demo-text-16", "demo-text-24", "demo-text-32"].map(|class| view! {
                    <span class=class><Icon icon=SEARCH /></span>
                })}
            </div>
        </Section>
    }
}

/// A closure picks the icon; the drawing swaps when the signal changes.
#[component]
fn ChosenAtRuntime() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <Section title="Chosen at runtime">
            <div class="demo-row">
                <Button on_click=move |_| open.update(|o| *o = !*o)>
                    <Icon icon=move || if open.get() { CHEVRON_DOWN } else { CHEVRON_RIGHT } />
                    {move || if open.get() { "Collapse" } else { "Expand" }}
                </Button>
            </div>
        </Section>
    }
}
