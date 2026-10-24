# `Menu`

A list of actions behind a trigger, as ARIA's menu button has it.

```rust
use ferrox_icons::MORE_VERTICAL;
use ferrox_ui::{Menu, MenuDivider, MenuItem, MenuTrigger};

view! {
    <Menu label="Recipe actions" trigger=MenuTrigger::icon(MORE_VERTICAL, "More actions")>
        <MenuItem on_select=move || edit()>"Edit"</MenuItem>
        <MenuItem on_select=move || duplicate()>"Duplicate"</MenuItem>
        <MenuDivider />
        <MenuItem on_select=move || ask_delete()>"Delete"</MenuItem>
    </Menu>
}
```

| Prop | Default | |
|---|---|---|
| `label` | required | the menu's accessible name |
| `trigger` | required | `MenuTrigger::button("Export")` (a secondary `.btn` with a chevron; `MenuTrigger::Button { text, family }` for another family), `MenuTrigger::icon(icon, label)` (an icon button named by `label`), `MenuTrigger::custom(view, label)` (the caller's view in a plain button), `MenuTrigger::chevron(label, family)` (a `.btn` showing only a chevron: `SplitButton`'s second half) |
| `align` | `MenuAlign::End` | which edge of the trigger the panel lines up with: `End` for a menu at a row's end, `Start` for one read from its button |
| `header` | none | a view above the items, outside the menu itself (who is signed in) |
| `disabled` | `false` | the trigger is disabled |
| `class` | | extra classes on the `.menu` wrapper |
| children | | `MenuItem`s, `MenuLink`s and `MenuDivider`s |

- **Opening:** a click, Enter or Space on the trigger opens it with the first
  item focused; ↓ opens on the first item, ↑ on the last.
- **In the menu:** ↑ / ↓ move, wrapping; Home / End; a letter moves to the
  next item starting with it; Enter or Space chooses.
- **Closing:** choosing (the menu closes, focus returns to the trigger, then
  the action runs, so a dialog it opens takes focus from there), Esc (focus
  back to the trigger; an enclosing `Dialog` stays open), Tab (focus moves
  on), a press outside.
- **The panel** is a popover in the top layer: below the trigger, above
  when there's no room, lined up by `align`, kept inside the viewport,
  following the trigger while open. No `overflow` parent clips it. Items
  are built on first opening, then kept, so a long list of row menus costs
  little. On a phone, items are 44px tall.
- **Width:** at least 12rem, at most `--menu-max-width` (22rem by default,
  never wider than the window). Past it, a label ends in an ellipsis; a
  panel never scrolls sideways (a very long menu scrolls down). Set the
  variable on `:root` for every menu, or on one menu's `class`:
  `.wide-menu { --menu-max-width: 34rem; }`.
