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
| `/feedback` | `pages/feedback.rs` | `Spinner` (region, inline in a row action's place, field), shown on demand so the delayed reveal can be watched; `LoadingOverlay` for 2s |

The header and the theme bar (`theme_bar.rs`, plain `<select>`s for family and
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
| `tests/feedback.spec.ts` | Spinners: status role + hidden label, reveal armed on `--loading-delay`, sizes, 0.8s spin, reduced motion (2.4s, still revealed); the overlay covers the page and goes |
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
