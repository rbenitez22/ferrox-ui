# forms (feature `forms`)

A `Field` puts a label, one control and at most one message together. Each
control is bound two-way to an `RwSignal` (`String` for text, `bool` for a
checkbox or switch): the control writes what the user does, and setting the
signal from code updates the control.

**Where a message goes.** A problem one field can fix goes under that field
(`error`). Anything cross-field, or a server refusal no single field owns,
goes in an `Alert` at the top of the form. A problem never goes in both. A
field shows a hint *or* an error, never both: once the field is wrong, the
message under it says what's wrong.

<!-- example: forms -->
```rust
use ferrox_ui::{Alert, AlertSeverity, Field, Input, InputKind};

let email = RwSignal::new(String::new());
let email_error = Signal::derive(move || {
    let e = email.get();
    (!e.is_empty() && !e.contains('@')).then(|| "Enter an email address, like name@example.com".to_string())
});

view! {
    {move || refused.get().map(|why| view! { <Alert severity=AlertSeverity::Error>{why}</Alert> })}
    <Field label="Email" hint="Where receipts go" error=email_error>
        <Input value=email kind=InputKind::Email autocomplete="email" />
    </Field>
}
```
<!-- /example -->

**The wiring is automatic.** The control inside a `Field` takes the field's
id (so the `<label for>` names it), `aria-describedby` (the message, while
there is one) and, while there's an error, `aria-invalid` and `.is-invalid`.
Call sites pass no ids, and every field on a page gets a unique one. Put
**one control per field**. Only the field's own children see its wiring: a
control after the field, or outside any field, generates its own id and has
no description.

`Input`, `Textarea`, `PasswordInput`, `Select` and `Combobox` go inside a
`Field`. `Checkbox` and `Switch` carry their own label and don't.

| Component | Use it for |
|---|---|
| `Input` | one line of text: a name, an email, a search |
| `Textarea` | text that may run to several lines |
| `PasswordInput` | a password, to sign in or to set one (with a strength meter) |
| `Checkbox` | a yes/no that a later Save sends |
| `Switch` | an on/off that takes effect as it flips: a filter, a preference written straight back |
| `Select` | a short, fixed list of choices |
| `Combobox` | a long list, filtered by typing |

Which to use: [choosing.md](choosing.md#forms).

## Pages

- [`Field`](field.md)
- [`Input`](input.md)
- [`Textarea`](textarea.md)
- [`PasswordInput`](password-input.md)
- [`Checkbox`](checkbox.md)
- [`Switch`](switch.md)
- [`Select`](select.md)
- [`Combobox<T>`](combobox.md)
