# `AppBar`

The app's top band: what opens its menu, its name, and its actions (a user
menu, a search). A `<header>`, so at the top of the page it's the banner
landmark. The name is plain text (or a link home): the page's own `Panel`
carries the `<h1>`.

<!-- example: app-bar -->
```rust
use ferrox_ui::{AppBar, Menu, MenuItem, MenuTrigger, SideMenuButton};

let menu = RwSignal::new(false); // the SideMenu's, declared beside it

view! {
    <AppBar title="Cookbook" href="/" start=move || view! { <SideMenuButton open=menu /> }>
        <Menu label="Account" trigger=MenuTrigger::custom(|| view! { <span>"Ada Lovelace"</span> }, "Account")>
            <MenuItem on_select=move || sign_out()>"Sign out"</MenuItem>
        </Menu>
    </AppBar>
}
```
<!-- /example -->

| Prop | Default | |
|---|---|---|
| `title` | required | the app's name |
| `href` | none | makes the name a link, usually home |
| `start` | none | before the name: what opens the app's menu (a `SideMenuButton`). Built once |
| `sticky` | `true` | stays at the top of the window while the page scrolls; below every popover and modal |
| `class` | | extra classes on the `<header>` |
| children | | at the end: the app's actions |

The band is `--surface-1` with a hairline under it, padded `--space-lg` at
the sides (`--space-md` on a phone). The name takes the free space and ends
in an ellipsis rather than pushing the actions off. `app_bar_classes(sticky,
extra)` returns the class list.
