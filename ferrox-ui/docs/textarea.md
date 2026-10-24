# `Textarea`

A multi-line text field (`.form-textarea`). It's resized by hand
(vertically), and Enter is a newline: a `Dialog` never submits from it.

<!-- example: textarea -->
```rust
use ferrox_ui::{Field, Textarea};

view! { <Field label="Notes"><Textarea value=notes rows=6 /></Field> }
```
<!-- /example -->

Props: `value` (required), `rows` (4), `placeholder`, `name`, `required`,
`readonly`, `disabled`, `autofocus`, `class`.
