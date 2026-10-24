# `Input`

A one-line text field (`.form-input`).

<!-- example: input -->
```rust
use ferrox_ui::{Field, Input, InputKind};

view! {
    <Field label="Name"><Input value=name autofocus=true /></Field>
    <Input value=query kind=InputKind::Search placeholder="Search accounts" />
}
```
<!-- /example -->

| Prop | Default | |
|---|---|---|
| `value` | required | `RwSignal<String>` |
| `kind` | `Text` | `Text`, `Email`, `Search`, `Tel`, `Url`, `Number` (the `type`). Passwords are `PasswordInput` |
| `placeholder`, `name`, `autocomplete` | none | `autocomplete` is the browser hint (`"email"`, `"off"`, …) |
| `required` | `false` | |
| `readonly`, `disabled` | `false` | may be signals |
| `autofocus` | `false` | where a `Dialog` puts focus when it opens |
| `class` | | |
