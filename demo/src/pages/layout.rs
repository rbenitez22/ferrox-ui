//! Layout: panels as a page's title and as sections, with their actions,
//! permissions, help and variants.

use std::sync::LazyLock;

use ferrox_icons::{Icon, DELETE, PLUS, SEARCH};
use ferrox_ui::{Button, ButtonFamily, ButtonSize, Card, Field, HeadingLevel, HelpTopic, IconButton, Input, Panel, PanelTone, Switch};
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
        <Collapsible />
        <Cards />
    }
}

/// The `+` an app puts in a panel's actions: an `IconButton` drawing `PLUS`.
fn add_button(label: &'static str, on_add: impl Fn() + Send + Sync + 'static) -> impl IntoView {
    view! {
        <IconButton label=label on_click=move |_| on_add()>
            <Icon icon=PLUS />
        </IconButton>
    }
}

/// A page's own panel (H1), with a `+` that counts its clicks.
#[component]
fn PagePanel() -> impl IntoView {
    let added = RwSignal::new(0);
    view! {
        <Section title="Page panel">
            <Panel title="Shopping Lists" heading_level=HeadingLevel::H1 actions=move || add_button("Add", move || added.update(|n| *n += 1))>
                <p>{move || format!("Added {} times", added.get())}</p>
            </Panel>
        </Section>
    }
}

/// Sections: a search action, then a `+` the page shows only while a
/// permission allows (an app's own `Show`), help; and a panel with no buttons
/// at all.
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
                    help=ITEMS_HELP.clone()
                    actions=move || view! {
                        <IconButton label="Search items" on_click=move |_| searched.update(|n| *n += 1)>
                            <Icon icon=SEARCH />
                        </IconButton>
                        <Show when=move || may_add.get()>{add_button("Add item", || {})}</Show>
                    }
                >
                    <p>{move || format!("Searched {} times", searched.get())}</p>
                </Panel>
                <Panel title="Load Data">
                    <p>"A panel with no buttons."</p>
                </Panel>
                <Panel title="A very long panel title that cannot fit beside its buttons on a phone" actions=|| add_button("Add", || {})>
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
                <Panel title="Compact" compact=true actions=|| add_button("Add", || {})>
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

/// Collapsible panels: a summary while shut, a body built on first opening
/// (counted), actions only while open, and the state owned by the page.
#[component]
fn Collapsible() -> impl IntoView {
    let builds = RwSignal::new(0);
    let note = RwSignal::new(String::new());
    let added = RwSignal::new(0);
    let owned = RwSignal::new(false);
    view! {
        <Section title="Collapsible">
            <div class="demo-stack">
                <p>{move || format!("Notes body built {} times; added {} times", builds.get(), added.get())}</p>
                <Panel
                    title="Notes"
                    collapsible=true
                    summary="2 notes"
                    help=ITEMS_HELP.clone()
                    actions=move || add_button("Add note", move || added.update(|n| *n += 1))
                >
                    <CountsBuilds builds=builds />
                    <Field label="Note"><Input value=note /></Field>
                </Panel>
                <Panel title="Danger zone" tone=PanelTone::Danger collapsible=true summary="Delete this book">
                    <p>"Deleting a book can't be undone."</p>
                </Panel>
                <Panel title="Opens expanded" collapsible=true start_expanded=true>
                    <p>"Open on arrival."</p>
                </Panel>
                <Panel title="Compact collapsible" compact=true collapsible=true summary="One line">
                    <p>"A slimmer band."</p>
                </Panel>
                <Panel title="Owned by the page" collapsible=true expanded=owned>
                    <p>"The page holds this state."</p>
                </Panel>
                <div class="demo-row">
                    <Button family=ButtonFamily::Secondary size=ButtonSize::Sm on_click=move |_| owned.update(|o| *o = !*o)>
                        "Toggle the owned panel"
                    </Button>
                    <span>{move || if owned.get() { "Owned panel: open" } else { "Owned panel: shut" }}</span>
                </div>
            </div>
        </Section>
    }
}

/// Counts how many times the body it sits in is built.
#[component]
fn CountsBuilds(builds: RwSignal<u32>) -> impl IntoView {
    builds.update(|n| *n += 1);
}

/// Cards in a grid: one that is a link, one that is a button, each with a
/// remove action of its own; a plain one; a compact one.
#[component]
fn Cards() -> impl IntoView {
    let removed = RwSignal::new(0);
    let opened = RwSignal::new(0);
    let remove = move || view! {
        <IconButton label="Remove" danger=true on_click=move |_| removed.update(|n| *n += 1)>
            <Icon icon=DELETE />
        </IconButton>
    };
    view! {
        <Section title="Cards">
            <p>{move || format!("Removed {} times; opened Grace {} times", removed.get(), opened.get())}</p>
            <div class="demo-card-grid">
                <Card title="Ada Lovelace" href="/navigation/details" actions=remove>
                    <p>"Analyst. The whole card is a link."</p>
                </Card>
                <Card title="Grace Hopper" on_click=move || opened.update(|n| *n += 1) actions=remove>
                    <p>"Admiral. The whole card is a button."</p>
                </Card>
                <Card title="Plain card">
                    <p>"Not a link, not a button."</p>
                </Card>
                <Card title="Compact card" compact=true>
                    <p>"For dense lists."</p>
                </Card>
            </div>
        </Section>
    }
}
