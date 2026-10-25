# ferrox-ui: theme, primitives and icons for Leptos CSR apps

A public UI kit, split out of webapp-lib (whose `v1.0` branch is where the
theme and the first primitives were built). Modelled on rust-ui.com's
*behaviour and inventory*, not its construction: a normal crate dependency on
token-based, class-only CSS. No copy-in CLI, no Tailwind.

webapp-lib keeps the app plumbing (http, storage, reactive, auth, app-level
components such as `Login`) and depends on ferrox-ui; see webapp-lib's
`plans/crate-split.md` for that side.

Status: pieces 0–5 done; piece 6 under way (core, overlay, help (plan `plans/help.md`), forms (plan `plans/forms.md`) navigation (plan `plans/navigation.md`) and layout (plan `plans/layout.md`) done; `plans/generalise.md` done (the user's stack out of the ported components; Combobox/Select `on_create`); data (plan `plans/data.md`) done 2026-10-24; menus (plan `plans/menus.md`) done 2026-10-24, with ContextMenu; `plans/agent-first.md` (the user's direction 2026-10-24: the audience is an agent) done 2026-10-25; the help follow-ups (`plans/help-followups.md`: HelpReveal, HelpKeys, HelpSignals) done 2026-10-25; the Sheet scrolling fix done 2026-10-25; `plans/app-shell-page.md` (AppShell, Page) done 2026-10-25; next: webapp-lib piece 2, its base component on `AppShell`). Repo `/programming/ferrox-ui`
(`git@github.com:rbenitez22/ferrox-ui.git`, branch `master`).

## Crates (one workspace, this repo)

```
ferrox-theme-build  (std only, build-dependency)
        │
ferrox-theme        tokens, themes, runtime switching, CSS file generation
        │                       ferrox-icons   SVG icons (leptos only)
        │                          │
        └────────────┬─────────────┘
                 ferrox-ui        primitives, one Cargo feature per group
                     │
          (webapp-lib, other repo: plumbing + app components)
                     │
                    apps
```

| Crate | Contains | Depends on |
|---|---|---|
| `ferrox-theme-build` | theme file discovery + validation, generated CSS file (today's `webapp-lib-build`) | std |
| `ferrox-theme` | `tokens.css`, `themes/*.css`, `register`/`init`/`set_choice`…, `PREPAINT_SCRIPT`. Feature `leptos`: `provide_theme`/`use_theme` | web-sys, wasm-bindgen; leptos (optional) |
| `ferrox-icons` | one Lucide SVG file per icon → one `IconData` const per icon, drawn by one `Icon` component | leptos; roxmltree (build only) |
| `ferrox-ui` | primitives + their class-only CSS, injected per enabled group | ferrox-theme, ferrox-icons, leptos; leptos_router only for `navigation` |
| `ferrox-charts` (later) | charts on the theme's chart tokens | ferrox-theme, leptos (+ chart deps) |
| `demo/` (not published) | showcase of every theme and component; Playwright e2e in `demo/e2e/` | all of the above, by path |

## Decisions (2026-10-23, carried over from webapp-lib's crate-split discussion)

- Names `ferrox-*`. Public repo.
- **Class-only CSS.** No element selectors, no global pseudo-classes; tokens on
  `:root` only. Every style is opt-in through a class.
- **`ferrox-ui` features are broad groups**, `core` always on (inventory below).
- **Theme + loading CSS is a generated file** (review C9): the app's `build.rs`
  calls `ferrox-theme-build`, which writes one combined CSS file into the app;
  `index.html` links it with `data-trunk`. The page is styled before the WASM
  loads, so pre-paint and the `#loading` screen work. Component CSS is
  injected by `ferrox_ui::init()` for the enabled groups.
- **Lockstep inside this repo**: one version, one git tag (`vX.Y.Z`) for all
  crates. Starts at `0.1.0`.
- Apps normally reach ferrox-ui through webapp-lib's re-exports (facade); an
  app without webapp-lib uses ferrox-ui directly. **An app using webapp-lib
  never lists ferrox-ui or ferrox-theme itself**: two references that differ
  (tag, or ssh vs https URL) make two copies of every type. The build script's
  `ferrox-theme-build` is the one direct dependency; `check_build_version`
  catches a mismatched tag, and `embed_themes_for(dir, "::webapp_lib::theme")`
  points the generated code at the re-export.
- Charts: `ferrox-charts`, later.
- Icons are SVG (UTF-8 glyph icons were abandoned in webapp-lib).
- **ferrox-ui is the source of truth** for its tokens, themes and component
  CSS. They started as copies of conrad's (TIER 1 files); conrad, a React app,
  goes its own way from here. No sync script, no "edit it in conrad" headers.
- Licence `MIT OR Apache-2.0`. Fresh first commit; webapp-lib keeps the
  history of how the theme and first primitives were built.
- Release policy as in webapp-lib: develop and harden here, test, then tag.
  The demo's release check (every feature alone, clippy, tests, Trunk build,
  Playwright in Chromium + Firefox) gates every tag. Apps use tags only and
  extend, never modify.

