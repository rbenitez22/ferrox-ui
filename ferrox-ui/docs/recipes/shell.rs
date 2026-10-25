//! Recipe: an app shell. The store for every page, the app bar (the menu
//! button, the app's name linking home, a user menu), the side menu with the
//! app's pages, then the page (the router's `Outlet`). Each page brings its
//! own `PageGrid`, so it decides its width and side column.

use ferrox_ui::{AppBar, Menu, MenuItem, MenuLink, MenuTrigger, NavLink, SideMenu, SideMenuButton};
use leptos::prelude::*;
use leptos_router::components::Outlet;

use super::store::Store;

#[component]
pub fn RecipeShell() -> impl IntoView {
    Store::provide();
    let menu = RwSignal::new(false);
    let signed_in = RwSignal::new(true);

    view! {
        <AppBar title="Cookbook" href="/recipes" start=move || view! { <SideMenuButton open=menu /> }>
            <Menu
                label="Account"
                trigger=MenuTrigger::custom(|| view! { <span>"Ada Lovelace"</span> }, "Account")
                header=|| view! { "Signed in as ada@example.org" }
            >
                <MenuLink href="/recipes/settings">"Settings"</MenuLink>
                <MenuItem on_select=move || signed_in.set(false)>"Sign out"</MenuItem>
            </Menu>
        </AppBar>
        <SideMenu open=menu title="Cookbook" footer=|| view! { <a href="/">"Back to the ferrox-ui demo"</a> }>
            <NavLink href="/recipes" exact=true>"Recipes"</NavLink>
            <NavLink href="/recipes/settings">"Settings"</NavLink>
        </SideMenu>
        <Outlet />
    }
}
