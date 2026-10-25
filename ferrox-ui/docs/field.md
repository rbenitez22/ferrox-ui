# `Field`

<!-- example: field -->
```rust
use ferrox_ui::{Field, Input};

view! {
    <Field label="Name" hint="As it appears on statements" error=name_error help=NAME_HELP.clone()>
        <Input value=name autofocus=true />
    </Field>
}
```
<!-- /example -->

| Prop | | |
|---|---|---|
| `label` | required | the `<label>`, pointing at the control |
| `hint` | none | guidance while the field is fine; hidden while there's an error |
| `error` | none | the field's problem. Present and not empty means invalid |
| `help`, `help_skip_tab`, `help_shares_value`, `help_reveal` | | a `?` after the label (see [the `help` prop](help.md#the-help-prop)); `help_reveal` shows it only while the field is in use ([`HelpReveal`](help.md#helpreveal-only-while-in-use)) |
| `class` | | appended to `.field` |
| children | | the one control |

The message is tied to the control by `aria-describedby`, so a screen reader
reads it with the control. An error never times out; it clears when the
field is right. `field_classes(extra)` returns the class list.
