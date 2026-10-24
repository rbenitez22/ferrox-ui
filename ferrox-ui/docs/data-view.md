# `DataView<T>`

A record list rendered through one item template and laid out as the screen
needs, with its loading, error and empty states beside it.

<!-- example: data-view -->
```rust
use ferrox_ui::{Card, DataLayout, DataView};

view! {
    <DataView
        items=recipes
        key_of=|r: &Recipe| r.id.clone()
        item=|r: Recipe| view! { <Card title=r.name.clone() href=format!("/recipes/{}", r.id)>"…"</Card> }
        label="Recipes"
        layout=DataLayout::default()
        loading=loading
        error=error
        on_retry=move || reload()
        empty_label="No recipes yet."
    />
}
```
<!-- /example -->

| Prop | Default | |
|---|---|---|
| `items` | required | `Signal<Vec<T>>`; `T: Clone + PartialEq + Send + Sync + 'static` |
| `key_of` | required | the item's identity, a `String` (an id, as text) |
| `item` | required | renders one item: a `Card` in a grid, a row in a list |
| `label` | required | the list's accessible name |
| `layout` | `Grid(CardSize::Md)` | `DataLayout::Grid(CardSize::{Sm,Md,Lg})` (columns at least 10 / 14 / 20rem, as many as fit), `List` (full-width rows, a hairline between), `Row` (a sideways strip that snaps), `Wrap` (content-sized, wrapping: tags, chips). May be reactive |
| `loading` | `false` | no items yet: a spinner (`loading_label`, `"Loading…"`) and no list; with items: they stay, the list is `aria-busy` |
| `error` | none | an error `Alert` above the list; items already shown stay |
| `on_retry` | none | adds Retry to the error (`retry_label`, `"Retry"`) |
| `empty_label` | `"Nothing here yet."` | shown when there are no items, no error and no loading |
| `empty_view` | none | a view shown instead of `empty_label`, e.g. a line and an Add button |
| `on_more` | none | infinite scroll: called when the end of the list comes within 300px of the viewport and `has_more` |
| `has_more` | `false` | there are more pages to fetch |
| `loading_more` | `false` | a page is on its way: a spinner at the end (`loading_more_label`, `"Loading more…"`), and no new calls |
| `class` | | extra classes on the root |

**Infinite scroll.** With `on_more`, a marker (`<li role="none">`) follows
the last item, watched by an `IntersectionObserver`. Each sighting asks for
at most one page, and never while `loading` or `loading_more`. When a page
lands and the marker is still in sight (a page too short to fill the
screen), it asks again, until the screen is full or `has_more` is false. It
works whatever scrolls: the window, or an `overflow: auto` box (there it
counts once the marker shows in the box; the 300px head start applies to
the viewport only). The observer is disconnected when the view unmounts.
The first page is the app's to load; `on_more` only extends a list that has
items.

<!-- example: data-view-more -->
```rust
use ferrox_ui::{DataLayout, DataView};

view! {
    <DataView
        items=entries
        key_of=|e: &Entry| e.id.clone()
        item=entry_row
        label="Activity"
        layout=DataLayout::List
        on_more=move || load_next_page()
        has_more=has_more
        loading_more=loading_more
    />
}
```
<!-- /example -->

The `<For>` is keyed by `key_of`. An item whose key and value stay keeps its
DOM node and state (a loaded image, typed text) through updates, appends,
re-orders and layout changes. An item whose value changed (by `PartialEq`)
is drawn again in its place, and the others are left alone, so key by id
and edits show. It doesn't fetch, sort or filter.
`data_view_classes(layout, extra)` returns the class list.
