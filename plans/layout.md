# layout: Panel, Card

The `layout` step of piece 6 in `plans/ferrox-ui.md`. It replaces
webapp-lib's `ComponentTitleBar` with a `Panel`: a titled box with its
actions and its body. It adds a `Card`, the item template for `data`'s
`DataView`. It ports conrad's `.panel` look and its
`CollapsiblePanel` (`ui/src/components/CollapsiblePanel.tsx`,
`theme/components.css:616-770`).

Status: approved (2026-10-23), all open points as recommended. Piece 1 done
(2026-10-23); piece 2 (collapsible) next.

Notes from piece 1:
- `HeadingLevel` lives in `layout/heading.rs`, with a crate-private
  `heading(level, class, id, children)` that renders the element, for
  `Panel` now and `Card` later.
- The title is styled the same at every level (the level is for the
  outline), ends in an ellipsis rather than wrapping under the buttons, and
  the band keeps a 3rem minimum height so a panel with no buttons lines up
  with one that has them.
- `children` is already `ChildrenFn`, so piece 2's lazy body doesn't change
  the signature. `actions` is `Option<ViewFn>`, as `SideMenu`'s `footer`.
- A help request names a panel as `panel`.
- Release check: 16 feature combinations, 380 e2e tests.

## Sources

| Today | Where | Becomes |
|---|---|---|
| `ComponentTitleBar { title: String, on_add, show_add, children }`: a primary-fill bar with a `+` on the left, any extra buttons (children) after it, and the title centred. The body isn't part of it: it's whatever the page puts next. The `+` is shown and hidden with an inline `style:display` (S2) | webapp-lib `components/mod.rs:184`, `style/base.css:190-225` | `Panel`: the bar *and* the body. Title on the left, actions on the right (as `Toolbar`), the `+` an `IconButton` drawing `PLUS` |
| Its uses, 12 across three apps. As a **page's title** (Recipes, Planner, Shopping Lists, Load Data, Reload Data: some with `show_add=false`, a bar with no button) and as a **section's title** inside a page (Shared with, Sub-stores, Items, Permissions). `show_add` follows a permission (`is_admin`, `can_add_items`) | rusty-skillet, shoppinglist-app, lesmagasins | `Panel` for both; `heading_level` says which |
| conrad `.panel`: a card (`--surface-1`, a border, `--radius-lg`), a header band on `--surface-2` in display caps, the body padded; `--compact`, `--warning`, `--danger` | `components.css:616-770` | `Panel`'s look, `compact`, `tone` |
| conrad `CollapsiblePanel`: a button *inside* the heading (the outline stays), `aria-expanded`, a `summary` said while shut, the body not rendered until first opened, then kept and `hidden` | `CollapsiblePanel.tsx` | `Panel collapsible` |
| nothing: each app's list cells are hand-made `<div>`s inside webapp-lib's `DataGrid` (or rusty-skillet's `DataListView`) | apps | `Card`, the template those cells become |

Review items covered: S2 (no inline style), X7 (the `+` glyph class becomes
an SVG icon), and the title becomes a real heading.

## Design

### Panel: a heading, its actions, its body

```rust
#[component]
pub fn Panel(
    #[prop(into)] title: Signal<String>,
    /// The heading's level: H2 by default; H1 for the page's own panel.
    #[prop(optional)] heading_level: HeadingLevel,
    /// Buttons at the right of the bar (`IconButton`s, a small `Button`).
    #[prop(optional, into)] actions: Option<ViewFn>,
    /// Adds a `+` `IconButton` as the last action, labelled `add_label`.
    #[prop(optional, into)] on_add: Option<UnsyncCallback<()>>,
    #[prop(into, default = "Add".to_string())] add_label: String,
    /// Whether the `+` shows: a permission (`is_admin`). Hidden, not disabled.
    #[prop(into, default = true.into())] can_add: Signal<bool>,
    /// A slimmer bar and body, for one panel among several on a working screen.
    #[prop(optional)] compact: bool,
    /// No padding round the body, for a body that is a table or a grid.
    #[prop(optional)] flush: bool,
    /// `Warning`: something to act on. `Danger`: something that can't be undone.
    #[prop(optional)] tone: PanelTone,
    #[prop(optional, into)] help: MaybeHelp,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView
```

```rust
<Panel title="Shopping Lists" heading_level=HeadingLevel::H1 on_add=move || new_list()>
    <ListGrid … />
</Panel>

<Panel title="Items" can_add=can_add_items on_add=move || new_item()
       actions=move || view! { <IconButton title="Search items" …><Icon icon=SEARCH /></IconButton> }>
    …
</Panel>

<Panel title="Load Data" heading_level=HeadingLevel::H1>…</Panel>      // a bar with no buttons
```

- **Markup:** `<section class="panel" aria-labelledby=title>`, a
  `.panel__header` holding the `<hN class="panel__title">` (with its `?` when
  there's `help`) and the actions, then `.panel__body`.
- **The bar** follows `Toolbar`'s rule: title on the left, buttons on the
  right, the `+` last. The title no longer sits in the middle, and the `+`
  moves from the left to the right.
- **`can_add`** hides the `+` (no `style:display`). A user who may not add
  shouldn't be shown a button they can't use.
- **Look** (conrad): a card on `--surface-1` with a hairline border and
  `--radius-lg`, the header a band on `--surface-2`. Today's apps show a
  primary-fill bar instead (open point 1).
- `HeadingLevel` is `H1`…`H4`, `#[non_exhaustive]`, H2 by default.
  `PanelTone` is `Default`, `Warning`, `Danger`: a tinted band and border,
  and the danger title in `--danger-text`.

### Collapsible: a panel whose header opens and closes it

```rust
    #[prop(optional)] collapsible: bool,
    #[prop(optional)] start_expanded: bool,
    #[prop(optional)] expanded: Option<RwSignal<bool>>,
    /// Said in the bar while it's shut: "3 overdue".
    #[prop(optional, into)] summary: MaybeProp<String>,
```

```rust
<Panel title="Danger zone" tone=PanelTone::Danger collapsible=true summary="Delete this book">
    …
</Panel>
```

As `NavGroup` and conrad's `CollapsiblePanel`:

- **The button is inside the heading**, so the page's outline keeps the
  heading: `<h2><button aria-expanded aria-controls>title, summary,
  chevron</button></h2>`. The whole bar is the target.
- **The body isn't rendered until it's first opened**, so a shut panel that
  would load data doesn't. From then on it stays mounted and is `hidden`
  while shut, so whatever was typed in it survives.
- `start_expanded`, and `expanded` for an app that owns (or remembers) the
  state, as on `NavGroup`.
- **The actions sit outside the button** (a button in a button is invalid),
  to its right, and show only while open, the `+` too (open point 3).

### Card: a surface for one thing

Moved here from `core`'s inventory (2026-10-23). A card is a panel without
the header band: one record, one item, one choice. Its main job is to be the
item template of `data`'s `DataView` (a grid of cards, a list of rows), so it
comes before that group.

```rust
#[component]
pub fn Card(
    /// An optional heading at the top.
    #[prop(optional, into)] title: MaybeProp<String>,
    /// H3 by default: a card usually sits inside a panel or a page section.
    #[prop(default = HeadingLevel::H3)] heading_level: HeadingLevel,
    /// The whole card follows this link (a plain `<a href>`, which the
    /// router handles when there is one).
    #[prop(optional, into)] href: MaybeProp<String>,
    /// The whole card is this button. Not with `href`.
    #[prop(optional, into)] on_click: Option<UnsyncCallback<()>>,
    /// Buttons of its own (edit, delete), in the top-right corner; they stay
    /// separate targets on a card that is a link or a button.
    #[prop(optional, into)] actions: Option<ViewFn>,
    #[prop(optional)] compact: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView
```

```rust
<Card title=employee.name href=format!("/employees/{}", employee.id)
      actions=move || view! { <IconButton title="Remove" danger=true …><Icon icon=DELETE /></IconButton> }>
    <p>{employee.role}</p>
</Card>
```

- **Markup:** `<article class="card">`, the heading, the body. An
  `<article>` because each card stands on its own in a list of them.
- **A whole card as one target, done accessibly:** the link (or the button)
  is the *title*, so it's named by the title and read once. Its hit area is
  stretched over the whole card with a `::after`, so a click anywhere on
  the card follows it. The actions sit above that layer and stay separate
  targets. (Not a card wrapped in an `<a>`: that would make a screen reader
  read the whole card as one link, and buttons inside it are invalid.) A
  clickable card needs a `title`.
- **Look:** `--surface-1`, a hairline border, `--radius-lg` (as a panel).
  A clickable card answers hover and press in colour (its border and
  surface), never in size, and shows the focus ring round the whole card
  (`:focus-within` on the title's link).
- No `selected` state here: choosing records belongs to `DataView`, which
  will say it with the right roles.
- No router dependency: an `<a href>` inside a `<Router>` is routed anyway.

### The `layout` feature and CSS

- `layout = []` in `ferrox-ui/Cargo.toml`. No dependencies. The powerset goes
  from 8 to 16 combinations.
- `layout/panel.css`: `.panel`, `.panel--compact`, `.panel--flush`,
  `.panel--warning`, `.panel--danger`, `.panel__header`, `.panel__title`,
  `.panel__actions`, `.panel__body`, and for collapsible ones
  `.panel__toggle`, `.panel__summary`, `.panel__chevron`,
  `.panel__body[hidden]`. `layout/card.css`: `.card`, `.card--compact`,
  `.card--clickable`, `.card__title`, `.card__link` (the stretched
  target), `.card__actions`, `.card__body`. Class-only, tokens only. Hover and press tint the
  band in colour only (mixed into `--surface-2`, as conrad does, so it never
  turns translucent).
- `PLUS` and `CHEVRON_RIGHT` exist, so no new icons.

### What changes in webapp-lib

Its piece 3 keeps `ComponentTitleBar` for one release, as a deprecated
wrapper (a `Panel` bar with an empty body), so the three apps move to
`Panel` one page at a time. `Panel` wraps the body, so a page changes shape:
the content that followed the bar moves inside it.

## Pieces

Each one passes `scripts/release-check.sh` and can be reviewed on its own.
The demo gets a `/layout` page.

1. **Feature and Panel.** `HeadingLevel`, `PanelTone`, `panel.css`, the
   16-way powerset. e2e:
   - a `section` named by its heading, at the level asked for;
   - title on the left, actions on the right, the `+` last;
   - `on_add` runs once per click; `can_add=false` removes the `+`, and it
     comes back when the signal does;
   - no buttons: a bar with only the title, the same height;
   - `help` puts a `?` after the title;
   - `compact`, `flush` and each tone change the classes and the look;
   - a phone: the bar doesn't wrap the title under the buttons, the `+` is
     44px.
2. **Collapsible.** e2e:
   - the button is inside the heading, `aria-expanded` / `aria-controls`;
   - starts shut, the body not in the DOM until first opened;
   - closing hides it, and typed text survives reopening;
   - the summary shows only while shut;
   - Enter / Space; the chevron turns; `start_expanded`; a two-way
     `expanded`;
   - the actions and the `+` show only while open, and aren't inside the
     button.
3. **Card.** e2e:
   - an `article` with its heading at the level asked for;
   - `href`: the title is the link, named by the title; a click anywhere on
     the card follows it (client-side, in the demo's router);
   - `on_click`: the title is a button; a click anywhere runs it once;
     Enter and Space too;
   - the actions are separate targets: clicking one doesn't follow the
     card;
   - hover and press change colour only; the focus ring is round the whole
     card;
   - a plain card (no `href`, no `on_click`) has no link, button or
     pointer cursor.
4. **Docs.** A layout section in `ferrox-ui/README.md`, the root and demo
   READMEs, and this plan's status.

Not here:
- Tabs (conrad's), and Accordion (a stack of collapsible panels, optionally
  one open at a time): in this group, when an app needs them.
- Page frame (conrad's `PageFrame`: a head that stays, a body that scrolls),
  Card columns, Scroll area (inventory, when an app needs them).
- Carousel: not layout (it's `media`), and not until an app needs one.
- Remembering a collapsible panel's state in browser storage (conrad's
  `storageKey`): `expanded` covers it, as for `NavGroup`.

## Open points

1. **The look.** Recommended: conrad's card (a `--surface-2` band, title in
   display caps), the same for a page and a section; the page's own panel
   is told apart by its `H1`. The alternative keeps today's primary-fill bar
   as an option (`filled=true`), so the three apps don't change look while
   they move.
2. **Collapsible now** (piece 2), recommended: it's conrad's, small, and the
   same disclosure pattern as `NavGroup`. The alternative is later, as its
   own story.
3. **`on_add` on a shut collapsible panel:** recommended, the `+` shows only
   while open (with the other actions), so there's nothing to resolve. The
   alternative shows it while shut and opens the panel as it adds.
