//! Navigation: link lists that mark the current page. Mounted at
//! `/navigation` and `/navigation/details`, so a link can be current on a
//! path below its own.

use ferrox_icons::{HELP, MENU, SEARCH};
use ferrox_ui::{NavLink, NavList};
use leptos::prelude::*;

use crate::section::Section;

#[component]
pub fn NavigationPage() -> impl IntoView {
    view! {
        <LinkList />
        <WithIcons />
        <Nested />
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
