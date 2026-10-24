# `SideMenu`, `SideMenuButton`

A `Sheet` holding a `NavList` of the children, with an optional footer
pinned to the bottom. It shares an `open` signal with its button.

<!-- example: side-menu -->
```rust
use ferrox_ui::{NavLink, SideMenu, SideMenuButton};

view! {
    <SideMenuButton open=menu />
    <SideMenu
        open=menu
        title="Groceries"
        help=MENU_HELP.clone()
        footer=|| view! { <span>"Version 1.4"</span> }
    >
        <NavLink href="/" exact=true>"Lists"</NavLink>
    </SideMenu>
}
```
<!-- /example -->

**It closes itself**, like every modal, and also on its own:

- when the user follows a link inside it: a plain left click on any
  `<a href>` without a `target`, so a plain `<A>` closes it too. A ctrl- or
  cmd-click opens a new tab and leaves the menu open;
- when the current page's link is clicked, though the route doesn't change;
- on any route change while it's open, such as the back button;
- on Esc, the × or a backdrop click, as any `Sheet`.

Focus then returns to the `SideMenuButton`. After a navigation, moving focus
into the new page is the app's routing concern, not the menu's.

| `SideMenu` prop | Default | |
|---|---|---|
| `open` | required | `RwSignal<bool>`, shared with `SideMenuButton` |
| `title` | required | the title bar's text, and the sheet's accessible name |
| `nav_label` | `"Sections"` | the name of the `<nav>` inside |
| `side` | `Left` | a `SheetSide` |
| `footer` | none | a closure returning a view, pinned below the links behind a hairline: a version, a legal line |
| `help` | none | a `?` in the title bar, before the × |
| `close_label` | `"Close"` | the ×'s label |
| `class` | | on the sheet, after `side-menu` |
| children | | `NavLink`s and `NavGroup`s |

`SideMenuButton` is an `IconButton` drawing `MENU`, with
`aria-haspopup="dialog"` and an `aria-expanded` that follows `open`. Props:
`open` (required), `label` (`"Menu"`, its only label and its tooltip),
`class`. `side_menu_classes(extra)` returns the sheet's class list.

Who's signed in, the app's title bar and a page frame aren't here: the bar
is the app's (or webapp-lib's), and a page frame is `layout`'s, later.
