# ferrox-icons: SVG icons as Leptos functions

Piece 5 of `plans/ferrox-ui.md` (the SVG icon story). One SVG file per icon
in `ferrox-icons/icons/`. The build script validates each one and generates
one `const` per icon. A single `Icon` component renders any of them:
`<Icon icon=CLOSE/>`. Because icons are values, components can take one as a
prop (`<MenuLink icon=DELETE …>`) and an app can choose one at runtime.

Status: approved (2026-10-23). Pieces 1–3 done (2026-10-23); piece 4 next.

Changes from the design during piece 1:
- `IconData` has only `name` and `body`. Validation requires Lucide's root
  attributes exactly (`fill="none"`, `stroke="currentColor"`,
  `stroke-width="2"`, round caps and joins), so `Icon` writes them itself and
  doesn't need per-icon fields. A future icon style would bring the fields back.
- `icons/SOURCES` (`<ours> <lucide>` per line) records the mapping and the
  Lucide commit. The build checks that it matches the files one to one.
- `LICENSE-lucide` came in with the first file (piece 1, not 2).
- There's no keyword or clash check: upper-case consts can't be keywords, and
  kebab-case stems can't clash once converted.
- Piece 2: Lucide renamed `trash-2` → `trash` (the old `trash-2` drawing) and
  `circle-help` → `circle-question-mark`. The current names are used.
- A reactive `class` is a `Signal<String>`. Leptos's `MaybeProp<String>`
  doesn't take a bare closure.

## Why

- webapp-lib's UTF-8 glyph classes (`icons.css`: `multiply`, `plus-sign`,
  `trigram-for-heaven`, `open-lock`, `clockwise-gapped-circle-arrow`) were
  abandoned. Review item X7 asks for icon slots plus SVG.
- The demo draws its own SVG paths today (`demo/src/pages/buttons.rs:135`,
  `DemoIcon`), described as "until the SVG icon story lands".
- Coming components need icons: Dialog (close), Select/Combobox (chevron),
  Side menu (menu), Data grid (delete, refresh).

## Design

### Source set

Icons come from **Lucide** (ISC licence, a fork of Feather, MIT). Every icon
is drawn on a 24×24 grid with a 2px stroke in `currentColor`. That's the same
style as the demo's hand-drawn paths, so the look doesn't change. The files are
copied in unchanged, renamed to our names (`x.svg` → `close.svg`), with
`icons/LICENSE-lucide` beside them. There's no npm and no download at build
time.

### Crate layout

```
ferrox-icons/
  Cargo.toml        leptos; build-dependency roxmltree
  build.rs          discover → validate → generate $OUT_DIR/icons.rs
  icons/*.svg       one file per icon; the file stem is the function name
  icons/LICENSE-lucide
  src/lib.rs        IconData, include!(concat!(env!("OUT_DIR"), "/icons.rs"))
  src/icon.rs       the Icon component
```

It has no CSS of its own. Sizing and the pointer-events rule come from
ferrox-ui's `.icon` class (see "CSS" below). The SVG's `width="1em"` /
`height="1em"` attributes keep a bare icon sensibly sized for an app without
ferrox-ui.

### Build script: validation

The script parses each file with `roxmltree`, a small build-time-only parser
with no dependencies. It fails the build with the file name and the reason
when:

- the root isn't `<svg>`, or `viewBox` isn't `0 0 24 24`;
- an element isn't one of `path circle rect line polyline polygon ellipse g`
  (so no `script`, `style`, `use`, `image`, `foreignObject`, `text`);
- an attribute is outside the allowed geometry and presentation set (so no
  `id`, `class`, `style`, `on*`, `href`);
- a `fill` or `stroke` value is anything other than `none` or `currentColor`,
  so every icon follows the text colour of each theme;
- the file stem isn't kebab-case, or its snake_case form is a Rust keyword or
  clashes with another icon.

These rules keep the generated `inner_html` (next section) static and
known-safe.

### API: icons as values, one `Icon` component (decided 2026-10-23)

Handwritten in `src/`:

```rust
/// One icon's data. Every field is a validated, static string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct IconData {
    pub name: &'static str,   // "close"
    pub body: &'static str,   // the SVG's children, e.g. `<path d="…"/>`
    pub fill: &'static str,   // root presentation attributes from the file
    pub stroke: &'static str,
    pub stroke_width: &'static str,
    // + stroke_linecap, stroke_linejoin (all on the 24×24 grid)
}

/// Renders an icon: `<svg class="icon {class}" viewBox="0 0 24 24"
/// width="1em" height="1em" aria-hidden="true" focusable="false">`.
#[component]
pub fn Icon(
    #[prop(into)] icon: Signal<IconData>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView
```

Generated for each `icons/<name>.svg` (`$OUT_DIR/icons.rs`):

```rust
/// `close.svg`: Lucide `x` (ISC).
pub const CLOSE: IconData = IconData { name: "close", body: "<path d=\"M18 6 6 18\"/><path d=\"m6 6 12 12\"/>", … };
```

plus `pub const ALL: &[IconData]`, sorted by name, for the demo gallery and
the tests.

Use:

```rust
use ferrox_icons::{Icon, CHEVRON_DOWN, CHEVRON_RIGHT, CLOSE, DELETE};

<IconButton title="Close" on_click=…><Icon icon=CLOSE/></IconButton>
<Icon icon=CHEVRON_DOWN class="select__chevron"/>
<Icon icon=move || if open.get() { CHEVRON_DOWN } else { CHEVRON_RIGHT }/>
<MenuLink href="/items" icon=DELETE>"Items"</MenuLink>   // a component taking an icon prop (piece 6+)
```

- **Why values:** components in the inventory take an icon as a prop (menu
  items, side-menu links, Alert, Empty, Toast). A value is easy to store,
  compare, pass around and switch on state. `CLOSE` can't clash with a
  component named `Close` or `Menu`. And there's only one macro expansion,
  not one per icon.
- **Changing course later** (before `v0.1.0`): per-icon components can be
  added on top. The generator emits `CloseIcon` as `<Icon icon=CLOSE/>`, and
  both styles coexist. Removing the values would cost more: every component
  with an `icon` prop would need a children/`ViewFn` slot instead.
- **Reactive `icon`** (`Signal<IconData>`): a const, a signal or a closure all
  work. `body` is set with `inner_html`, so a change swaps the markup in place.
- **Decorative by default** (`aria-hidden`, `focusable="false"`). The control
  around the icon carries the label; `IconButton` already sets `aria-label`
  from `title`. An icon that stands alone with meaning gets wrapped by the app
  (`<span role="img" aria-label=…>`). That stays out of this crate.
- **`inner_html` rather than generated `view!` nodes.** One string per icon
  instead of one DOM call per element, so less WASM. The browser parses it in
  the SVG namespace because the context element is `<svg>`. An e2e test checks
  the namespace.
- **No per-icon Cargo features.** Release builds drop consts that nothing
  uses. Piece 4 below measures this before we rely on it (`ALL` keeps every
  icon alive, so only the demo and tests use it).
- `class` follows X3 (`MaybeProp`, reactive, no `style` prop). Size and colour
  come from CSS (`font-size`, `color`) or that class.

### First set

`close`, `plus`, `menu`, `refresh`, `check`, `delete`, `chevron-down`,
`chevron-right`, `search`, `lock`, `unlock`, plus `help`, `camera`, `ban`, so
the demo has no hand-drawn SVGs left.

| Ours | Lucide |
|---|---|
| close | x |
| plus | plus |
| menu | menu |
| refresh | refresh-cw |
| check | check |
| delete | trash |
| chevron-down | chevron-down |
| chevron-right | chevron-right |
| search | search |
| lock | lock |
| unlock | lock-open |
| help | circle-question-mark |
| camera | camera |
| ban | ban |

### CSS

The `.icon` rule (`ferrox-ui/style/core/buttons.css:242`) isn't
button-specific. It moves to its own core sheet, `ferrox-ui/style/core/icon.css`,
injected after `buttons.css`. It keeps `display: block; flex: none;
pointer-events: none`. Size comes from the `1em` attributes, so an icon matches
the font size around it.

### Dependencies

ferrox-ui doesn't depend on ferrox-icons yet. It takes the dependency in
piece 6, when a component first draws an icon (Dialog's close button). That
way nothing ships an unused dependency. webapp-lib re-exports it as
`webapp_lib::icons` (crate-split plan).

## Pieces

Each one passes `scripts/release-check.sh` and can be reviewed on its own.

1. **Crate + build script, one icon.** `ferrox-icons` in the workspace with
   `roxmltree` as a workspace build-dependency. `IconData`, the `Icon`
   component, and a `build.rs` that discovers, validates and generates
   `CLOSE`. Unit tests on the validator
   (the logic lives in a `build/` module that the tests include): each
   rejection rule, kebab → snake naming, keywords, clashes. Add
   `ferrox-icons` to `crates=(…)` in the release check.
2. **First set + licence + `ALL`.** The other icons, `LICENSE-lucide`, the
   `ALL` table, and a unit test that `ALL` matches `icons/*.svg` (names,
   order, no duplicates).
3. **`.icon` CSS sheet + demo `/icons` gallery + e2e.** Move the `.icon` rule
   into `core/icon.css` (class-only test covers it). The demo gets a gallery
   page (name + icon, rendered from `ALL`) and a header link.
   `demo/e2e/tests/icons.spec.ts` checks that every icon renders, that its
   children are in the SVG namespace, that it has a non-zero bounding box and
   `aria-hidden="true"`, and that its stroke follows the text colour when a
   theme changes, and that a reactive `icon` swaps the drawing.
4. **Demo buttons use the crate.** Replace `DemoIcon` and its path constants
   in `buttons.rs` with `<Icon icon=…/>`. `buttons.spec.ts` keeps passing.
   Check that a release build of the demo has no unused icon strings
   (`grep` the `.wasm` for a path from an unused icon).
5. **Docs.** `ferrox-icons/README.md` (purpose, how to call an icon, the
   validation rules, how to add an icon, Lucide licence). Root README map and
   `plans/ferrox-ui.md` status.

## Decisions (2026-10-23)

1. **Source set:** Lucide (ISC).
2. **First set:** as listed above, including `lock` / `unlock` (for
   webapp-lib's `open-lock` glyph).
3. **App icons:** later, when an app asks (a `ferrox-icons-build`, like the
   themes). Until then an app writes its own `<svg class="icon">`.
4. **Names:** our own (`CLOSE`, `DELETE`, `REFRESH`), mapped from Lucide's
   (`x`, `trash`, `refresh-cw`). The doc comment on each const names the
   Lucide source.
