# data: DataView, DataLayoutSwitch, Pagination

The `data` step of piece 6 in `plans/ferrox-ui.md`. A record list rendered
through one item template and laid out as the screen needs (a grid of cards,
a list of rows, a strip, a cloud of chips), with its loading, error and empty
states, infinite scroll, a layout switch and pagination.

Status: approved (2026-10-23), all open points as recommended. Piece 1 done
(2026-10-24): `DataView`, `DataLayout`, `CardSize`, `data-view.css`, demo
`/data`. Markup as built: the root is a `<div class="data-view
data-view--grid-md">` holding the states and the `<ul class="data-view__list">`
(hidden while there are no items); a `loading_label` and `retry_label` were
added. Piece 2 (infinite scroll) next.

## Where it comes from

The user's line of this idea, oldest first:

1. **PrimeFaces' `DataGrid` / `DataView`** (JSF, years ago): a record list
   plus one template for a record; the component lays the records out
   (grid, list) and pages them.
2. **webapp-lib's `DataGrid`** (`components/mod.rs:249`) and
   **`ListComponentView`** (`:275`): `data` + a `cell` closure, one `<div
   class="grid-cell">` per record; the second adds a spinner and an error
   line. Every change re-creates every row (review Q10), and the error is an
   inline-styled `<p>` (S2).
3. **rusty-skillet's `DataListView`** (`ui/src/gen_components.rs:534`), the
   user's attempt at improving it: no fixed classes, the caller's container
   class picks the layout, `empty_message`, `loading`, leading `children`,
   `on_scroll` for infinite scroll, a `node_ref`, and an optional
   `item_key` for a keyed `<For>`.

What this keeps and fixes:

| Today | Problem | `DataView` |
|---|---|---|
| `item_key` + keyed `<For>` | The `<For>` sits inside a closure that also reads `data()` (for the empty check), so every change rebuilds the `<For>` and the keys buy nothing | The list is rendered once; only the states beside it react. Keys are required, not optional |
| `class="rs-recipe-grid"` / `"rs-tag-cloud"` / `"rs-plan-list"`: each app writes its own grid, cloud and list CSS | Five layouts written per app | `layout`: `Grid`, `List`, `Row`, `Wrap`, built in |
| `on_scroll` + a 300px threshold, plus `auto_fill_grid_pages` (about 40 lines) to keep fetching while a page doesn't fill the screen, since the container never scrolls then | Every app re-implements it; it needs the scrolling element's `scroll` event | `on_more`: an `IntersectionObserver` on a marker after the last item. It fires when the marker comes into view, whichever element scrolls, and also when a short page leaves it in view, which covers the auto-fill case |
| `loading` replaces the list with a spinner | A refresh blanks a list already on screen | A spinner only for the first load; a refresh keeps the items and sets `aria-busy` |
| `error` as a red `<p>` | Inline style; no retry | An `Alert` (Error), with a Retry button when `on_retry` is given |
| `empty_message: &'static str` | Text only | `empty` (text) or `empty_view` (a view, e.g. "No recipes yet" + an Add button) |

Uses today: rusty-skillet 13 (`DataListView`: a recipe card grid with
infinite scroll, lists of plans, batches, steps and ingredients, tag clouds,
chips), shoppinglist-app 3 and lesmagasins 5 (`DataGrid` /
`ListComponentView`: lists of rows).

## Design

### DataView: one template, laid out

```rust
#[component]
pub fn DataView<T, IV>(
    #[prop(into)] items: Signal<Vec<T>>,
    /// Stable identity per item: the keyed rows (Q10).
    key_of: impl Fn(&T) -> String + Clone + Send + Sync + 'static,
    /// Renders one item, usually a `Card` (grid) or a row (list).
    item: impl Fn(T) -> IV + Clone + Send + Sync + 'static,
    /// The list's accessible name: "Recipes".
    #[prop(into)] label: Signal<String>,
    #[prop(into, default = DataLayout::default().into())] layout: Signal<DataLayout>,
    /// The first load: a spinner where the list will be.
    #[prop(optional, into)] loading: Signal<bool>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] on_retry: Option<UnsyncCallback<()>>,
    #[prop(into, default = "Nothing here yet.".into())] empty: Signal<String>,
    #[prop(optional, into)] empty_view: Option<ViewFn>,
    // infinite scroll: below
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
where T: Clone + Send + Sync + 'static, IV: IntoView + 'static
```

