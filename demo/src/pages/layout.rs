//! Layout: panels as a page's title and as sections, with their actions,
//! permissions, help and variants.

use std::sync::LazyLock;

use ferrox_icons::{Icon, SEARCH};
use ferrox_ui::{HeadingLevel, HelpTopic, IconButton, Panel, PanelTone, Switch};
use leptos::prelude::*;

use crate::section::Section;

static ITEMS_HELP: LazyLock<HelpTopic> = LazyLock::new(|| {
    HelpTopic::new("demo-items", "Items").paragraph("Everything stored in this place.")
});

#[component]
pub fn LayoutPage() -> impl IntoView {
    view! {
        <PagePanel />
        <SectionPanels />
        <Variants />
    }
}

/// A page's own panel (H1), with a `+` that counts its clicks.
#[component]
fn PagePanel() -> impl IntoView {
    let added = RwSignal::new(0);
    view! {
        <Section title="Page panel">
            <Panel title="Shopping Lists" heading_level=HeadingLevel::H1 on_add=move || added.update(|n| *n += 1)>
                <p>{move || format!("Added {} times", added.get())}</p>
            </Panel>
        </Section>
    }
}

/// Sections: actions before the `+`, a `+` that follows a permission, help;
/// and a panel with no buttons at all.
#[component]
fn SectionPanels() -> impl IntoView {
    let may_add = RwSignal::new(true);
    let searched = RwSignal::new(0);
    view! {
        <Section title="Section panels">
            <div class="demo-stack">
                <Switch checked=may_add label="May add items" />
                <Panel
                    title="Items"
                    can_add=may_add
                    on_add=move || {}
                    add_label="Add item"
                    help=ITEMS_HELP.clone()
                    actions=move || view! {
                        <IconButton title="Search items" on_click=move |_| searched.update(|n| *n += 1)>
                            <Icon icon=SEARCH />
                        </IconButton>
                    }
                >
                    <p>{move || format!("Searched {} times", searched.get())}</p>
                </Panel>
                <Panel title="Load Data">
                    <p>"A panel with no buttons."</p>
                </Panel>
                <Panel title="A very long panel title that cannot fit beside its buttons on a phone" on_add=move || {}>
                    <p>"The title ends in an ellipsis instead of wrapping."</p>
                </Panel>
            </div>
        </Section>
    }
}

#[component]
fn Variants() -> impl IntoView {
    view! {
        <Section title="Variants">
            <div class="demo-stack">
                <Panel title="Compact" compact=true on_add=move || {}>
                    <p>"One panel among several on a working screen."</p>
                </Panel>
                <Panel title="Flush" flush=true>
                    <p>"This body runs to the panel's edges."</p>
                </Panel>
                <Panel title="Warning" tone=PanelTone::Warning>
                    <p>"Something to act on."</p>
                </Panel>
                <Panel title="Danger" tone=PanelTone::Danger>
                    <p>"Something that can't be undone."</p>
                </Panel>
            </div>
        </Section>
    }
}
