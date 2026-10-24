# navigation (feature `navigation`)

Links that say where the user is, and a side menu to hold them. The links
are leptos_router's `<A>`, so every component here goes inside a
`<Router>`. The feature turns on `overlay` (the side menu is a `Sheet`) and
brings in leptos_router. It's the only group that does.


```rust
use ferrox_ui::{NavGroup, NavLink, SideMenu, SideMenuButton};

let menu = RwSignal::new(false);

view! {
    <Router>
        <header class="app-bar">
            <SideMenuButton open=menu />
            <span class="app-bar__title">"Groceries"</span>
        </header>
        <SideMenu open=menu title="Groceries">
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
        </SideMenu>
        <Routes fallback=|| "Not found">…</Routes>
    </Router>
}
```

Put the `SideMenu` outside `<Routes>`, beside the header, so it stays
mounted while the user moves between pages.

Which to use: [choosing.md](choosing.md#navigation).

## Pages

- [`NavList`, `NavLink`](nav-list.md)
- [`NavGroup`](nav-group.md)
- [`SideMenu`, `SideMenuButton`](side-menu.md)
