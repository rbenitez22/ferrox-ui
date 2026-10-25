# `AppShell`

The app's frame: a skip link, the `AppBar` (the menu button, the app's
name, `bar_end`), the `SideMenu` holding `nav`, then the page in the app's
one `<main>`. The app brings the content; the frame and how it behaves are
the same in every app.

<!-- example: app-shell -->
```rust
use ferrox_ui::{AppShell, Menu, MenuItem, MenuTrigger, NavLink};
use leptos_router::components::Outlet;

view! {
    <AppShell
        title="Cookbook"
        href="/"
        nav=|| view! {
            <NavLink href="/" exact=true>"Recipes"</NavLink>
            <NavLink href="/settings">"Settings"</NavLink>
        }
        bar_end=|| view! {
            <Menu label="Account" trigger=MenuTrigger::custom(|| view! { <span>"Ada Lovelace"</span> }, "Account")>
                <MenuItem on_select=|| ()>"Sign out"</MenuItem>
            </Menu>
        }
        menu_footer=|| view! { <span>"Version 1.4"</span> }
    >
        <Outlet />
    </AppShell>
}
```
<!-- /example -->

| Prop | Default | |
|---|---|---|
| `title` | | the app's name: in the bar, and the side menu's title |
| `href` | | where the name links (usually `"/"`) |
| `nav` | | the side menu's links (`NavLink`, `NavGroup`) |
| `bar_end` | | the bar's end: actions, a user `Menu` |
| `menu_footer` | | pinned to the bottom of the side menu |
| `skip_label`, `menu_label`, `nav_label` | `"Skip to content"`, `"Menu"`, `"Sections"` | words |
| children | | the page: usually the router's `<Outlet/>` |

- **The menu** opens from the bar's button and closes itself when a link
  in it is followed (see [`SideMenu`](side-menu.md)).
- **The skip link** is the first Tab stop: out of sight until focused, it
  moves focus to the page, so a keyboard user skips the bar.
- **One `<main>`.** A `PageGrid` inside doesn't add its own (its side
  column stays an `<aside>`).
- It goes inside a `<Router>`, outside `<Routes>`, so the frame stays
  mounted between pages.
- Sign-in state, the user menu's items and which pages exist are the
  app's: slots and children. The side menu slides in at every width.
