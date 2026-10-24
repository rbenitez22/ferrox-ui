# `Card`

A surface for one thing: an `<article>` with an optional heading, its own
buttons, and the body. It's the item template for a list or a grid of cards.

```rust
use ferrox_icons::{Icon, DELETE};
use ferrox_ui::{Card, IconButton};

view! {
    <Card
        title=employee.name.clone()
        href=format!("/employees/{}", employee.id)
        actions=move || view! {
            <IconButton label="Remove" danger=true on_click=move || remove()><Icon icon=DELETE /></IconButton>
        }
    >
        <p>{employee.role.clone()}</p>
    </Card>

    <Card title="Groceries" on_click=move || open_list()>…</Card>   // the whole card is a button
    <Card title="Note" compact=true>…</Card>                         // just a surface
}
```

| Prop | Default | |
|---|---|---|
| `title` | none | the heading; required for a clickable card |
| `heading_level` | `H3` | a card usually sits inside a panel or a section |
| `href` | none | the whole card follows this link |
| `on_click` | none | the whole card is this button (not with `href`) |
| `actions` | none | a closure returning the card's own buttons, in its top-right corner |
| `compact` | `false` | tighter padding and a body-type title, for dense lists |
| `class` | | |
| children | | the body |

**A whole card as one target, done accessibly.** With `href` the *title* is
the link; with `on_click` it's a button. Its hit area is stretched over the
card, so a click anywhere follows it, but a screen reader meets one link or
button, named by the title, not the whole card read out as a link. The
`actions` sit above that layer and stay separate targets: pressing one never
follows the card. A clickable card answers hover and press in colour (its
border and surface) and shows the focus ring round the whole card. A
clickable card without a `title` logs a warning.

`href` is a plain `<a href>`: inside a `<Router>` the router handles it, so
`layout` doesn't depend on leptos_router. Choosing records (a selected card)
is left to the list that holds them. `card_classes(clickable, compact,
extra)` returns the class list.
