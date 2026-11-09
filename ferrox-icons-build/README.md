# ferrox-icons-build

Build-time support for [ferrox-icons](../ferrox-icons): the rules an icon SVG
file must follow, and the code that turns a checked file into an `IconData`
const. ferrox-icons' own `build.rs` uses it, so every icon ferrox ships goes
through these rules, and so does `ferrox_icons::icon!` (in
[`ferrox-macros`](../ferrox-macros)) for an app's own icons, at compile time.

Part of [ferrox-ui](../README.md). Std + `roxmltree` only: a build script
that uses it never compiles Leptos.

## Public items

| Item | What it is |
|---|---|
| `check_file(path)` | Reads one icon file and checks it, named by its stem. `Ok(CheckedIcon)` or an error naming the broken rule (the caller adds the file name). |
| `check_markup(name, svg)` | Checks one icon given as a whole `<svg>` document, under a kebab-case name. |
| `CheckedIcon` | `name` (the file stem, e.g. `"chevron-down"`) and `body` (the SVG's children as compact markup, safe for `inner_html`). |
| `stem(path)` | The file stem as UTF-8. |

`discover`, `emit` and `sources` are public but hidden: they serve
ferrox-icons' own build script and are not a stable API.

```rust
// build.rs
let icon = ferrox_icons_build::check_file(std::path::Path::new("icons/star.svg"))
    .unwrap_or_else(|e| panic!("icons/star.svg: {e}"));
assert_eq!(icon.name, "star");
```

## The rules

Listed in [ferrox-icons' README](../ferrox-icons/README.md#adding-an-icon):
a `<svg>` root on a 24×24 grid with 2px round `currentColor` strokes and no
fill, drawing elements only, geometry and presentation attributes only,
`fill`/`stroke` only `none` or `currentColor`, no text, kebab-case file name.

## Requirements

- Used from a build script (`[build-dependencies]`).
- At the same ferrox tag as the app's other ferrox crates (`links` guard).

## Planned

- A solid (filled) icon style, in a later version.

## Testing

`cargo test -p ferrox-icons-build`: each rule's rejections, naming,
`SOURCES` parsing, file discovery, the generated markup and consts.
