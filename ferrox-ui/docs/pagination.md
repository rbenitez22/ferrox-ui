# `Pagination`

Moves through pages of anything: a `DataView`, a table, search results.

```text
‹  1  …  5  [6]  7  …  20  ›
```

<!-- example: pagination -->
```rust
use ferrox_ui::{Card, DataView, Pagination};

let page = RwSignal::new(1);
let page_count = Signal::derive(move || total.get().div_ceil(20));

view! {
    <DataView
        items=Signal::derive(move || slice(page.get()))
        key_of=|r: &Recipe| r.id.clone()
        item=|r: Recipe| view! { <Card title=r.name>"…"</Card> }
        label="Recipes"
    />
    <Pagination page=page page_count=page_count />
}
```
<!-- /example -->

| Prop | Default | |
|---|---|---|
| `page` | required | `RwSignal<usize>`, 1-based, two-way |
| `page_count` | required | `Signal<usize>` |
| `label` | `"Pages"` | the `<nav>`'s accessible name |
| `labels` | `PaginationLabels::default()` | `previous` ("Previous page"), `next` ("Next page"), `page` ("Page {n}", each number's name), `position` ("Page {page} of {count}", read beside the phone's `6 / 20`) |
| `class` | | extra classes on the `<nav>` |

- A `<nav aria-label>` with Previous, the numbers and Next. The current page
  is `aria-current="page"`. Previous and Next are disabled at the ends.
  With one page or none it renders nothing.
- **Which numbers show** is `page_window(current, count) -> Vec<PageItem>`
  (`Page(n)` or `Gap`), public and unit-tested. Up to seven pages all show.
  Beyond that it's always seven slots: the first and last, the current
  with one neighbour each side, and a gap (`…`, `aria-hidden`) for each
  hidden run, never a gap for a single page. Slots are a fixed width, so
  paging never moves Previous or Next.
- The numbers are keyed by page, so the one you pressed keeps focus as the
  window moves.
- **On a phone** (≤ 768px): Previous, `6 / 20`, Next, as 44px targets.
- It doesn't slice, fetch or route. The app owns `page`, so putting it in
  the URL is the app's choice. Pagination or infinite scroll is the app's
  call per list: pages where users return to a place (admin, results),
  infinite scroll for feeds.
