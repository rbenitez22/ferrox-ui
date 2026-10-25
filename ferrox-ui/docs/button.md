# `Button`

A padded button (`.btn`). The **family** is what the button means. **Quiet**
is a separate axis: how heavy it looks.

<!-- example: button -->
```rust
use ferrox_ui::{Button, ButtonFamily, ButtonSize};

view! {
    <Button on_click=move || save()>"Save"</Button>
    <Button family=ButtonFamily::Cancel on_click=move || close()>"Cancel"</Button>
    <Button family=ButtonFamily::Secondary quiet=true size=ButtonSize::Sm>"Export"</Button>
    <Button saving=saving button_type="submit">"Create"</Button>
}
```
<!-- /example -->

| Prop | Default | |
|---|---|---|
| `family` | `Primary` | `Primary` (affirmative), `Secondary` (committing, not the main action), `Cancel` (dismissive, always an outline), `Danger` (destructive) |
| `quiet` | `false` | outline instead of fill |
| `size` | `Md` | `Sm`, `Md`, `Lg` |
| `saving` | `false` | a request is running: `aria-disabled` and a sweeping border. The button stays focusable and refuses clicks. It never sets `disabled`. |
| `disabled` | `false` | |
| `button_type` | `"button"` | not HTML's `"submit"`; pass `"submit"` for a form's submit button |
| `on_refused` | | runs when a press is refused because it's `saving` (from the keyboard: the pointer passes through a saving button) |
| `on_click`, `title`, `class` | | |

`button_classes(family, quiet, size, saving, extra)` returns the class list.
