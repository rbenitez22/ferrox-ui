# ferrox-ui

Theme, UI primitives and icons for [Leptos](https://leptos.dev) CSR / WASM
apps. Token-based themes (one CSS file per theme, light and dark halves,
`system | light | dark` mode) and primitives styled by class-only CSS, so every
style is opt-in and every theme reaches every component. Primitives come in
broad Cargo feature groups: an app that needs buttons and dialogs compiles
nothing else.

The component inventory follows [rust-ui.com](https://www.rust-ui.com) in
behaviour, not construction: ferrox-ui is an ordinary crate dependency, with
no copy-in CLI and no Tailwind.

## Status

Pre-0.1. The crates are being moved in from
[webapp-lib](https://github.com/rbenitez22/webapp-lib), where the theme system
and the first primitives were built and tested. Nothing is released yet. The
plan, with the decisions and the full component inventory, is
[`plans/ferrox-ui.md`](plans/ferrox-ui.md).

## Crates (planned)

| Crate | What it is |
|---|---|
| `ferrox-theme` | Design tokens, themes, runtime theme switching, the pre-paint script |
| `ferrox-theme-build` | Build-script helper: discovers and validates an app's theme files, writes the CSS file the app links in `index.html` |
| `ferrox-ui` | Primitives in feature groups: `core` (always on), `forms`, `menus`, `overlay`, `navigation`, `layout`, `data` |
| `ferrox-icons` | SVG icons, one function per icon |
| `ferrox-charts` | Charts on the theme's chart tokens (later) |
| `demo/` | Showcase of every theme and component, with Playwright end-to-end tests (not published) |

All crates share one version and are released together under one git tag.

## Using it

Not yet: there is no release. Once there is, depend on a tag, never a branch
or a local path:

```toml
ferrox-ui = { git = "https://github.com/rbenitez22/ferrox-ui", tag = "v0.1.0", features = ["overlay"] }
```

Apps that use [webapp-lib](https://github.com/rbenitez22/webapp-lib) get
ferrox-ui through its re-exports and need no separate dependency.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT licence ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 licence, shall
be dual licensed as above, without any additional terms or conditions.
