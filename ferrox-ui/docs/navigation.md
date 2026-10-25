# navigation (feature `navigation`)

Links that say where the user is, a side menu to hold them, and the app's
frame around it all (`AppShell`). The links are leptos_router's `<A>`, so
every component here goes inside a `<Router>`. The feature turns on
`overlay` (the side menu is a `Sheet`) and `layout` (the shell's
`AppBar`), and brings in leptos_router. It's the only group that does.


<!-- example: navigation -->
```rust
use ferrox_ui::{AppShell, NavGroup, NavLink};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

view! {
    <Router>
        <AppShell
            title="Groceries"
            href="/"
            nav=|| view! {
                <NavLink href="/" exact=true>"Lists"</NavLink>
                <NavLink href="/load">"Load Data"</NavLink>
                <NavGroup label="Product database" collapsible=true>
                    <NavLink href="/units">"Units"</NavLink>
                    <NavLink href="/categories">"Categories"</NavLink>
                    <NavLink href="/products">"Products"</NavLink>
                </NavGroup>
                <NavGroup label="Account">
                    <NavLink href="/account">"Account"</NavLink>
                    <NavLink href="/logout">"Logout"</NavLink>
                </NavGroup>
            }
        >
            <Routes fallback=|| "Not found">
                <Route path=path!("/") view=|| "Lists" />
            </Routes>
        </AppShell>
    </Router>
}
```
<!-- /example -->

The `AppShell` goes outside `<Routes>`, so the bar and the menu stay
mounted while the user moves between pages. Build the frame by hand from
`AppBar`, `SideMenuButton` and `SideMenu` only if `AppShell` can't do what
the app needs.

Which to use: [choosing.md](choosing.md#navigation).

## Pages

- [`AppShell`](app-shell.md)
- [`NavList`, `NavLink`](nav-list.md)
- [`NavGroup`](nav-group.md)
- [`SideMenu`, `SideMenuButton`](side-menu.md)
