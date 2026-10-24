# `IconButton`

A compact icon-only button (`.icon-btn`): 1.75rem square, no border. It
answers hover and press in colour, not size.

<!-- example: icon-button -->
```rust
use ferrox_icons::{Icon, CAMERA, DELETE, SEARCH};
use ferrox_ui::IconButton;

view! {
    <IconButton label="Search" on_click=move || open_search()><Icon icon=SEARCH /></IconButton>
    <IconButton label="Camera" pressed=camera on_click=move || camera.update(|c| *c = !*c)><Icon icon=CAMERA /></IconButton>
    <IconButton label="Remove line" danger=true on_click=move || remove()><Icon icon=DELETE /></IconButton>
}
```
<!-- /example -->

| Prop | |
|---|---|
| `label` | required: the button's only name (`aria-label`, and its tooltip) |
| `pressed` | makes it a switch (`aria-pressed`); on reads as tinted |
| `danger` | for a destructive action: drawn in the danger colour, tinted on hover |
| `disabled`, `on_click`, `class` | |
| children | the icon: ferrox-icons' `Icon`, or any `<svg class="icon">` |

`icon_button_classes(danger, extra)` returns the class list.