### Inventory (draft for review)

Sources: **W** = webapp-lib today, **C** = conrad has it (React; behaviour and
CSS to port), **R** = in rust-ui's catalogue. **Now** = needed by today's
apps or already exists, so it moves/ports during the split; the rest are new
components, one story each, when an app needs them.

| Group | Component | Source | Now? | Notes |
|---|---|---|---|---|
| **core** (always on) | Button, IconButton | W C R | now | done on `v1.0`. In core, not forms, because dialogs, toolbars, alerts and pagination all use them (proposed; user leaned to forms 2026-10-23) |
| | Toolbar | W R | now | done on `v1.0` |
| | Spinner | W C R | now | |
| | Alert (today `Message`) | W C R | now | |
| | Skeleton | C R | | conrad has `Skeleton`/`SkeletonSwap` |
| | Badge, Separator, Kbd, Empty, Status | R | | Card moved to `layout` (2026-10-23) |
| | Icon slot | C | now | renders a `ferrox-icons` icon |
| | Theme toggle | R | | needs `ferrox-theme`'s leptos context |
| **forms** | Field (label + help + error) | C R | now | conrad `FormField`; fixes id clashes (H12) |
| | Input (text, password + strength) | W R | now | today `VerifiedPassword`/`EntropyIndicator` |
| | NumberInput | R | | conrad has `AmountKeypad` for amounts |
| | DateInput / Date Picker | C R | | conrad `PeriodPicker`/`PeriodChooser` |
| | Textarea | R | | |
| | Checkbox | W R | now | today `CheckBox` |
| | Switch / Toggle | C R | | conrad `Toggle` |
| | Radio, Radio group | R | | |
| | Select | W R | now | today `render_options`; data-bound `SelectMenu` stays in lib as an adapter |
| | MultiSelect | R | | |
| | Combobox / Autocomplete | W C R | now | today `AutocompleteInput` (UI half) |
| | Slider, Input OTP, Input Phone, Chips, Dropzone | R | | |
| **menus** | Dropdown menu (popup menu) | C R | done | conrad `Menu`. Scheduled 2026-10-23: row actions, a user menu, an overflow "More" all need one |
| | Split button | | done | main action + chevron opening related actions (e.g. "Edit ▾"); a menu with a button trigger |
| | Menu button | | done | one button with a chevron; looks like a select, each option is an action |
| | Context menu | R | done | added to `plans/menus.md` as piece 4 (2026-10-24): `Menu`'s items opened by right-click, Shift+F10 / Menu key, long press |
| | Menubar, Command palette | R | | |
| **overlay** | Dialog | W C R | now | conrad header / body / footer (footer band padding, user request 2026-10-23) |
| | Alert dialog (confirm) | W C R | now | today `ConfirmDialog`, `DeleteConfirmDialog` (C6) |
| | Sheet / Drawer | W R | now | today `SideMenu` is a left sheet |
| | Popover, Tooltip, Hover card | R | | |
| | Toast | C R | | conrad `Toasts` |
| **navigation** | Side menu + menu link | W | now | today `SideMenu`/`MenuLink` (needs leptos_router) |
| | Breadcrumb, Bottom nav, Stepper / Wizard | C R | | conrad `Wizard`. Pagination moved to `data` (2026-10-23) |
| **layout** | Panel (title + control box + body) | W C | now | today `ComponentTitleBar` (as a panel header: `Toolbar` title + buttons); collapsible variant = conrad `CollapsiblePanel` |
| | Card | R | done | moved from `core` (2026-10-23): a surface for one thing, optionally a link or a button as a whole; the record template for `DataView` |
| | Tabs | C R | | conrad `Tabs`; when an app needs it |
| | Accordion / Collapsible | C R | | a stack of collapsible `Panel`s, optionally one open at a time; when an app needs it |
| | Card columns | C | | conrad `CardColumns` |
| | Page frame, Scroll area, Aspect ratio | C R | | conrad `PageFrame` |
| **data** | Table | R | | |
| | DataView (was "Data grid") | W R | done | a record list rendered through one item template (usually a `Card`), laid out as a grid of cards, a list of rows or a column, optionally switchable; loading / empty / error states; keyed rows (Q10). Lineage: PrimeFaces' `DataGrid` / `DataView` → webapp-lib `DataGrid` + `ListComponentView` → rusty-skillet's `DataListView` (empty message, loading, infinite scroll), the user's attempt at improving it. Named `DataView`, since "data grid" usually means a spreadsheet-like table |
| | Pagination | C R | done | moved from `navigation` (2026-10-23): pages through a `DataView`; no router needed |
| | Data table (sort, filter, page) | R | | |
| **media** | Avatar, Image, Carousel, Marquee, Attachment | R | | probably not needed; listed for completeness |
| **motion** | Animate, Shimmer, Drag and drop | R | | probably not needed |

