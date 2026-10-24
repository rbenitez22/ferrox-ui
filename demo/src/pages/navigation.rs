//! Navigation: link lists that mark the current page. Mounted at
//! `/navigation` and `/navigation/details`, so a link can be current on a
//! path below its own.

use ferrox_icons::{HELP, MENU, SEARCH};
use ferrox_ui::{Button, ButtonFamily, ButtonSize, NavGroup, NavLink, NavList};
use leptos::prelude::*;

use crate::section::Section;

#[component]
pub fn NavigationPage() -> impl IntoView {
    view! {
        <LinkList />
        <WithIcons />
        <Nested />
        <Groups />
        <GroupStates />
    }
}

/// Every demo page: this one is current, `/` is not.
#[component]
fn LinkList() -> impl IntoView {
    view! {
        <Section title="Link list">
            <div class="demo-nav-column">
                <NavList label="Demo pages">
                    <NavLink href="/">"Themes"</NavLink>
                    <NavLink href="/buttons">"Buttons"</NavLink>
                    <NavLink href="/icons">"Icons"</NavLink>
                    <NavLink href="/feedback">"Feedback"</NavLink>
                    <NavLink href="/overlay">"Overlay"</NavLink>
                    <NavLink href="/forms">"Forms"</NavLink>
                    <NavLink href="/help">"Help"</NavLink>
                    <NavLink href="/navigation">"Navigation"</NavLink>
                </NavList>
            </div>
        </Section>
    }
}

#[component]
fn WithIcons() -> impl IntoView {
    view! {
        <Section title="With icons">
            <div class="demo-nav-column">
                <NavList label="With icons">
                    <NavLink href="/icons" icon=SEARCH>"Icons"</NavLink>
                    <NavLink href="/help" icon=HELP>"Help"</NavLink>
                    <NavLink href="/navigation" icon=MENU>"Navigation"</NavLink>
                </NavList>
            </div>
        </Section>
    }
}

/// A link is current on the paths below its own, unless it's `exact`.
#[component]
fn Nested() -> impl IntoView {
    view! {
        <Section title="Paths below a link">
            <div class="demo-nav-column">
                <NavList label="Nested">
                    <NavLink href="/navigation">"Navigation and below"</NavLink>
                    <NavLink href="/navigation" exact=true>"Navigation only"</NavLink>
                    <NavLink href="/navigation/details">"Details"</NavLink>
                </NavList>
            </div>
        </Section>
    }
}

/// The shopping-list menu's shape: the app's own pages, then a collapsible
/// group (holding `/navigation/details`, so arriving there expands it) and a
/// plain one.
#[component]
fn Groups() -> impl IntoView {
    view! {
        <Section title="Groups">
            <div class="demo-nav-column">
                <NavList label="Grouped">
                    <NavLink href="/">"Themes"</NavLink>
                    <NavLink href="/forms">"Forms"</NavLink>
                    <NavGroup label="Product database" collapsible=true>
                        <NavLink href="/navigation/details">"Details"</NavLink>
                        <NavLink href="/buttons">"Buttons"</NavLink>
                        <NavLink href="/icons">"Icons"</NavLink>
                    </NavGroup>
                    <NavGroup label="Account">
                        <NavLink href="/help">"Help"</NavLink>
                        <NavLink href="/overlay">"Overlay"</NavLink>
                    </NavGroup>
                </NavList>
            </div>
        </Section>
    }
}

/// `start_expanded`, and a group whose state the page owns.
#[component]
fn GroupStates() -> impl IntoView {
    let remembered = RwSignal::new(false);
    view! {
        <Section title="Group states">
            <div class="demo-nav-column">
                <NavList label="Group states">
                    <NavGroup label="Starts expanded" collapsible=true start_expanded=true>
                        <NavLink href="/feedback">"Feedback"</NavLink>
                    </NavGroup>
                    <NavGroup label="Owned by the page" collapsible=true expanded=remembered>
                        <NavLink href="/overlay">"Overlay"</NavLink>
                    </NavGroup>
                </NavList>
            </div>
            <div class="demo-row">
                <Button family=ButtonFamily::Secondary size=ButtonSize::Sm on_click=move || remembered.update(|e| *e = !*e)>
                    "Toggle from code"
                </Button>
                <span>
                    {move || if remembered.get() { "Owned group: expanded" } else { "Owned group: collapsed" }}
                </span>
            </div>
        </Section>
    }
}
