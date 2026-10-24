//! The demo's side menu: every page, grouped, as an app's menu would be. It
//! lives outside the routes, so it stays mounted while the user navigates.

use std::sync::LazyLock;

use ferrox_ui::{HelpTopic, NavGroup, NavLink, SideMenu};
use leptos::prelude::*;
use leptos_router::components::A;

static MENU_HELP: LazyLock<HelpTopic> = LazyLock::new(|| {
    HelpTopic::new("demo-menu", "The demo menu")
        .paragraph("Every page of the demo, grouped. Following a link closes the menu.")
});

#[component]
pub fn DemoMenu(open: RwSignal<bool>) -> impl IntoView {
    view! {
        <SideMenu
            open=open
            title="ferrox-ui demo"
            help=MENU_HELP.clone()
            footer=|| view! {
                // A plain router link, not a NavLink: the menu closes for it too.
                <A href="/help">"About help"</A>
            }
        >
            <NavLink href="/">"Themes"</NavLink>
            <NavGroup label="Components">
                <NavLink href="/buttons">"Buttons"</NavLink>
                <NavLink href="/icons">"Icons"</NavLink>
                <NavLink href="/feedback">"Feedback"</NavLink>
                <NavLink href="/overlay">"Overlay"</NavLink>
                <NavLink href="/forms">"Forms"</NavLink>
                <NavLink href="/help">"Help"</NavLink>
                <NavLink href="/layout">"Layout"</NavLink>
                <NavLink href="/data">"Data"</NavLink>
                <NavLink href="/menus">"Menus"</NavLink>
            </NavGroup>
            <NavLink href="/recipes">"Recipes"</NavLink>
            <NavGroup label="Navigation pages" collapsible=true>
                <NavLink href="/navigation" exact=true>"Navigation"</NavLink>
                <NavLink href="/navigation/details">"Details"</NavLink>
            </NavGroup>
        </SideMenu>
    }
}