Charts are a separate crate, `ferrox-charts` (decided 2026-10-23): line, bar,
pie, sparkline, on the theme's chart tokens (`--chart-*`), so `ferrox-ui`
never carries their dependencies.

Not ported: conrad's `ErrorBoundary` (Leptos has its own) and its hooks
(`useDropdownPosition`, …), which become internals of menus/overlay.


## Pieces

Each piece passes this repo's release check once it exists (piece 2), and is
reviewable alone. Code moves from webapp-lib `v1.0` (`54eb481` + later), with
webapp-lib's history as the record (fresh first commit here).

0. **Bootstrap** — done (2026-10-23). Workspace `Cargo.toml` (`[workspace.package]` version
   `0.1.0`, shared deps), `LICENSE-MIT` + `LICENSE-APACHE`, `README.md`,
   `.gitignore`, this plan. First commit, push `master`; check GitHub's
   default branch is `master`.
1. **`ferrox-theme` + `ferrox-theme-build`** — done (2026-10-23). The CSS
   assets (tokens, built-in themes, fonts + OFL licences, `.ferrox-loading`
   screen) live in `ferrox-theme-build/assets/`, compiled into the build crate;
   `write_css(out_dir, themes_dir)` writes `ferrox.css` + `fonts/` (only when
   changed). The runtime no longer injects CSS. Storage key `ferrox.theme`,
   legacy `webapp-lib.theme` read as fallback (runtime and pre-paint).
   Generated code calls `check_build_version` (compile_fail doc test).
   `register` dedupes by id; `init` is idempotent; `set_starting_family` (X5).
   **To verify in piece 2:** Trunk processes `rel="css"` while cargo runs the
   build script, so on a clean first build `ferrox.css` may not exist yet.
   Check; if it bites, run the write from a Trunk `pre_build` hook instead.
   Originally: move `lib/src/theme`,
   `lib/style/{tokens.css,themes/}`, `webapp-lib-build`. Not the conrad sync
   script: ferrox-ui owns the CSS now, so drop the "TIER 1 / edit it in conrad"
   headers. Rename identifiers and the storage key
   (`webapp-lib.theme` → `ferrox.theme`, with a one-time read of the old key so
   users keep their choice). Add the generated CSS file (C9), idempotent
   `init`/`register` (H9), a `ThemeFile` constructor (H8), `#[non_exhaustive]`
   (H7), configurable default family (X5).
2. **Demo + e2e + release check** — done (2026-10-23). Demo on port 3200
   (fresh-build e2e 3299), header + theme bar + themes page, demo-only
   `harbor` theme through the app path. Clean-build finding: Trunk resolves
   `index.html` links before cargo runs, so the generated CSS did not exist on
   a clean build. Fixed with a Trunk `pre_build` hook (`cargo check`) plus
   `write_css` watching its own output (`rerun-if-changed`). New e2e tests:
   styled before mount (theme, `--background`, loading screen at
   DOMContentLoaded), loading screen hides, fonts load from `fonts/`, legacy
   key kept. Release check: per-feature checks, clippy `-D warnings`, tests,
   demo build from scratch, e2e. Originally: Move the demo's theme page, theme bar and
   the Playwright setup (`fixtures.ts`, `themes.spec.ts`, smoke test),
   `scripts/release-check.sh`. The theme tests pass here unchanged; the
   pre-paint test gains a check that the page is styled before mount.
