# ferrox-ui

Theme, UI primitives and icons for [Leptos](https://leptos.dev) CSR / WASM
apps. Token-based themes (one CSS file per theme, light and dark halves,
`system | light | dark` mode) and primitives styled by class-only CSS, so every
style is opt-in and every theme reaches every component. Primitives come in
broad Cargo feature groups: an app that needs buttons and dialogs compiles
nothing else.

## Status

Pre-0.1. The crates are being moved in from
[webapp-lib](https://github.com/rbenitez22/webapp-lib), where the theme system
and the first primitives were built and tested. Nothing is released yet. The
plan, with the decisions and the full component inventory, is
[`plans/ferrox-ui.md`](plans/ferrox-ui.md).

## Crates

| Crate | What it is |
|---|---|
| `ferrox-theme` | Runtime theme switching (families, `system`/`light`/`dark`, stored choice), the pre-paint script, Leptos signals (feature `leptos`). *Moved in.* |
| `ferrox-theme-build` | Build-script helper: the tokens, built-in themes and fonts; validates an app's theme files; writes the CSS file and fonts the app links in `index.html`. *Moved in.* |
| `ferrox-ui` | Primitives in feature groups: `core` (always on), `forms`, `menus`, `overlay`, `navigation`, `layout`, `data` |
| `ferrox-icons` | SVG icons, one function per icon |
| `ferrox-charts` | Charts on the theme's chart tokens (later) |
| `demo/` | Showcase of every theme and component, with Playwright end-to-end tests (not published). *Moved in: themes.* |

All crates share one version and are released together under one git tag.

## Using it

Not yet: there is no release. Once there is, depend on a tag, never a branch
or a local path:

```toml
ferrox-ui = { git = "https://github.com/rbenitez22/ferrox-ui", tag = "v0.1.0", features = ["overlay"] }
```

Apps that use [webapp-lib](https://github.com/rbenitez22/webapp-lib) get
ferrox-ui through its re-exports (`webapp_lib::ui`, `webapp_lib::theme`) and
must **not** list ferrox-ui or ferrox-theme themselves. Two references that
differ (tag, or `ssh://` vs `https://`) make Cargo build two copies of every
type. The only direct dependency such an app adds is `ferrox-theme-build`, in
`[build-dependencies]`, from the same tag webapp-lib uses; a mismatch fails
the build with a message saying so.

### Theme setup (`demo/` is a working example)

```toml
[build-dependencies]
ferrox-theme-build = { git = "https://github.com/rbenitez22/ferrox-ui", tag = "v0.1.0" }
```

```rust
// build.rs
fn main() {
    ferrox_theme_build::embed_themes("style/themes");                 // the app's own themes (optional)
    ferrox_theme_build::write_css("style/ferrox", Some("style/themes")); // CSS + fonts for index.html
}
```

```html
<!-- index.html <head>: the pre-paint script first (ferrox_theme::PREPAINT_SCRIPT), then -->
<link data-trunk rel="css" href="style/ferrox/ferrox.css" />
<link data-trunk rel="copy-dir" href="style/ferrox/fonts" />
```

```rust
// main.rs, before mounting
ferrox_theme::register(include!(concat!(env!("OUT_DIR"), "/ferrox_themes.rs")));
ferrox_theme::init();
```

```toml
# Trunk.toml — Trunk resolves index.html's links before it runs cargo, so the
# build script must run first, or a clean checkout fails to build.
[[hooks]]
stage = "pre_build"
command = "cargo"
command_arguments = ["check", "--quiet", "--target", "wasm32-unknown-unknown"]

[watch]
ignore = ["style/ferrox"]   # written by every build, never a reason for one
```

Optional loading screen, shown until the app mounts (in `<body>`):

```html
<div class="ferrox-loading">
  <div class="ferrox-loading__spinner"></div>
  <div class="ferrox-loading__text">Loading…</div>
</div>
```

Add `style/ferrox/` to `.gitignore`: it is generated.

## Developing

```sh
cd demo && trunk serve          # http://127.0.0.1:3200
cd demo/e2e && npm ci && npm test
scripts/release-check.sh        # the gate before any tag
```

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT licence ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 licence, shall
be dual licensed as above, without any additional terms or conditions.
