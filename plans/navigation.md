# navigation: NavList, NavLink, NavGroup, SideMenu, SideMenuButton

The `navigation` step of piece 6 in `plans/ferrox-ui.md`. It replaces
webapp-lib's `SideMenu` / `MenuLink` with a link list that works anywhere and
a side menu built on `Sheet`. It ports the link styling of conrad's
`.shell__nav-item` (`ui/src/app/shell.css`) and webapp-lib's `.side-menu a`.

Status: done (2026-10-23), all open points as recommended. All four pieces
done; piece 4 is the navigation section of `ferrox-ui/README.md`.

Notes from piece 1:
- leptos_router 0.8.17's `<A>` already treats `/` as current only on `/`
  (its prefix match skips the root segment), so `exact` is for prefix routes:
  `/accounts` against `/accounts/42`. `NavLink` passes its class with
  `attr:class`.
- A theme's `--primary-fill` may be a gradient, so the current link's tint is
  in `background-image`; the e2e test compares colour and image together.
- The release check: 8 feature combinations for ferrox-ui, 314 e2e tests
  (the smoke test now loads both navigation routes).
  Two Firefox layout tests (buttons "does not move its neighbour", forms
  "the toggle sits inside the field") failed once under the full parallel
  run, then passed 3/3 alone and in two full runs: a load-dependent flake,
  as the fonts one in themes.spec.ts.

