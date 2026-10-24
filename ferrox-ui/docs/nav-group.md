# `NavGroup`

Links that belong together, under a heading that names them. A screen reader
hears "Product database, list, 3 items".

<!-- example: nav-group -->
```rust
use ferrox_ui::{NavGroup, NavLink};

let remember = RwSignal::new(false); // store it wherever the app keeps preferences

view! {
    <NavGroup label="Account"><NavLink href="/account">"Account"</NavLink></NavGroup> // plain
    <NavGroup label="Product database" collapsible=true> // starts collapsed
        <NavLink href="/units">"Units"</NavLink>
    </NavGroup>
    <NavGroup label="Reports" collapsible=true start_expanded=true>
        <NavLink href="/reports">"Reports"</NavLink>
    </NavGroup>
    <NavGroup label="Admin" collapsible=true expanded=remember> // the app owns it
        <NavLink href="/admin">"Admin"</NavLink>
    </NavGroup>
}
```
<!-- /example -->

| Prop | Default | |
|---|---|---|
| `label` | required | the heading, and the name of the group's list |
| `collapsible` | `false` | the heading is a button that shows and hides the links |
| `start_expanded` | `false` | where a collapsible group starts |
| `expanded` | none | `RwSignal<bool>`, two-way, owned by the app; overrides `start_expanded` |
| `class` | | |
| children | | `NavLink`s; one level only, no group inside a group |

- **Plain:** the heading is quiet text (small, muted), not focusable, and a
  hairline splits the group from what comes before it.
- **Collapsible:** the heading is a `<button aria-expanded aria-controls>`
  with a chevron that points right while collapsed and turns down once
  expanded. Enter and Space toggle it. A collapsed group's links are
  `hidden`, so Tab skips them.
- **The current page is never hidden.** When the route moves to one of a
  collapsed group's links, or the page opens on one, the group expands
  itself. That happens on arrival only, so the user can still collapse the
  group around the page they're on.
- **Inside a `SideMenu`** a group's own state starts over each time the
  menu opens (the menu's body is mounted only while open): collapsed, unless
  the current page is in it. To remember it, pass `expanded`.

`nav_group_classes(collapsible, extra)` returns the class list.
