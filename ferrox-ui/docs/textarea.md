# `Textarea`

A multi-line text field (`.form-textarea`). It's resized by hand
(vertically), and Enter is a newline: a `Dialog` never submits from it.

```rust
use ferrox_ui::Textarea;

view! { <Field label="Notes"><Textarea value=notes rows=6 /></Field> }
```

Props: `value` (required), `rows` (4), `placeholder`, `name`, `required`,
`readonly`, `disabled`, `autofocus`, `class`.
