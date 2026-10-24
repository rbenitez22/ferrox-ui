# `Sheet`

A modal panel pinned to an edge, with a title bar, the × and a scrolling
body. It slides in over `--react-time`; under reduced motion it just appears,
and closing never waits on an animation. A click on the backdrop closes it,
but only if the press also started there, so a text selection dragged out of
the sheet doesn't close it.

```rust
use ferrox_ui::{Sheet, SheetSide};

view! {
    <IconButton label="Menu" on_click=move || menu.set(true)><Icon icon=MENU/></IconButton>
    <Sheet open=menu title="Menu">
        <nav>…</nav>
    </Sheet>
}
```

| Prop | Default | |
|---|---|---|
| `open`, `title` | required | `open`: `RwSignal<bool>`, closed by the backdrop, the × and Esc; `title` is shown and is the sheet's accessible name |
| `on_close` | none | told after that dismissive close |
| `side` | `Left` | `Left` / `Right`: full height, `min(20rem, 85vw)` wide. `Bottom`: full width, at most 85vh tall |
| `close_label` | `"Close"` | |
| `help` | none | a `?` in the title bar, before the × |
| `class` | | |

`sheet_classes(side, extra)` returns the class list.
