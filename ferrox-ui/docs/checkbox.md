# `Checkbox`

A checkbox and its words as one `<label>`, so the words are part of the
target. **A checkbox states an intention that a later Save carries out.**
For a setting that takes effect the moment it flips, use a `Switch`.

<!-- example: checkbox -->
```rust
use ferrox_ui::Checkbox;

view! {
    <Checkbox checked=recurring label="Repeat every month" hint="On the same day each month" />
    <Checkbox checked=agreed label="I accept the terms" error=terms_error required=true />
}
```
<!-- /example -->

| Prop | | |
|---|---|---|
| `checked` | required | `RwSignal<bool>` |
| `label` | required | the words beside the box |
| `hint`, `error` | none | one message under it, as a `Field`'s (never both); an error also marks the box |
| `help`, `help_skip_tab`, `help_shares_value`, `help_reveal` | | a `?` after the words, outside the `<label>`, so clicking it never ticks the box. A shared value is `"true"` / `"false"`. `help_reveal`: see [`HelpReveal`](help.md#helpreveal-only-while-in-use) |
| `name`, `required`, `disabled`, `class` | | |

It's the native checkbox, coloured with `accent-color` and showing the
theme's focus ring. Space ticks it. `checkbox_classes(invalid, extra)`
returns the class list.