Notes from piece 2:
- The group's heading sits in its own `<li>` with the nested `<ul
  aria-labelledby>`; the chevron sits at the row's right end and turns 90°
  (not under reduced motion).
- `.nav-group__list` shares the list rule with `.nav-list__items`, so it
  needs `.nav-group__list[hidden] { display: none }`: a class's `display`
  beats the browser's own `[hidden]`.
- Expanding on arrival: an `Effect` on the router's `pathname`, then a frame
  later (once `<A>` has moved `aria-current`) a DOM check of the group's own
  list. On arrival only, so the user can still collapse the group around the
  current page.
- e2e trap: opening a second page in a test (beforeEach, then `openApp`
  again) aborts the first page's font loads, which Firefox logs as a console
  error. A test that starts on another route gets its own describe.
- Release check: 334 e2e tests.

Notes from piece 3:
- `SideMenu` closes on a link click in a microtask, after the router has
  handled the same click (the router finds the link through the event's
  composed path), and only for the clicks the router handles: a plain left
  click on an `<a href>` without `target`. A ctrl-click (new tab) leaves the
  menu open. Route changes close it through `Effect::watch` on `pathname`
  (not on the first run).
- `children` is `ChildrenFn` (the sheet rebuilds its body on each opening),
  kept in a `StoredValue`. `footer` is `Option<ViewFn>` with `into`, so a
  plain closure converts (`Option<ChildrenFn>` doesn't).
- `SideMenuButton` passes `aria-haspopup` / `aria-expanded` to its
  `IconButton` with `attr:`.
- The demo's own header now has the menu, outside the routes, as an app
  would: a menu inside a page would unmount on every navigation and test
  nothing.
- Fixed a piece-2 bug the new tests exposed: `NavGroup`'s frame-later check
  ran after a route change had unmounted the group and panicked on a disposed
  node ref (`get_untracked`). It now uses `try_get_untracked` / `try_set`.
- Release check: 358 e2e tests.

## Sources

| Today | Where | Becomes |
|---|---|---|
| `SideMenu { title, user_name, is_authenticated, children }`. It renders three things: a top bar (a hamburger, the title with an inline style, a user badge), its own scrim, and a hand-made drawer (`transform`, no focus trap, no Esc, no `aria-modal`) | webapp-lib `components/mod.rs:426`, `style/base.css:125-190, 613` | `SideMenu` (the drawer, on `Sheet`) + `SideMenuButton` (the hamburger). The top bar and the user badge stay in webapp-lib (below) |
| `MenuLink { href: &'static str }`: `<A>` that closes the drawer, through context of a bare `RwSignal<bool>` (H11), panicking outside a `SideMenu` | `mod.rs:477` | `NavLink`. It no longer closes anything; the menu does that (below) |
| nothing: webapp-lib's menu is one flat list (in the shopping-list app, its own pages, the product database's and auth's mixed together) | | `NavGroup`: headed, optionally collapsible groups |
| conrad's sidebar items: an icon, then the name; `aria-current` tinted; hover in colour only | `ui/src/app/AppShell.tsx` `SidebarItem`, `shell.css:91-135` | `NavLink`'s look and its `icon` |
| The hamburger is a `<button>` with the glyph class `trigram-for-heaven` (X7); the close is `icon-btn multiply` | `mod.rs:442, 464` | `SideMenuButton` draws ferrox-icons `MENU`; the close is `Sheet`'s × |

Review items covered: H11 (no bare-typed context), X7 (no glyph icons), S2
(no inline style), and the drawer gains everything `Sheet` already does
(focus trap, Esc, focus returns to the button, the page doesn't scroll).

## Design

### NavLink and NavList: links that say where you are

```rust
#[component]
pub fn NavLink(
    href: String,                               // into
    /// Mark the link current only on this exact path, not below it. For "/".
    #[prop(optional)] exact: bool,
    /// An icon before the words (ferrox-icons `IconData`).
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView

#[component]
pub fn NavList(
    /// The landmark's name: "Sections", "Settings".
    #[prop(into)] label: Signal<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,                         // NavLinks
) -> impl IntoView
```

```rust
<NavList label="Sections">
    <NavLink href="/" exact=true icon=HOME>"Home"</NavLink>
    <NavLink href="/accounts">"Accounts"</NavLink>
</NavList>
```

- **Markup:** `<nav class="nav-list" aria-label><ul>` with one
  `<li><a class="nav-link">` per link, so a screen reader hears "list, 6
  items".
- **The current page** comes from leptos_router's `<A>`, which sets
  `aria-current="page"`. `/` matches every path unless it's `exact`; piece 1
  checks how `<A>` decides this in 0.8 and documents it.
- **Look** (from conrad): a full-width row, icon then words, hover and press
  in colour only (a growing row shoves its neighbours), the current page
  tinted in the primary colour, and the theme's focus ring.
- Links only. No `help` on a link: a `?` in a list of places is noise. The
  menu or the page around it carries help.

### NavGroup: links that belong together

New (webapp-lib's menu is one flat list). A menu that mixes an app's own
places with those of the services it builds on reads better grouped. In the
shopping-list app, Lists and Load Data are the app's own. Units, Categories,
Products, Tag Dimensions and Tags belong to the product database. Account,
Invitations and Logout belong to auth.

```rust
#[component]
pub fn NavGroup(
    /// The group's heading, and the name of its list.
    #[prop(into)] label: Signal<String>,
    /// The heading is a button that shows and hides the links.
    #[prop(optional)] collapsible: bool,
    /// Where a collapsible group starts: collapsed unless this is true.
    #[prop(optional)] start_expanded: bool,
    /// Two-way, for an app that remembers the state itself (overrides
    /// `start_expanded`).
    #[prop(optional)] expanded: Option<RwSignal<bool>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,                         // NavLinks
) -> impl IntoView
```

```rust
<NavList label="Sections">
    <NavLink href="/" exact=true>"Lists"</NavLink>
    <NavLink href="/load">"Load Data"</NavLink>
    <NavGroup label="Product database" collapsible=true>
        <NavLink href="/units">"Units"</NavLink>
        <NavLink href="/categories">"Categories"</NavLink>
        <NavLink href="/products">"Products"</NavLink>
        <NavLink href="/tag-dimensions">"Tag Dimensions"</NavLink>
        <NavLink href="/tags">"Tags"</NavLink>
    </NavGroup>
    <NavGroup label="Account">
        <NavLink href="/account">"Account"</NavLink>
        <NavLink href="/invitations">"Invitations"</NavLink>
        <NavLink href="/logout">"Logout"</NavLink>
    </NavGroup>
</NavList>
```

- **Markup:** the group is one `<li class="nav-group">` in the list, holding
  its heading and a nested `<ul aria-labelledby=heading>`, so a screen reader
  hears "Product database, list, 5 items". One level only: no group inside a
  group.
- **Plain group:** the heading is quiet text (smaller, muted) and a hairline
  splits the group from what comes before. It isn't a link and isn't in the
  Tab order.
- **Collapsible group:** the heading is a `<button aria-expanded
  aria-controls>` with a chevron (`CHEVRON_RIGHT`, turned down while
  expanded) and is in the Tab order. Collapsed, the nested list is `hidden`.
  The disclosure pattern, not `<details>`, so the state is a signal like
  every other control here.
- **The current page is never hidden.** A collapsed group that holds the
  current page expands itself when the route moves into it. Otherwise the
  user lands on Units and the menu hides where they are.
- **State across openings:** a `SideMenu`'s body is mounted only while it's
  open (it's a `Sheet`), so a group's own state starts over each time. With
  the rule above, that is: collapsed, unless the user is inside it. An app
  that wants the menu to remember passes `expanded` (open point 4).

### SideMenu: navigation that slides in

```rust
#[component]
pub fn SideMenu(
    /// Two-way: `SideMenuButton` opens it; it closes itself (see below).
    open: RwSignal<bool>,
    /// Shown in the title bar; the sheet's accessible name.
    #[prop(into)] title: Signal<String>,
    /// The `<nav>`'s name, inside the sheet.
    #[prop(into, default = "Sections".into())] nav_label: Signal<String>,
    #[prop(optional)] side: SheetSide,         // Left
    /// Pinned to the bottom of the menu: an app version, a legal line.
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(optional, into)] help: MaybeHelp,
    #[prop(into, default = "Close".to_string())] close_label: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,                         // NavLinks
) -> impl IntoView

#[component]
pub fn SideMenuButton(
    open: RwSignal<bool>,
    #[prop(into, default = "Menu".to_string())] label: String,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

```rust
let menu = RwSignal::new(false);

view! {
    <header class="app-bar">
        <SideMenuButton open=menu />
        <span class="app-bar__title">"Groceries"</span>
    </header>
    <SideMenu open=menu title="Groceries">
        <NavLink href="/" exact=true>"Lists"</NavLink>
        <NavLink href="/load">"Load Data"</NavLink>
        <NavGroup label="Product database" collapsible=true> … </NavGroup>
        <NavGroup label="Account"> … </NavGroup>
    </SideMenu>
}
```

- **It's a `Sheet`** (`.sheet` plus `.side-menu`), so it inherits the modal
  behaviour, the slide, the × and the backdrop click. Its children are
  wrapped in a `NavList`, and `footer` sits below the list, pinned to the
  bottom.
- **It closes itself** when the user follows a link: on a click of any
  `<a href>` inside it (so a plain `<A>` works too, not only `NavLink`), and
  on any route change while it's open (the back button). That's why `open`
  is an `RwSignal` here, where `Sheet` takes `open` + `on_close` (open
  point 1). No context is needed, so H11 goes away rather than getting a
  newtype.
- **It needs a `<Router>`** around it, for the route-change close.
- **Focus** returns to `SideMenuButton` on close, as for any `Sheet`. After a
  navigation that's the button, not the new page's heading. Moving focus to
  the page is the app's routing concern, not the menu's.
- **`SideMenuButton`** is an `IconButton` drawing `MENU`, labelled `label`,
  with `aria-haspopup="dialog"` and `aria-expanded` following `open`.

### What stays in webapp-lib

The top bar (the hamburger, the app title, the user badge) and
`is_authenticated` are app-shell and session concerns. webapp-lib's piece 3
rebuilds its `SideMenu` as an adapter: its own bar with `SideMenuButton`, and
ferrox-ui's `SideMenu` (the user's name in `footer` only until its app-shell
story finds it a better place). A shared page frame
(header + body) is `layout`'s **Page frame**, later.

### The `navigation` feature and CSS

- `navigation = ["overlay", "dep:leptos_router"]`: `SideMenu` needs `Sheet`,
  and only this group pulls in leptos_router (as the master plan says).
- The powerset release check grows from 4 to 8 combinations.
- `navigation/nav.css`: `.nav-list`, `.nav-link`, `.nav-link__icon`,
  `.nav-link[aria-current='page']`, `.nav-group`, `.nav-group__heading`,
  `.nav-group__toggle`, `.nav-group__list`. `navigation/side-menu.css`:
  `.side-menu`, `.side-menu__footer`. Class-only and tokens only, as every
  sheet.
- No new icons: `MENU`, `CHEVRON_RIGHT` exist. The demo uses icons that exist (or adds
  `HOME` from Lucide if an example needs it).

## Pieces

Each one passes `scripts/release-check.sh` and can be reviewed on its own.
The demo gets a `/navigation` page.

1. **Feature, NavList, NavLink.** Feature and leptos_router dependency,
   `nav.css`, the 8-way powerset. The demo page shows a `NavList` of the
   demo's own pages. e2e:
   - a list of links inside a labelled `nav` landmark;
   - the current page has `aria-current="page"` and the tint; the others
     don't;
   - `exact` keeps `/` from matching every page;
   - the icon is drawn before the words and hidden from screen readers;
   - keyboard: Tab reaches each link, with the focus ring;
   - hover doesn't change the row's size.
2. **NavGroup.** e2e:
   - a plain group: its heading names its nested list, a hairline before
     it, the heading isn't focusable;
   - a collapsible group: a button with `aria-expanded` / `aria-controls`,
     collapsed by default, Enter and Space toggle it, the chevron turns;
   - `start_expanded`, and a two-way `expanded`;
   - a collapsed group expands itself when the route moves into it;
   - Tab skips the links of a collapsed group.
3. **SideMenu, SideMenuButton.** `side-menu.css`. e2e:
   - the button's name, `aria-haspopup`, and `aria-expanded` on open and
     close;
   - it opens as a modal sheet named by `title`, with a labelled `nav`
     inside;
   - following a `NavLink` navigates and closes it; so does a plain `<A>`;
   - clicking the current page's link still closes it;
   - the back button while open closes it;
   - Esc, the ×, and the backdrop close it, and focus returns to the
     button;
   - the footer sits at the bottom of the sheet;
   - `help` puts a `?` in the title bar;
   - groups inside it: a collapsed group starts collapsed on each opening,
     unless the current page is in it.
4. **Docs.** A navigation section in `ferrox-ui/README.md` (each component
   with an example, the class reference, the Router requirement), the root
   README, the demo README, and this plan's status.

Not here:
- A persistent desktop sidebar that folds to a rail of icons (conrad's
  `shell__sidebar--rail`; open point 2).
- Count badges on links (conrad's `shell__nav-count`), waiting for `core`'s
  Badge.
- Groups inside groups, disabled "planned" links, Breadcrumb, Pagination,
  Bottom nav, Stepper (inventory, built when an app needs them).

## Open points

1. **`open` is an `RwSignal`**, not `Sheet`'s `open` + `on_close`, because
   the menu closes itself on navigation. Recommended. The alternative keeps
   `Sheet`'s shape and makes every app write
   `on_close=move || menu.set(false)`.
2. **The persistent sidebar (with the rail)** comes later, as its own story.
   Recommended: no app needs it now, and `NavLink` is built to sit in one
   (icon, then words). The alternative is a `Sidebar` in this group now.
3. **Names:** `NavLink` and `NavList` (`.nav-link`, `.nav-list`), with no
   `MenuLink`, so that "menu" stays the `menus` group's dropdowns.
   webapp-lib's adapter keeps `MenuLink` as an alias while its apps move.
4. **A group's expanded state isn't kept** between openings of a `SideMenu`:
   it starts collapsed (or `start_expanded`), and opens itself around the
   current page. Recommended: it's predictable, and an app that wants memory
   passes `expanded` (and may store it, as conrad's sidebar stores its
   rail). The alternative is ferrox-ui remembering it in the browser's
   storage, keyed by the group's label.

Also raised, not here: the user's name doesn't belong in the menu (today's
footer badge). Where who's signed in shows is webapp-lib's app-shell story;
`footer` stays for anything an app wants pinned there.
