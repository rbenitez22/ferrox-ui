//! Recipe: an app shell. The store for every page, then an `AppShell`: the
//! app's name linking home, a user menu at the bar's end, the side menu
//! with the app's pages, then the page (the router's `Outlet`). Each page
//! brings its own `PageGrid`, so it decides its width and side column.

use ferrox_ui::{AppShell, Menu, MenuItem, MenuLink, MenuTrigger, NavLink};
use leptos::prelude::*;
use leptos_router::components::Outlet;

use super::store::Store;

#[component]
pub fn RecipeShell() -> impl IntoView {
    Store::provide();
    let signed_in = RwSignal::new(true);

    view! {
        <AppShell
            title="Cookbook"
            href="/recipes"
            nav=|| view! {
                <NavLink href="/recipes" exact=true>"Recipes"</NavLink>
                <NavLink href="/recipes/settings">"Settings"</NavLink>
            }
            bar_end=move || view! {
                <Menu
                    label="Account"
                    trigger=MenuTrigger::custom(|| view! { <span>"Ada Lovelace"</span> }, "Account")
                    header=|| view! { "Signed in as ada@example.org" }
                >
                    <MenuLink href="/recipes/settings">"Settings"</MenuLink>
                    <MenuItem on_select=move || signed_in.set(false)>"Sign out"</MenuItem>
                </Menu>
            }
            menu_footer=|| view! { <a href="/">"Back to the ferrox-ui demo"</a> }
        >
            <Outlet />
        </AppShell>
    }
}
