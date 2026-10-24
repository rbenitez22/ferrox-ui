# ferrox-ui demo

A Leptos CSR app that shows every theme and component, wired exactly the way
an app is (except that it depends on the crates by `path`; apps use a git
tag). It is also what the Playwright e2e tests run against, so it gates every
release. Not published.

## Run

```sh
cd demo
trunk serve          # http://127.0.0.1:3200
```

`?theme=<family>:<mode>` picks a theme on load, for links and screenshots:
`http://127.0.0.1:3200/?theme=jamaica:dark`.

Trunk watches only `demo/`. After changing a library crate, restart
`trunk serve` or make a real edit under `demo/src`. Touching a file without
changing it does not trigger a rebuild.

## Pages

| Route | Page | Shows |
|---|---|---|
| `/` | `pages/themes.rs` | Every surface, text, role (explicit and derived slots) and chart token of the active theme, with the values the browser computed |
| `/buttons` | `pages/buttons.rs` | `Button` (families, quiet, sizes, disabled, saving), `IconButton` (plain, pressed, danger, disabled), `Toolbar` (standard, compact, grouped) |
| `/icons` | `pages/icons.rs` | Every ferrox-icons icon (`ALL`), one icon at 16/24/32px (it is 1em), and an icon chosen at runtime (chevron right/down) |
| `/feedback` | `pages/feedback.rs` | `Spinner` (region, inline in a row action's place, field), shown on demand so the delayed reveal can be watched; `LoadingOverlay` for 2s; `Alert` (each severity, no icon, dismissible) |
| `/overlay` | `pages/overlay.rs` | `Dialog`: a form (autofocus field, textarea, Save with saving, Save as draft), a wide dialog, one with no dismiss button, and a dialog opened from a dialog; `ConfirmDialog` (Delete account, with a saving step); `Sheet` at the left, right and bottom; a log of every way out |
| `/forms` | `pages/forms.rs` | `Field` (hint, live error), `Input` (text, email, search outside a field), `Textarea`, each value echoed below, and a button that sets values from code; `PasswordInput` to sign in and a new password with its strength meter; `Checkbox` (hint; error until ticked), `Switch`, `Select` (plain; and one that loads and refreshes its options); `Combobox` of 24 accounts (with refresh), and the same inside a `Dialog` |
| `/help` | `pages/help.rs` | `HelpButton` with three topics (paragraphs; a list; a table with a Full guide link), a `?` out of the Tab order in a row of fields, `?`s at the right edge and the bottom for placement, and an "Asking an assistant" section: `HelpAssistant`, nested `HelpScope`s (screen, a changing account, a sibling scope), a `?` that sends its field's value, and a fake assistant Sheet showing the request; "Help on components": the `help` prop on Toolbar, Field (shared value, not shared, password), Checkbox, Switch (`help_skip_tab`), Dialog, ConfirmDialog and Sheet, inside an assistant |
| `/layout` | `pages/layout.rs` | `Panel`: a page's own panel (H1) whose `+` counts clicks; section panels: Items (a search action, a `+` behind a "May add items" switch, help), Load Data (no buttons), a very long title; variants: compact, flush, warning, danger; collapsible: Notes (summary, help, a `+`, a field, and a count of how often its body was built), a danger zone, one that opens expanded, a compact one, and one whose state the page owns |
| `/navigation`, `/navigation/details` | `pages/navigation.rs` | `NavList` of every demo page, links with icons, and a nested list (a link current below its path, an `exact` one, and a link to `/navigation/details`) in a narrow column; `NavGroup`: the shopping-list menu's shape (two links, a collapsible "Product database" holding `/navigation/details`, a plain "Account"), one that starts expanded, and one whose state the page owns (toggled from code) |

The header (with `SideMenuButton`), the side menu (`menu.rs`: every page, grouped, with help and a footer link; outside the routes, as an app's) and the theme bar (`theme_bar.rs`, plain `<select>`s for family and
mode, plus the active `data-theme`) are on every page.

### Adding a page

1. Add `src/pages/<name>.rs` with a `#[component] pub fn <Name>Page()`; use
   `crate::section::Section` for each titled block.
2. Declare it in `src/pages/mod.rs`.
3. Add a `<Route>` and a nav link in `src/app.rs`.
4. Add the route to `e2e/tests/smoke.spec.ts`, and a spec for what it shows.

## Theme wiring (same as an app)

- `build.rs` calls `ferrox_theme_build::embed_themes("style/themes")` (the
  demo's own themes, for `register`) and `write_css("style/ferrox", …)` (the
  stylesheet and fonts, into `style/ferrox/`, git-ignored).
- `index.html` carries a copy of `ferrox_theme::PREPAINT_SCRIPT`, links
  `style/ferrox/ferrox.css` and copies `style/ferrox/fonts`, and shows the
  `.ferrox-loading` screen until the app mounts. `tests/prepaint.rs` fails if
  the script copy drifts.
- `Trunk.toml` runs `cargo check` as a pre-build hook, because Trunk resolves
  `index.html`'s links before it runs cargo; without it a clean checkout fails
  to build. It also ignores `e2e/` and `style/ferrox/` when watching.
- `main.rs` calls `ferrox_theme::register(include!(…))`, then `init()`, then
  mounts and sets `<body data-ready="true">`.
- `style/themes/harbor.css` is a demo-only theme. It shows that an app adds a
  theme by adding one file.

`style/demo.css` is demo-only layout; class-only and token-only, like the
library's CSS.

## End-to-end tests

Playwright (TypeScript, Chromium + Firefox) in `e2e/`. One-time setup: npm,
then `cd e2e && npm ci && npx playwright install chromium firefox` (browsers
are cached per machine in `~/.cache/ms-playwright`).

```sh
cd demo/e2e
npm test               # against a running `trunk serve` on :3200, or starts one
npm run test:ui        # interactive: pick tests, watch them, time-travel
npm run report         # the HTML report of the last run
E2E_FRESH=1 npm test   # own server on :3299, fresh build (what the release check runs)
```

| File | What |
|---|---|
| `playwright.config.ts` | server, browsers, the two modes above |
| `fixtures.ts` | `openApp(page, path)` waits for `<body data-ready="true">`; every test fails on a page error or `console.error`, except patterns in `test.use({ expectedErrors: /…/ })` (one regex, not an array) |
| `tests/smoke.spec.ts` | every route loads |
| `tests/buttons.spec.ts` | Button / IconButton / Toolbar, including "no neighbour moves" on hover, press and saving |
| `tests/icons.spec.ts` | Every icon draws (SVG namespace, non-empty box), is decorative, follows the font size and the text colour in every theme; a runtime choice swaps the drawing |
| `tests/feedback.spec.ts` | Spinners: status role + hidden label, reveal armed on `--loading-delay`, sizes, 0.8s spin, reduced motion (2.4s, still revealed); the overlay covers the page and goes. Alerts: role per severity, icon per severity (none with `icon=false`), text and border from the severity tokens in two themes, dismiss, and the close button adds no height |
| `tests/overlay.spec.ts` | Dialog: a real modal labelled by its title, title bar layout, focus in / kept / returned, fresh content per opening, Esc / × / Cancel each call `on_close` once, the backdrop doesn't close it, Enter (submits once from a field, a newline in a textarea, presses a focused button), footer order, `no_dismiss`, scroll lock (also nested), sizes, phone layout. ConfirmDialog: an alertdialog with a danger verb last, focus on ×, Cancel / Esc call `on_cancel` once, Delete runs once through saving, reopening starts ready. Sheet: pinned to each edge at its size, a real modal (focus, lock, focus back), backdrop click / Esc / × each call `on_close` once, clicks inside and a drag out don't close it, slides in over 0.2s and not under reduced motion |
| `tests/forms.spec.ts` | Field: the label names the control, the hint or error describes it (never both), an error marks it invalid (`aria-invalid`, `.is-invalid`, danger border), no message means no description, unique ids inside and outside fields. Input / Textarea: two-way values, kinds and attributes, ≥ 16px on a phone. PasswordInput: hidden by default, the toggle is a button (`aria-pressed`, eye / eye-off, Space), sits inside the field, `autocomplete`, a meter only when asked and not empty, each level and its word, described by hint + meter. Checkbox: words are the label, Space, hint then error. Switch: role, click and Space, the knob slides. Select: placeholder can't be chosen, two-way value, loading (disabled, busy, says so), refresh button with a spinner, same element keeps focus and choice when options change, its own caret. Combobox: ARIA combobox pattern, filtering, arrows / Enter / Tab / Esc, click, leaving with no match reports none, chevron and clear (out of the Tab order), refresh, and in a dialog: not clipped, Enter doesn't submit, Esc keeps the dialog |
| `tests/help.spec.ts` | The `?` names its topic (label and tooltip), opens it in the top layer below itself with `aria-expanded`, renders paragraphs / lists / tables / the guide link, Esc closes and returns focus, an outside click closes, one open at a time, `skip_tab`, kept inside the viewport, above when there's no room, follows scrolling. Asking: nothing to ask outside an assistant; the request's topic, component, label and value; context outermost first; context read when asking; a scope doesn't reach its siblings; a value only where given; asking closes the popover. Components: each `?`'s place (after a label and outside it, beside a switch, by a toolbar's title, before a title bar's ×), a `?` never ticks its checkbox, the request's component and label per component, a shared value, never a password |
| `tests/layout.spec.ts` | Panel: a region named by its heading at the level asked for; title left, actions right, the `+` last (the `PLUS` icon); `on_add` and actions run per click; `can_add` removes and restores the `+`; no buttons, same bar height; help right after the title; compact slims band and body; flush has no body padding; each tone tints band and border, danger the title; on a phone the title ends in an ellipsis beside the buttons and the `+` is 44px. Collapsible: the toggle is a button inside the heading and the region keeps the title alone as its name; shut at first with no body built; built once, typed text survives reopening; the summary only while shut; Enter / Space and the chevron; actions and `+` only while open (the `?` always), outside the toggle; `start_expanded`; a two-way owned state; one line at a shut panel's bottom; the toggle reaches the band's left edge, 44px on a phone |
| `tests/navigation.spec.ts` | NavList: a labelled `nav` with a list of `.nav-link`s. NavLink: the current page is `aria-current` and tinted (colour or gradient), `/` isn't current elsewhere, current below its path unless `exact` (followed by a click), the icon first and hidden, Tab reaches each link with the focus ring, hover and press move nothing, ≥ 44px on a phone. NavGroup: a plain group's heading names its list (hairline, not focusable); a collapsible one starts collapsed, toggles by click / Enter / Space, the chevron turns, Tab skips its hidden links; `start_expanded`; a two-way owned state; it expands itself when the route moves into it, or when the page opens inside it (and can still be collapsed); ≥ 44px toggle on a phone. SideMenu (the demo's own): the button's `aria-haspopup` / `aria-expanded` / icon; a modal sheet named by its title with a labelled nav; following a NavLink, a plain `<A>` in the footer, or the current page's link closes it; the back button closes it; Esc / × / backdrop close it and return focus to the button; the footer sits at the bottom; help in the title bar; a collapsible group starts collapsed on each opening unless the current page is in it |
| `tests/themes.spec.ts` | choosing, persisting, pre-paint, styled before mount, loading screen, fonts, legacy key, `?theme=`, system mode, fallbacks |

Writing a test:

- Import `test`, `expect`, `openApp` from `../fixtures`, never from
  `@playwright/test`, so the page-error check applies.
- Find elements by role and label. An element without an accessible name is a
  bug to fix.
- Fake time with `page.clock.install()` before `openApp`. After an action that
  fires a request, wait for its effect before moving the clock again:
  responses arrive in real time.
- `toBeEnabled()` treats `aria-disabled="true"` as disabled; check the
  `disabled` attribute instead for a busy button.
- A known failure is `test.fail()` with a comment saying when to remove it.

The release gate is `scripts/release-check.sh` at the repo root.
