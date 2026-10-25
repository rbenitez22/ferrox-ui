# `Page`

A page: a `PageGrid` holding the page's own `Panel` (its `<h1>`, with
`actions` and `help` beside it, the children as its body), then `more`.
No styling of its own. Use one per route, for every page.

<!-- example: page -->
```rust
use ferrox_icons::{Icon, PLUS};
use ferrox_ui::{IconButton, Page, PageWidth, Panel};

view! {
    <Page
        title="Recipes"
        width=PageWidth::Wide
        actions=move || view! { <IconButton label="Add recipe" on_click=move || add()><Icon icon=PLUS /></IconButton> }
        side=move || view! { <Panel title="Filters">"…"</Panel> }
        more=move || view! { <Panel title="Recently made">"…"</Panel> }
    >
        "The page's own panel's body: the recipes."
    </Page>
}
```
<!-- /example -->

| Prop | Default | |
|---|---|---|
| `title` | | the page's title: its `<h1>`, and the tab's title. May be reactive |
| `width` | `Medium` | `Narrow` (a form, settings), `Medium`, `Wide` (grids of cards), `Full` |
| `actions` | | buttons beside the title |
| `side` | | a side column (under the main one on a phone) |
| `help` | | a `?` after the title |
| `more` | | panels under the page's own (a settings page's sections) |
| children | | the page's own panel's body |

- **The tab's title** follows `title`: "Recipes · Cookbook" inside an
  `AppShell` (the app's name last), "Recipes" outside. A detail page's
  title can arrive with its data.
- **After a navigation**, inside an `AppShell`, focus moves to the page's
  `<h1>`, so a screen reader announces the new page and Tab goes on from
  it. On the first load, and when a page re-renders on the same path (a
  loading state, then the record), focus stays where it is.
- **One per route.** A `Page` inside a `Page` logs a warning.
- A state with no title yet (a spinner while a record loads) can be a bare
  `PageGrid`; the `Page` comes with the data.
