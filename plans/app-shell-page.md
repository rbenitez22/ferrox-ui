# AppShell and Page: the app's frame and a page's frame

The user's question (2026-10-25): do an app shell and a page component
belong in ferrox-ui? Yes, kept thin: ferrox-ui owns the frame and its
behaviour; the app (or webapp-lib) brings the content. Before webapp-lib
piece 2, whose auth-guarded base component would wrap `AppShell`.

## Why

Today every app copies two things:

- **The shell** (`docs/recipes/shell.rs`, 30 lines): an `AppBar` whose
  `start` is a `SideMenuButton`, a `SideMenu`, the `RwSignal<bool>` tying
  them, then the router's `Outlet`. Nothing gives the page a skip link,
  and a `PageGrid` inside a shell has to be told `landmark=false` by hand
  (or there's no `<main>`, or two).
- **The page:** `PageGrid` and a first `Panel` with
  `heading_level=HeadingLevel::H1` (every recipe and the starter). Nothing
  sets `document.title`, and after a route change focus stays on the link
  that was followed (inside the closed side menu), so a screen reader
  hears nothing of the new page.

What's copied is behaviour, which drifts; what differs between apps is
content. For an agent, a component is fewer tokens than a recipe, and the
drift check can require it.

## What stays out

The store, sign-in state and the user menu's items, which pages exist,
routes and guards: slots and children. The user's conventions (an
auth-guarded base component, the user menu with sign-out) are a
webapp-lib adapter over `AppShell` ([[general-not-stack]]).

## A. `AppShell` (group `navigation`, which gains `layout`)

```rust
#[component]
pub fn AppShell(
    /// The app's name: the bar, the side menu's title, `document.title`'s end.
    #[prop(into)] title: Signal<String>,
    /// Where the name links (usually "/").
    #[prop(optional, into)] href: MaybeProp<String>,
    /// The side menu's links (`NavLink`, `NavGroup`).
    nav: ChildrenFn,
    /// The bar's end: the app's actions, a user `Menu`.
    #[prop(optional, into)] bar_end: Option<ViewFnOnce>,
    /// Pinned to the bottom of the side menu.
    #[prop(optional, into)] menu_footer: Option<ViewFn>,
    #[prop(into, default = "Skip to content".into())] skip_label: Signal<String>,
    #[prop(into, default = "Menu".into())] menu_label: Signal<String>,
    #[prop(into, default = "Sections".into())] nav_label: Signal<String>,
    /// The page: usually the router's `<Outlet/>`.
    children: Children,
) -> impl IntoView
```

It renders, in order: a skip link (`.app-shell__skip`, hidden until
focused, to `#<main id>`), the `AppBar` (title, href, `SideMenuButton` at
its start, `bar_end`), the `SideMenu` (title, nav, footer; it already
closes itself on navigation), and `<main class="app-shell__main"
tabindex="-1">` with the children. The menu's open signal is its own.

It provides a context: the app's title, and that a `<main>` exists. So:
- **`PageGrid` drops its own `<main>` inside `AppShell`** (its `landmark`
  becomes "automatic" by default: `<main>` only outside a shell). An
  explicit `landmark` still wins.
- `Page` (B) reads the app's title for `document.title`.

It must be inside a `<Router>` (the side menu is). The side menu slides in
on every width for now; a menu kept open beside the content on a wide
screen is a later option, not this plan.

**Tests (e2e):** the skip link is the first Tab stop, appears on focus, and
moves focus to `<main>`; the menu button opens the side menu and a link
closes it; one `<main>` with a `PageGrid` inside; `bar_end` at the bar's
end; the menu's footer. Demo: the recipes' Cookbook shell becomes
`AppShell` (the demo's own frame, with its theme bar, stays as it is).

## B. `Page` (group `layout`)

```rust
#[component]
pub fn Page(
    /// The page's title: its H1, and `document.title` ("{title} · {app}").
    #[prop(into)] title: Signal<String>,
    #[prop(optional)] width: PageWidth,
    /// Buttons beside the title.
    #[prop(optional, into)] actions: Option<ViewFnOnce>,
    /// A side column (under the main one on a phone).
    #[prop(optional, into)] side: Option<ViewFnOnce>,
    #[prop(optional, into)] help: MaybeHelp,
    /// More panels under the page's own (a settings page's sections).
    #[prop(optional, into)] more: Option<ViewFnOnce>,
    /// The page's own panel's body.
    children: Children,
) -> impl IntoView
```

It is `PageGrid` (width, side) holding the page's own `Panel` (H1, title,
actions, help, the children), then `more`. No styling of its own.

- **`document.title`** follows `title`: "{title} · {app}" inside
  `AppShell`, "{title}" outside. Reactive (a detail page's title arrives
  with its data).
- **Focus after navigation:** when a `Page` appears after the first one in
  its `AppShell` (a route change, not the first load), focus moves to its
  H1 (`tabindex="-1"`, no scroll), so a screen reader announces the new
  page and Tab continues from it. On the first load, focus stays where the
  browser puts it.
