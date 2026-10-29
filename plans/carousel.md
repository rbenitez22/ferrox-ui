# media: Carousel

A new component from the inventory (`plans/ferrox-ui.md`, group **media**:
"Carousel … probably not needed"), now that an app needs it: garcon-dashboard's
launcher cuts off its ninth tile on a phone. The user chose a carousel (pages
swiped sideways, like a phone's home screen) over paging `DataView`
(2026-10-28).

Status: approved 2026-10-28, open points as proposed (`v0.1.1`, group `media`). Piece 1 done (2026-10-28): `Carousel`, `CarouselLabels`, `carousel_pages`, `page_at`; `carousel.css`; docs `carousel.md`, `media.md`, the choosing row; demo `/media`; `media.spec.ts` (8 tests, Chromium + Firefox). The current page follows the track's scroll position; a page set by code or a control scrolls the track unless it is there already. Piece 2 done (2026-10-28): `phone_only`, `carousel_classes`, `track_classes`. As built: the track takes `page_class` too, so above a phone it is one grid (ferrox sets `display: grid`; `page_class` gives the columns and gap) and the pages and lists are `display: contents`; `page_class` must therefore be a grid. Demo "Launcher tiles"; 2 more e2e tests. Piece 3 done (2026-10-28): README (groups, requirements, testing, planned), root README, CHANGELOG 0.1.1, every crate and the agent kit at 0.1.1, the starter pinned to `v0.1.1`; `scripts/release-check.sh` passed; tagged `v0.1.1`.

## What it is

```text
phone (≤ 768px)                         desktop, phone_only=true
┌──────────────────────┐┊┌───           ┌────┐┌────┐┌────┐
│ ┌──────┐ ┌──────┐    │┊│ Prof         │ 1  ││ 2  ││ 3  │
│ │  1   │ │  2   │    │┊└───           ├────┤├────┤├────┤
│ │ ...  │ │ ...  │    │┊               │ .. ││ .. ││ 9  │   one grid, no
│ └──────┘ └──────┘    │┊               └────┘└────┘└────┘   controls
└──────────────────────┘┊
   ‹    ●  ○    ›
```

- **Items in pages.** `items`, `key_of` and `item` as `DataView` takes them, and
  `per_page`. The carousel splits the items into pages of `per_page`; each
  page is a `<ul>` the app lays out with `page_class` (its own grid).
- **Swiping is the browser's.** The track scrolls sideways with CSS scroll
  snap, one page per snap. There is no gesture code, so touch, trackpad and
  Shift+wheel all work natively.
- **Controls.** Previous / Next (`IconButton`s, disabled at the ends) and one
  dot per page (buttons named "Page {n} of {count}", the current one
  `aria-current`). They scroll to the page, `smooth` unless
  `prefers-reduced-motion`. The current page follows the scroll position, so
  a swipe moves the dots. With one page there are no controls.
- **`phone_only`** (default `false`). Above 768px (ferrox's phone breakpoint,
  as `Pagination` uses) the pages flatten by CSS (`display: contents`), so the
  items flow into one grid and the controls hide. The DOM is the same on
  both, so no media query runs in Rust.
- **Accessibility.** A `role="region"` with `aria-roledescription="carousel"`
  and `label`. Each page is a group named "{n} of {count}". Off-screen pages
  stay in the tab order: tabbing into one scrolls it into view (scroll snap),
  and the dots follow.
- **`page`.** An optional `RwSignal<usize>` (1-based) for an app that wants
  to own or remember the page.
- Words are props (`label`, `labels: CarouselLabels { previous, next, page }`),
  extra classes go in `class`: ferrox's conventions.

## Pieces

Each piece is one commit, reviewed before the next.

1. **`Carousel`, always paged.** Feature `media` (no deps). `media/carousel.rs`
   with the pure `pages(len, per_page) -> Vec<Range<usize>>` and
   `page_at(scroll_left, width, count)` unit-tested; `carousel.css`
   (class-only, on the tokens); `docs/carousel.md` and `docs/media.md` with a
   compiled example; a row in the choosing table ("pages of things swiped
   sideways: a launcher, a gallery"); demo page `/media`; e2e in Chromium +
   Firefox: Next/Previous, the dots, a swipe (scroll) moving the dots, a
   keyboard Tab into page 2, one page leaving no controls.
2. **`phone_only`.** The flattening CSS, and e2e at desktop width (one grid,
   no controls) and phone width (paged).
3. **Release `v0.1.1`.** README (media section), CHANGELOG, the agent kit's
   catalogue; `scripts/release-check.sh` green; tag and push.

## After this, outside this repo

- **webapp-lib:** move its ferrox lines to `v0.1.1` (Cargo.toml, README,
  `docs/migrating-from-0.4.md`), its tests, tag `v1.0.0-alpha.3`.
- **garcon-dashboard:** webapp-lib `alpha.3` and ferrox `v0.1.1` with feature
  `media`; the launcher's grid becomes `<Carousel per_page=8 phone_only=true
  page_class="launcher__grid" …>`. Its plan: `plans/phone-tile-pages.md`.

## Open points

1. Version: `v0.1.1`, since the change only adds (a new feature group), or
   `v0.2.0`? Proposed: `v0.1.1`.
2. Group: a new `media` group, as the inventory has it, or `data` beside
   `DataView`? Proposed: `media`, as listed.
