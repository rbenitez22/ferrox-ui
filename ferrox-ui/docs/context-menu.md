# `ContextMenu`

The same panel, items and keyboard as `Menu`, opened from an area (a row,
a card) instead of a trigger.

```rust
use ferrox_ui::{ContextMenu, MenuItem};

view! {
    <ContextMenu label="Recipe actions" menu=move || view! {
        <MenuItem icon=EDIT on_select=move || edit()>"Edit"</MenuItem>
        <MenuItem icon=DELETE danger=true on_select=move || ask_delete()>"Delete"</MenuItem>
    }>
        <Card title=recipe.name.clone() href=…>…</Card>
    </ContextMenu>
}
```

| Prop | Default | |
|---|---|---|
| `label` | required | the menu's accessible name |
| `menu` | required | a closure returning the `MenuItem`s, `MenuLink`s and `MenuDivider`s |
| `header` | none | a view above the items |
| `disabled` | `false` | the browser's own menu stays |
| `class` | | extra classes on the area's wrapper |
| children | | the area |

- **Opening:** a right-click in the area, at the pointer (the browser's own
  menu is suppressed there, and only there); the Menu key or Shift+F10
  while focus is in the area, under the focused element; a long press on
  touch (half a second; a touch that moves, as in scrolling, never opens
  it, and the lift doesn't click what was under it).
- At the pointer it opens below and to the right, flipping left and up near
  the window's edges. The first item takes focus; closing (choosing, Esc,
  Tab) returns focus to what had it before. A press outside closes it
  without moving focus; a right-click elsewhere in the area moves it there.
- The wrapper is `display: contents`: the area keeps its own layout.
- **Nothing shows that a context menu exists.** Offer the same actions
  somewhere visible too, such as a row's ⋮ `Menu`. It's a shortcut, never
  the only way.