- One `Page` per route; a second inside the first is a mistake (debug
  warning).

**Tests (e2e):** the H1 and the actions; `document.title` with and without
a shell, and following a reactive title; after following a side-menu
link, focus is on the new page's H1; on first load it isn't; `more` after
the page's panel; `side`. Demo: the recipes' list, detail and settings
pages use `Page`.

## C. The kit and the docs

- `docs/app-shell.md`, `docs/page.md` (compiled examples), `docs/index.md`
  regenerated, `docs/choosing.md` rows ("the app's frame: `AppShell`", "a
  page: `Page`"), `docs/recipes.md` and the recipes rewritten on them
  (the shell recipe shrinks to the store and an `AppShell`), the starter's
  page as a `Page` (inside an `AppShell` with one link, so a new app starts
  with the frame).
- Drift check rules (`agent-kit/check/rules.tsv`): `page-h1` forbids
  `HeadingLevel::H1` in app code (instead: `Page`); `hand-shell` forbids
  `SideMenuButton` in app code (instead: `AppShell`). The demo's own
  showcase pages are exempt as before.
- `CHANGELOG.md`, the README's group table.
- Measure again (the piece-10 prompt, a fresh folder), to see the recipe
  copy shrink.

## Pieces

Each passes `scripts/release-check.sh` and is reviewed on its own.

1. **`AppShell`**, the context, `PageGrid`'s automatic landmark, the skip
   link CSS; the Cookbook shell on it; e2e; its docs page.
   *Done 2026-10-25:* `nav` is a `ViewFn` (a plain closure). Inside the
   shell, `PageGrid` drops only its `<main>`; its side stays an `<aside>`
   (valid inside `<main>`, and the detail recipe's tests rely on it). The
   skip link moves focus itself: leptos_router takes over a `#` link's
   click, so the browser never focuses the target. `docs/navigation.md`'s
   example, which hand-made an `app-bar` header, is now an `AppShell`.
2. **`Page`**, `document.title`, focus after navigation; the recipes'
   pages on it; e2e; its docs page.
   *Done 2026-10-25:* "after a navigation" is a new path since the last
   page took focus (the shell passes the router's path), so a page that
   re-renders on its path (a spinner, then the record) doesn't take
   focus twice, and the first load never does. The `Modal` gave focus back
   to its opener on `close` unconditionally, which raced the new page's
   title after a side-menu link (Firefox); it now does so only if focus is
   on `<body>` or still in the dialog. A loading state with no title yet
   stays a bare `PageGrid` (the detail recipe's spinner).
3. **The kit:** the two drift rules, `choosing.md`, `recipes.md`, the
   starter, CHANGELOG; then one measured run.
   *Done 2026-10-25:* `page-h1` forbids `HeadingLevel::H1` and a raw
   `<h1>`; `hand-shell` forbids `SideMenuButton`. The starter: `App` is a
   `Router` around an `AppShell` (one `NavLink`) and `Routes` (a `Page`
   "Not found" fallback); its screen is `ItemsPage`, a `Page`. Its name
   is `APP_NAME` in `src/app.rs` (`setup.md` says to rename it). Its pins
   moved to `ce70859`, the first commit with `AppShell` and `Page`: a
   copied starter resolves once that commit is pushed.
   *Measured* (the piece-10 prompt, Sonnet, fresh folder): 62,641 tokens,
   10 tool calls, 1 min 35 s, one clean build, no drift; it read no
   component page, and its app is an `AppShell` (from the starter) with
   the list recipe as a `Page`. Against piece 10's last run (60,793
   tokens, 11 calls, 1 min 59 s) the cost is level; what it gains is the
   frame for free. Left to guess: how the starter and a copied recipe
   combine (replace `ItemsPage` and `Item`, the route renders the
   recipe's page, `Store::provide()` in `App`), and deleting the git lines
   leaves the starter's `# data, menus.` comment orphaned.

## Open points (all approved as recommended, 2026-10-25)

1. **`Page` renders the page's own `Panel`** (H1, actions, help), with
   `more` for further panels, recommended: it's what every recipe does,
   and one H1 per page follows. The alternative, `Page` as a bare
   `PageGrid` that only sets the title and focus, leaves the H1 panel to
   copy.
2. **`Page` sets `document.title` and moves focus after navigation**,
   recommended: both are forgotten in every app and both are invisible
   until a screen-reader user or a browser tab needs them. The
   alternative leaves them to the app.
3. **Slide-in menu on every width for now**, recommended; a menu kept open
   beside the content on wide screens later, if an app asks.
4. **The starter gains `AppShell`** (one link), recommended, so a new app
   starts with the frame. The alternative keeps the starter to one page.
5. **The two drift rules**, recommended: they make the components the only
   way, which is the point for an agent.
