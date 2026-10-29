# Changelog

All crates in this repo share one version and are released together under
one git tag.

## 0.1.1 (2026-10-28)

### Added: the `media` group and its `Carousel`

For garcon-dashboard's launcher, whose ninth tile didn't fit a phone
(`plans/carousel.md`). Only adds: nothing else changes.

- `Carousel` (feature `media`): items in pages of `per_page`, each a list
  laid out by the app's `page_class`. The track scrolls sideways and snaps
  a page at a time, so the swipe is the browser's; Previous, Next and a dot
  per page scroll to a page, and the page follows the scroll position. An
  optional two-way `page`; no controls for one page; labels in
  `CarouselLabels`. Helpers `carousel_pages`, `page_at`, `carousel_classes`,
  `track_classes`.
- `phone_only`: pages on a phone (≤ 768px) only; above it every item flows
  into one `page_class` grid and the controls hide.
- Docs `carousel.md`, `media.md` and a choosing row (so the agent kit's
  `AGENTS.md` too); demo `/media`; e2e in Chromium and Firefox.
- The starter pins `tag = "v0.1.1"`; the agent kit is `0.1.1`.

## 0.1.0 (2026-10-25)

The first release.

### One copy per build

- `ferrox-theme`, `ferrox-icons` and `ferrox-theme-build` declare `links`,
  so a build that would hold two ferrox versions fails to resolve instead
  (see the README's "Versions"). `ferrox-theme-build` gains an empty build
  script for it. The starter pins `tag = "v0.1.0"`.

### Changed (breaking): one name, one type per idea

From the names audit in `plans/agent-first.md` (piece 3).

- Every human-readable word is a `Signal<String>` (or `MaybeProp<String>`
  when optional). Literals still work. Changed from `String`: `Alert`'s,
  `Dialog`'s, `ConfirmDialog`'s, `Sheet`'s and `SideMenu`'s `close_label`;
  `Combobox` `clear_label`, `toggle_label`, `refresh_label`; `Select`
  `refresh_label`, `create_label`; `PasswordInput` `show_label`;
  `SideMenuButton` `label`; `SplitButton` `text`, `menu_label`;
  `DataView` `retry_label`; `MenuTrigger`'s texts and labels. `Button`
  `title` is a `MaybeProp<String>`.
- `IconButton` `title` → `label` (it's the button's name; `title` is a
  heading everywhere else).
- `LoadingOverlay` `text` → `label`.
- `Alert` `on_dismiss` → `on_close`, `dismiss_label` → `close_label`
  (default `"Close"`), class `.alert__dismiss` → `.alert__close`.
- `Dialog` `dismiss_label` → `cancel_label`, `no_dismiss` → `no_cancel`.
- `DataView` `empty` → `empty_label`.
- `ContextMenu` `items` → `menu`.
- No callback passes a DOM event: `Button`, `IconButton` and `SplitButton`
  `on_click` are `UnsyncCallback<()>` (as `Card`'s already was). Write
  every callback as `move || …`.
- `Dialog`, `Sheet` and `ConfirmDialog` own their state, as `SideMenu`
  did: `open` is an `RwSignal<bool>`, and every dismissive way out (Esc,
  the ×, Cancel, a sheet's backdrop) sets it false itself. `on_close` /
  `on_cancel` are optional notices; drop `on_close=move || open.set(false)`.
- Content built once takes a run-once closure, so closures inside it may
  move values: `Menu`'s, `SplitButton`'s and `Panel`'s children
  (`Children`), `ContextMenu`'s `menu`, `Card`'s and `Panel`'s `actions`,
  `Menu`'s and `ContextMenu`'s `header` (`ViewFnOnce`).
- `DataView` draws an item again when its value changed, so keying by id
  is enough (a rename used to keep showing the old name). Items need
  `PartialEq`.

### Added

- `layout`: `AppBar` (the app's top band: menu button, name, actions) and
  `Facts` / `Fact` (a record's labels and values).
- `.ferrox-app` styles plain links (an `<a>` with no class) in the theme's
  link colour.
- Recipes: whole screens in `docs/recipes/` (shipped with the crate, compiled
  by the demo), listed in `docs/recipes.md` with what to change when copying
  one alone.
- The agent kit's agent-neutral core (`ferrox-ui/agent-kit/`): `core/AGENTS.md`
  (the entry point, its rules and component table rendered from their
  sources), four task procedures and a reviewer's brief, `kit.tsv`,
  `BOOTSTRAP.md` (any agent), `ADAPTING.md` (writing an adapter), and the
  installer in bash and PowerShell with its spec.
- The kit's adapters: `--claude` (skills, commands, the reviewer as a
  sub-agent, a `CLAUDE.md` block), `--copilot` (prompt files, an
  instructions block), `--agents-md` (an `AGENTS.md` block), generated from
  templates and `kit.tsv`; `--update` lists and applies only what changed.
- The drift check's `page-h1` and `hand-shell` rules: a page's `<h1>` or an
  app frame made by hand (use `Page`, `AppShell`). The starter is an
  `AppShell` with one `Page` (rename `APP_NAME` too) and needs
  `navigation` and leptos_router.
- `Page` (`layout`): `PageGrid` with the page's own H1 `Panel`; sets the
  tab's title ("{page} · {app}") and, after a navigation inside an
  `AppShell`, moves focus to its title. A modal now gives focus back to its
  opener only if nothing else has taken it.
- `AppShell` (`navigation`, which now turns on `layout`): the app's frame,
  with a skip link, the bar and its menu button, the side menu, and one
  `<main>`. `PageGrid`'s `landmark` is automatic: no `<main>` of its own
  inside an `AppShell`.
- The installed kit carries the starter (`.ferrox/starter`), so `setup`
  starts a new app before it has any docs to find.
- `HelpReveal`: `help_reveal` on Field, Checkbox and Switch, and `reveal` on
  `HelpScope` for the controls inside, show a `?` only while its control is
  in use. `HelpScope`'s `context` is now optional.
- `HelpKeys`: F1 opens the focused control's help, Shift+F1 asks the
  assistant about it. Components with help mark themselves `data-help`;
  `HelpButton` takes an `id`.
- `HelpSignals`, `HelpSignal`, `HelpSignalKind`, `help_signals()` /
  `HelpReporter`: facts that may mean a user needs help (a `?` opened, or
  dismissed at once; the app's own reports), for the app's policy. Field
  reports its error shown, cleared and persisting; Dialog a refused
  submit; Combobox text left with no match. `Button` gains `on_refused`.
  `HelpSuggestions` (passed to `HelpSignals`; also `HelpReporter::suggest`
  / `clear`) marks a topic's `?`s as suggested.
- The drift check (`ferrox-ui/agent-kit/check/`): `ferrox-check` lists
  where an app drifts from ferrox-ui, with the fix; `ferrox-docs` prints
  the docs of the ferrox-ui an app builds against. Each in bash and
  PowerShell 7, with a spec (`*.spec.md`) to port it anywhere else; the
  rules are data (`rules.tsv`).
- `layout`: `PageGrid` (the page: a width cap, padding, `<main>` and an
  optional side column that moves under it on a phone), `Stack` and
  `Cluster` (a column, or a wrapping row, with a token gap), and `Gap`,
  `Align`, `Justify`, `PageWidth`, `SidePosition`. `.ferrox-app` resets
  `<body>`'s margin.

- `class` on `MenuItem`, `MenuLink` and `LoadingOverlay`;
  `menu_item_classes` takes the extra classes.
- `.ferrox-app` (in the theme stylesheet): the page's ground on `<body>`.
- `ferrox_theme_build::PREPAINT_SCRIPT` and `PREPAINT_FILE`: `write_css`
  writes `prepaint.js`, which `index.html` inlines.
- `starter/`: an app to copy.

### Fixed

- On a phone, scrolling a sheet (a `SideMenu`) or a dialog to its end
  could scroll the page behind it. The scroll lock is on `<html>` too, the
  modal's body contains its overscroll, and a side sheet is `100dvh` tall.

### Removed

- `ferrox_theme::PREPAINT_SCRIPT`: the script is written by
  `ferrox-theme-build` and inlined, not pasted.
