# `SplitButton`

A main action and, joined to it, a chevron that opens related actions.

<!-- example: split-button -->
```rust
use ferrox_ui::{MenuItem, SplitButton};

view! {
    <SplitButton text="Save" on_click=move || save() menu_label="More save options" saving=saving>
        <MenuItem on_select=move || save_as_draft()>"Save as draft"</MenuItem>
        <MenuItem on_select=move || save_and_close()>"Save and close"</MenuItem>
    </SplitButton>
}
```
<!-- /example -->

| Prop | Default | |
|---|---|---|
| `text` | required | the main button's text |
| `on_click` | required | the main action |
| `menu_label` | required | the chevron's name, and the menu's |
| `family` | `Primary` | both halves |
| `saving` | `false` | the main button sweeps and refuses clicks; the menu can't open |
| `disabled` | `false` | both halves |
| `class` | | extra classes on the group |
| children | | the menu's items |

Two buttons, two tab stops, two names. The halves join into one shape
(square inner corners, a faint hairline between) and don't grow under the
pointer, so one never rides over the other. The menu hangs from the
group's end.
