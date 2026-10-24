# `PageGrid`

```rust
use ferrox_ui::{PageGrid, PageWidth, Panel, HeadingLevel};

view! {
    <PageGrid width=PageWidth::Wide side=move || view! { <Panel title="Filters">…</Panel> }>
        <Panel title="Recipes" heading_level=HeadingLevel::H1>…</Panel>
        <Panel title="Recently made">…</Panel>
    </PageGrid>
}
```

| Prop | Default | |
|---|---|---|
| `width` | `PageWidth::Medium` | the content's cap, centred: `Narrow` 40rem (a form, reading), `Medium` 64rem, `Wide` 80rem (grids of cards), `Full` |
| `side` | none | a side column, 18rem wide, beside the main one; under it on a phone (≤ 768px). Built once |
| `side_position` | `SidePosition::End` | `Start` puts the side column first (on a phone it still goes under the main one) |
| `landmark` | `true` | the main column is `<main>` and the side `<aside>`; `false` inside a shell that has its own `<main>` |
| `class` | | extra classes on the grid |
| children | | the main column |

The page's padding is `--space-lg` (`--space-md` on a phone, less at the
bottom). Both columns space their children by `--space-lg`, so panels need
no wrapper. `page_grid_classes(width, side, extra)` returns the class list.