3. **`ferrox-ui` skeleton + `core`** — done (2026-10-23). `Button`,
   `IconButton`, `Toolbar` with reactive `MaybeProp` classes and no `style`
   prop (X3), `#[non_exhaustive]` enums (H7); `init()` injects core CSS
   (idempotent, `set_text_content`). The saving border now keeps 1px, so a
   saving button no longer moves its neighbour (conrad's 2px fixed here; the
   e2e test is a normal test now). A unit test enforces class-only selectors
   on every shipped sheet. e2e: 56 pass. Not yet run: repeated stability runs.
   Originally: Crate, group features, `init()` CSS
   injection. Move `Button`, `IconButton`, `Toolbar` with `focus.css`,
   `buttons.css` (now an owned file, not generated from conrad), `toolbar.css`, the `/buttons` demo page and
   `buttons.spec.ts`. Review items for primitives from here on: `MaybeProp`
   classes and no `style` props (X3), text props (X4), id props (H12), newtype
   contexts (H11).
4. **Feature hygiene** — done (2026-10-23). Release check step 1 runs
   `cargo hack check --each-feature --no-dev-deps` on every library crate;
   cargo-hack is required (the manifest-grep fallback is gone). Switch to
   `--feature-powerset` once ferrox-ui has two or more groups.
5. **`ferrox-icons`** — done (2026-10-23). Own plan: `plans/ferrox-icons.md`.
   14 Lucide icons (ISC) under our own names, checked at build time and
   generated as `IconData` consts. One `Icon` component draws them (icons are
   values, so components can take one as a prop). `.icon` CSS moved to
   `core/icon.css`. Demo `/icons` page + e2e. Release builds keep only the
   icons an app names. ferrox-ui itself depends on ferrox-icons from piece 6,
   when a component first draws an icon. Originally: build-time SVG discovery,
   first icons: close, plus, menu, refresh, check, delete, chevron.
6. **Port today's webapp-lib primitives**, one group per piece, each with its
   demo page and e2e tests: `core` rest (`Spinner`, `Alert`; done 2026-10-23,
   plan `plans/core-spinner-alert.md`, with `LoadingOverlay`, the severity
   icons, `ferrox-ui/README.md`) → `overlay` (done 2026-10-23, plan
   `plans/overlay.md`: `Dialog`, `ConfirmDialog`, `Sheet` on the native
   `<dialog>`, feature `overlay`)
   (`Dialog` with conrad's footer band, `Alert dialog`, `Sheet`) → `forms`
   (`Field`, `Input` with password strength, `Checkbox`, `Select`,
   `Combobox`) → `navigation` (side menu) → `layout` (`Panel`, from
   `ComponentTitleBar`, and `Card`) → `data` (`DataView`, `Pagination`) →
   `menus` (dropdown menu, menu button; added 2026-10-23).
7. **Docs.** README per crate (purpose, every public item with an example),
   root README as the map, CHANGELOG. Tag `v0.1.0` when webapp-lib is ready to
   switch.

New components from the inventory come after, one story each.

## Open points

Decided 2026-10-23: licence `MIT OR Apache-2.0`; fresh first commit; ferrox-ui
owns its CSS (conrad departs).

Still open (none block piece 0):

1. **Buttons:** plain `Button`/`IconButton` in `core` (proposed: other groups
   need them), split button and menu button in `menus`; or all buttons in
   `forms`?
2. **Name check:** `ferrox-ui` / `ferrox-theme` / `ferrox-icons` free on
   crates.io? (Matters only if we publish there; git tags work regardless.)
3. **Fonts:** ship Cormorant Garamond / DM Mono (OFL) self-hosted in
   `ferrox-theme`, or leave fonts to apps? Decide by piece 1.
4. The `ferrox-theme-build` version check (webapp-lib `plans/crate-split.md`).
5. The rest of the inventory: add, drop, move?
