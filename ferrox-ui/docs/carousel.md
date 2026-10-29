# `Carousel`

Items in pages, swiped sideways: a launcher's tiles, a gallery.

```text
┌───────────────────────┐┊┌────
│ ┌─────┐ ┌─────┐       │┊│ 9
│ │  1  │ │  2  │  …    │┊└────
│ └─────┘ └─────┘       │┊
└───────────────────────┘┊
      ‹    ●  ○    ›
```

<!-- example: carousel -->
```rust
use ferrox_ui::{Card, Carousel};

view! {
    <Carousel
        items=apps()
        key_of=|a: &App| a.id.clone()
        item=|a: App| view! { <Card title=a.name href=a.url>"…"</Card> }
        per_page=8
        page_class="launcher__grid"
        label="Apps"
    />
}
```
<!-- /example -->

| Prop | Default | |
|---|---|---|
| `items` | required | `Signal<Vec<T>>` |
| `key_of` | required | `Fn(&T) -> String`, a stable key per item |
| `item` | required | `Fn(T) -> impl IntoView`, renders one item |
| `per_page` | required | `usize`, items to a page (`0` counts as `1`) |
| `label` | required | the carousel's accessible name |
| `page` | its own | `RwSignal<usize>`, 1-based, two-way: pass one to own or remember the page |
| `labels` | `CarouselLabels::default()` | `previous` ("Previous page"), `next` ("Next page"), `page` ("Page {n} of {count}", each dot's name), `slide` ("{n} of {count}", each page's name) |
| `page_class` | | classes on each page's `<ul>`: the app's layout for one page, usually a grid |
| `class` | | extra classes on the carousel |

- **Pages.** The items are split into pages of `per_page`
  (`carousel_pages(len, per_page)`, public and unit-tested). Each page is a
  `<ul class="carousel__items">` with `page_class`, and each item an
  `<li class="carousel__item">`; the app lays the page out.
- **Swiping is the browser's.** The track scrolls sideways and snaps one
  page at a time (CSS scroll snap), so touch, a trackpad and Shift+wheel
  all page it, with no gesture code. Paging glides, unless the user turns
  motion down.
- **Controls.** Previous and Next (disabled at the ends) and a dot per page
  scroll to a page. The current dot is `aria-current="page"`. A swipe moves
  the dots: the page follows the scroll position (`page_at`). With one
  page or none there are no controls.
- **Accessibility.** A `<section aria-roledescription="carousel">` named by
  `label`; each page a `role="group"` slide named "1 of 2". Pages off
  screen stay in the tab order: tabbing into one scrolls it into view and
  the dots follow.
- It doesn't fetch, route or autoplay. The app owns `items`, and `page` if
  it passes one.
