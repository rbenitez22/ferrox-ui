# `Facts`, `Fact`

A record's facts, label beside value: a `<dl>` in two columns, the labels
muted. On a phone each label sits above its value.

<!-- example: facts -->
```rust
use ferrox_ui::{Fact, Facts};

view! {
    <Facts>
        <Fact label="Time">"30 minutes"</Fact>
        <Fact label="Serves">"4"</Fact>
    </Facts>
}
```
<!-- /example -->

`Facts` takes `class` and its `Fact`s; a `Fact` takes `label` (the `<dt>`),
`class`, and its value as children (the `<dd>`, any view: text, a link, a
badge).