```rust
<Panel title="Recipes" heading_level=HeadingLevel::H1
       actions=move || view! { <DataLayoutSwitch layout=layout /> }>
    <DataView
        items=recipes
        key_of=|r: &Recipe| r.id.clone()
        item=|r: Recipe| view! { <Card title=r.name.clone() href=format!("/recipes/{}", r.id)>…</Card> }
        label="Recipes"
        layout=layout
        loading=loading
        error=error
        on_retry=move || reload()
        empty="No recipes yet. Use + to add one."
    />
</Panel>
```

- **Markup:** `<ul class="data-view data-view--grid" aria-label>` with one
  `<li class="data-view__item">` per item, from a keyed `<For>` rendered
  once. An item whose key stays keeps its DOM node and its state (an image
  that has loaded, a field being typed in) across updates, appends and
  re-orders.
- **Layouts** (`DataLayout`, `#[non_exhaustive]`):

  | Layout | Looks like | For |
  |---|---|---|
  | `Grid(CardSize)`, default `Grid(Md)` | as many columns as fit, each at least `Sm` 10rem / `Md` 14rem / `Lg` 20rem | cards: recipes, stores, employees |
  | `List` | one full-width row per item, a hairline between | rows: plans, permissions, steps |
  | `Row` | one horizontal strip that scrolls sideways, snapping to items | a few cards across: "recent" |
  | `Wrap` | items side by side, wrapping, sized to their content | tags, chips |

  Classes (`.data-view--grid-md`, `.data-view--list`, …), not inline
  styles. A layout change re-lays the same items; nothing is rebuilt.
- **States, beside the list, never instead of a list with items in it:**
  - `loading` with no items yet: a region `Spinner` (with `label`'s text).
  - `loading` with items: they stay; the list is `aria-busy`.
  - `error`: an `Alert` (Error) above the list, Retry if `on_retry`. Items
    already shown stay.
  - Empty, not loading, no error: `empty_view` if given, else `empty` as
    muted text (`.data-view__empty`).
- It doesn't fetch, sort or filter: whatever loads `items` owns that.

### Infinite scroll: `on_more`

```rust
    /// Called when the end of the list comes into view and `has_more`.
    #[prop(optional, into)] on_more: Option<UnsyncCallback<()>>,
    #[prop(optional, into)] has_more: Signal<bool>,
    /// A page is on its way: an inline spinner at the end, no new calls.
    #[prop(optional, into)] loading_more: Signal<bool>,
```

- A marker `<li>` after the last item, watched by an `IntersectionObserver`
  (with a margin, so the next page starts loading before the user reaches
  the end). It doesn't care which element scrolls (the window or an
  `overflow: auto` container).
- It fires again whenever the marker is still in view after a page lands,
  so a page too short to fill the screen pulls the next one: today's
  `auto_fill_grid_pages` comes for free.
- Never while `loading`, `loading_more`, or `!has_more`; at most one call per
  marker sighting. While `loading_more`, the marker shows an inline spinner.
- The observer is disconnected when the view unmounts.

### DataLayoutSwitch: the user picks the layout

```rust
#[component]
pub fn DataLayoutSwitch(
    layout: RwSignal<DataLayout>,
    /// The layouts offered, in order.
    #[prop(default = vec![DataLayout::Grid(CardSize::Md), DataLayout::List])] choices: Vec<DataLayout>,
    #[prop(optional)] labels: DataLayoutLabels,   // "Grid", "List", "Strip", "Cloud"
) -> impl IntoView
```

A group (`role="group"`, "Layout") of `IconButton`s with `pressed`, one per
choice. It sits wherever the app wants, typically a `Panel`'s actions. New
Lucide icons: `LAYOUT_GRID`, `LIST` (and `GALLERY_HORIZONTAL`, `TAGS` for
`Row` and `Wrap`). Remembering the choice is the app's (it owns the signal).

### Pagination

```rust
#[component]
pub fn Pagination(
    /// The current page, 1-based. Two-way.
    page: RwSignal<usize>,
    #[prop(into)] page_count: Signal<usize>,
    #[prop(into, default = "Pages".into())] label: Signal<String>,
    #[prop(optional)] labels: PaginationLabels,   // "Previous page", "Next page", "Page {n}"
) -> impl IntoView
```

