# ferrox-icons

SVG icons for Leptos CSR apps. Each icon is a value, an `IconData` const
such as `CLOSE`. One component, `Icon`, draws any of them. Because icons are
values, a component can take one as a prop and an app can choose one at
runtime. The drawings are [Lucide](https://lucide.dev)'s (ISC), on a 24×24
grid with 2px round strokes.

Part of [ferrox-ui](../README.md). Apps using webapp-lib reach it as
`webapp_lib::icons` and don't list it themselves.

## Use

```rust
use ferrox_icons::{Icon, CHEVRON_DOWN, CHEVRON_RIGHT, CLOSE};

view! {
    // in an icon button: the button's title is the label
    <IconButton title="Close" on_click=move |_| open.set(false)><Icon icon=CLOSE/></IconButton>

    // extra classes: a string, or a Signal<String> to make them reactive
    // (a bare closure doesn't convert to MaybeProp<String>)
    <Icon icon=CLOSE class="dialog__close-icon"/>
    <Icon icon=CLOSE class=Signal::derive(move || if busy.get() { "is-busy".into() } else { String::new() })/>

    // chosen at runtime: a closure or a signal
    <Icon icon=move || if open.get() { CHEVRON_DOWN } else { CHEVRON_RIGHT }/>
}
```

`Icon` renders `<svg class="icon …" data-icon="close" viewBox="0 0 24 24"
width="1em" height="1em" aria-hidden="true" focusable="false">`:

- **Size** is `1em`, so an icon matches the font size around it.
- **Colour** is `currentColor`, so it follows the text colour and every theme.
- **Decorative:** the control around the icon carries the label. If an icon
  stands alone and means something, wrap it:
  `<span role="img" aria-label="Locked"><Icon icon=LOCK/></span>`.

### Public items

| Item | What it is |
|---|---|
| `Icon` | The component: `icon` (required: `IconData`, a closure, or a signal), `class` (optional, may be reactive). |
| `IconData` | One icon: `name` (the file stem, e.g. `"chevron-down"`) and `body` (the SVG's children). `#[non_exhaustive]`: only this crate makes them. |
| `icon_classes(extra)` | The class list `Icon` sets: `"icon"` or `"icon {extra}"`. |
| `ALL` | Every icon, sorted by name, for galleries and tests. It keeps every icon in the build, so apps name the icons they use instead. |

### Icons

| Const | Lucide | | Const | Lucide |
|---|---|---|---|---|
| `BAN` | `ban` | | `HELP` | `circle-question-mark` |
| `CAMERA` | `camera` | | `LOCK` | `lock` |
| `CHECK` | `check` | | `MENU` | `menu` |
| `CHEVRON_DOWN` | `chevron-down` | | `PLUS` | `plus` |
| `CHEVRON_RIGHT` | `chevron-right` | | `REFRESH` | `refresh-cw` |
| `CLOSE` | `x` | | `SEARCH` | `search` |
| `DELETE` | `trash` | | `UNLOCK` | `lock-open` |

## Adding an icon

Add icons with the component that first needs them, not ahead of time.

1. Find it on [lucide.dev](https://lucide.dev) and copy
   `https://raw.githubusercontent.com/lucide-icons/lucide/<commit>/icons/<lucide-name>.svg`
   unchanged to `icons/<our-name>.svg`. Use the commit in `icons/SOURCES`'s
   header. If you need a newer commit, update the header too. Our name is
   kebab-case, says what the icon is for (`close`, not `x`), and becomes the
   const name (`chevron-down` → `CHEVRON_DOWN`).
2. Add `<our-name> <lucide-name>` to `icons/SOURCES`.
3. Add a row to the icons table above. The demo's `/icons` gallery and its
   e2e tests pick the new icon up from `ALL` by themselves.

The build script checks every file and fails the build with the file name
and the rule broken:

- The root is `<svg>` with `viewBox="0 0 24 24"`, `fill="none"`,
  `stroke="currentColor"`, `stroke-width="2"`, and round line caps and joins.
  `width`/`height` are allowed and ignored. Nothing else is allowed on it.
- Drawing elements only: `path circle rect line polyline polygon ellipse g`.
- Only geometry and presentation attributes. No `id`, `class`, `style`,
  `on*` or `href`.
- `fill` and `stroke` are only `none` or `currentColor`.
- No text. Comments and whitespace are dropped.
- Every file has a `SOURCES` line, and every `SOURCES` line has a file.

These rules keep the markup static and safe to set with `inner_html`.

## Build size

An app's release build carries only the icons it names. In a release WASM
build (`opt-level = "z"`, LTO) that uses only `CLOSE`, `CLOSE`'s drawing is
in the `.wasm` and no other icon's is (measured 2026-10-23). `ALL` names
every icon, so using it keeps them all; it is for galleries and tests.

## Requirements

- `leptos` 0.8 (CSR).
- Sizing works without any CSS (the `1em` attributes). ferrox-ui's `.icon`
  class (`style/core/icon.css`) adds `display: block` and
  `pointer-events: none` for icons inside buttons.
- Apps can't make their own `IconData` (`#[non_exhaustive]`, so every drawing
  is one this crate checked). Until app icons arrive, an app draws its own
  `<svg class="icon" …>`.

## Testing

- `cargo test -p ferrox-icons`: the build script's rules (each rejection),
  naming, `SOURCES`, the generated markup, and checks that `ALL` matches
  `icons/*.svg` and no two icons draw the same.
- The demo's `/icons` page shows every icon. `demo/e2e/tests/icons.spec.ts`
  checks in Chromium and Firefox that each icon draws in the SVG namespace, is
  decorative and 1em, strokes in the text colour in every theme, and swaps
  its drawing when chosen at runtime.

## Planned

- More icons as components need them (see "Adding an icon").
- App-supplied icons through the same checks (a `ferrox-icons-build`, like
  the themes), when an app asks.

## Licence

The crate: `MIT OR Apache-2.0`. The icon drawings: Lucide's ISC licence, with
parts from Feather (MIT); see `icons/LICENSE-lucide`.
