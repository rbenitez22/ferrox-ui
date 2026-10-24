# `NavList`, `NavLink`

A labelled `<nav>` landmark holding a `<ul>`, one `<li>` per link, so a
screen reader hears the landmark's name and how many links it holds.

<!-- example: nav-list -->
```rust
use ferrox_icons::SEARCH;
use ferrox_ui::{NavLink, NavList};

view! {
    <NavList label="Settings">
        <NavLink href="/settings" exact=true>"General"</NavLink>
        <NavLink href="/settings/search" icon=SEARCH>"Search"</NavLink>
    </NavList>
}
```
<!-- /example -->

- `NavList`: `label` (required, the landmark's name), `class`, children
  (`NavLink`s and `NavGroup`s).
- `NavLink`: `href` (required), `exact`, `icon` (an `IconData`, drawn before
  the words and hidden from screen readers), `class` (on the link), children
  (the words). It renders an `<li>`, so it goes in a `NavList` or a
  `NavGroup`.

**The current page.** The router sets `aria-current="page"` on the link to
the page the user is on, and that's what tints it in the primary fill. A
link is current on its own path and on every path below it: `/accounts` is
current on `/accounts/42`. `exact` limits it to its own path, for a link
whose sub-pages have links of their own. `/` is current only on `/`.

Each link is a full-width row. It answers hover and press in colour only,
shows the theme's focus ring, and is at least 44px tall on a phone.
`nav_list_classes(extra)` and `nav_link_classes(extra)` return the class
lists.
