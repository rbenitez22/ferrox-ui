# `Stack` and `Cluster`

```rust
use ferrox_ui::{Align, Button, ButtonFamily, Cluster, Gap, Justify, Stack};

view! {
    <Stack gap=Gap::Lg>
        <Field label="Name"><Input value=name /></Field>
        <Field label="Minutes"><Input value=minutes kind=InputKind::Number /></Field>
        <Cluster justify=Justify::End>
            <Button family=ButtonFamily::Cancel on_click=move || cancel()>"Cancel"</Button>
            <Button on_click=move || save()>"Save"</Button>
        </Cluster>
    </Stack>
}
```

| | `Stack` | `Cluster` |
|---|---|---|
| Flow | a column | a row that wraps |
| `gap` | `Gap::Md` (16px) | `Gap::Sm` (8px) |
| `align` (across) | `Align::Stretch` (full width) | `Align::Center` |
| `justify` (along) | | `Justify::Start`; `Center`, `End`, `Between` |
| `class` | extra classes | extra classes |

`Gap` is on the spacing tokens: `None`, `Xs` (4px), `Sm` (8px), `Md`
(16px), `Lg` (24px), `Xl` (32px). `Align`: `Stretch`, `Start`, `Center`,
`End`. `stack_classes` and `cluster_classes` return the class lists.
