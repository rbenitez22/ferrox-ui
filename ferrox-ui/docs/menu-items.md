# Items

<!-- example: menu-items -->
```rust
use ferrox_icons::{DELETE, EDIT, MORE_VERTICAL};
use ferrox_ui::{Menu, MenuDivider, MenuItem, MenuLink, MenuTrigger};

view! {
    <Menu label="Actions" trigger=MenuTrigger::icon(MORE_VERTICAL, "More actions")>
        <MenuItem icon=EDIT on_select=move || edit()>"Edit"</MenuItem>
        <MenuItem on_select=move || archive() disabled=Signal::derive(move || !can_archive.get())>"Archive"</MenuItem>
        <MenuDivider />
        <MenuItem icon=DELETE danger=true on_select=move || ask_delete()>"Delete"</MenuItem>
    </Menu>

    <Menu label="Books" trigger=MenuTrigger::button("Book")>
        <MenuItem current=Signal::derive(move || book.get() == "Household") on_select=move || book.set("Household")>
            "Household"
        </MenuItem>
        <MenuLink href="/settings">"Settings"</MenuLink>
    </Menu>
}
```
<!-- /example -->

| | | |
|---|---|---|
| `MenuItem` | `on_select` (required), `icon`, `danger`, `disabled`, `current` | a `<button role="menuitem">` |
| `MenuLink` | `href` (required), `icon` | an `<a role="menuitem" href>`: inside a `<Router>` the router follows it, no page load. Space follows it too, as on a button |
| `MenuDivider` | | a `role="separator"` line between groups |

- **`icon`** is drawn before the text, muted.
- **`danger`**: a destructive action, in the danger colour with a danger
  tint on hover and focus.
- **`disabled`**: still shown and reachable with the arrows and letters (ARIA
  asks this of menu items), `aria-disabled`, faded; choosing it does
  nothing and the menu stays open.
- **`current`**: the one already in force (the open book). A check at the
  end, the primary's text colour, `aria-current="true"`, and still
  choosable: saying "you are here" isn't refusing to go there.

Choosing a `MenuItem` closes the menu and returns focus to the trigger
before `on_select` runs. Choosing a `MenuLink` closes the menu; the link
then navigates. `menu_item_classes(danger, current)` returns an item's
classes.