```text
‹  1  …  4  5  [6]  7  8  …  20  ›
```

- `<nav aria-label="Pages">` with Previous, page numbers and Next; the
  current page is `aria-current="page"`. Previous / Next are disabled at the
  ends. It renders nothing when there's one page or none.
- Which numbers show is a pure function, unit-tested:
  `page_window(current, count) -> Vec<PageItem>` (`Page(n)` or `Gap`):
  always the first and last, the current with one neighbour each side, a
  `Gap` for any run of two or more hidden pages (a run of one shows the page
  instead), so its width stays the same while paging.
- On a phone: Previous, "6 / 20", Next.
- It pages anything (a `DataView`, a table, search results). The app slices
  or fetches the page. No router: putting the page in the URL is the app's
  choice (it owns `page`).
- New Lucide icon: `CHEVRON_LEFT`.

Pagination or infinite scroll is the app's choice per list: pages for lists
a user returns to a place in (admin, results), infinite scroll for feeds.

### The `data` feature and CSS

- `data = []`. It uses `core` (Spinner, Alert, IconButton). The powerset goes
  from 16 to 32 combinations.
- `data/data-view.css`, `data/pagination.css`. Class-only, tokens only.
- web-sys features: `IntersectionObserver`, `IntersectionObserverEntry`,
  `IntersectionObserverInit`.

### What changes in webapp-lib and the apps

webapp-lib's piece 3 keeps `DataGrid` and `ListComponentView` for one
release as deprecated wrappers over `DataView` (`List` layout, the record's
index as the key until callers pass one). rusty-skillet's `DataListView`
calls move to `DataView`, and `auto_fill_grid_pages` and the scroll
handlers go.

## Pieces

Each passes `scripts/release-check.sh`. The demo gets a `/data` page.

1. **Feature and DataView.** `DataLayout`, `CardSize`, `data-view.css`, the
   32-way powerset. e2e:
   - a list named by `label`, one item per record;
   - keyed: an item's DOM node (and text typed in it) survives an update, an
     append, a re-order and a layout change;
   - each layout's shape (columns that fit per card size, full-width rows, a
     sideways strip, wrapping chips);
   - first load: a spinner, no list; a refresh keeps the items, `aria-busy`;
   - error: an alert with Retry, which calls `on_retry` once; items stay;
   - empty: the text, or the given view.
2. **Infinite scroll.** e2e:
   - reaching the end calls `on_more` once, and the new items append;
   - a short first page keeps pulling pages until the screen is full or
     `has_more` is false;
   - nothing while `loading_more` (the spinner shows) or once `has_more` is
     false;
   - works inside an `overflow: auto` container, not only the window.
3. **DataLayoutSwitch.** Icons `LAYOUT_GRID`, `LIST`, `GALLERY_HORIZONTAL`,
   `TAGS`. e2e: a labelled group of pressed buttons; choosing one re-lays the
   view; the choice is two-way.
4. **Pagination.** `CHEVRON_LEFT`, `page_window` unit tests. e2e: numbers
   and gaps per position, `aria-current`, Previous / Next disabled at the
   ends, two-way `page`, hidden for one page, the phone form.
5. **Docs.** A data section in `ferrox-ui/README.md`, root and demo READMEs,
   this plan's status.

Not here:
- Selecting records (one or many; the roles that says) — open point 3.
- Table and DataTable (columns, sorting, filtering): inventory, when an app
  needs them.
- Virtualised rendering for thousands of items.

## Open points

1. **Layouts:** `Grid` (three card sizes), `List`, `Row`, `Wrap`.
   Recommended: they cover every shape the apps draw today plus the "row"
   you remembered from PrimeFaces. The alternative is Grid and List only.
2. **Infinite scroll inside `DataView`** (piece 2), recommended: it replaces
   each app's scroll handler and auto-fill with one observer. The
   alternative keeps it in the apps, as today.
3. **Selection later**, recommended: no app selects records in a list
   today, and doing it right means listbox or grid roles and their keys.
4. **Pagination with page numbers and gaps** (recommended), or Previous /
   Next only.
