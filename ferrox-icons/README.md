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

    // extra classes (a string, or a Signal<String> to make them reactive)
    <Icon icon=CLOSE class="dialog__close-icon"/>

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
| `CLOSE` | Lucide `x`. |

## Adding an icon

1. Copy the Lucide file unchanged to `icons/<our-name>.svg`. The name is
   kebab-case and becomes the const name (`chevron-down` → `CHEVRON_DOWN`).
2. Add `<our-name> <lucide-name>` to `icons/SOURCES`.

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

## Requirements

- `leptos` 0.8 (CSR).
- Sizing works without any CSS (the `1em` attributes). ferrox-ui's `.icon`
  class adds `display: block` and `pointer-events: none` for icons inside
  buttons.

## Planned

`plans/ferrox-icons.md`: the rest of the first set (`plus`, `menu`, `refresh`,
`check`, `delete`, `chevron-down`, `chevron-right`, `search`, `lock`,
`unlock`, `help`, `camera`, `ban`), an `ALL` table, a demo gallery page with
e2e tests, and the `.icon` CSS in its own core sheet. Later: app-supplied
icons through the same checks.

## Licence

The crate: `MIT OR Apache-2.0`. The icon drawings: Lucide's ISC licence, with
parts from Feather (MIT); see `icons/LICENSE-lucide`.
