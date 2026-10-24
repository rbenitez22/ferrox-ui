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
| `ferrox-theme` | Runtime theme switching (families, `system`/`light`/`dark`, stored choice), Leptos signals (feature `leptos`). *Moved in.* |
| `ferrox-theme-build` | Build-script helper: the tokens, built-in themes and fonts; validates an app's theme files; writes the CSS file, fonts and pre-paint script the app links in `index.html`. *Moved in.* |
| `ferrox-ui` | Primitives in feature groups: `core` (always on: `Button`, `IconButton`, `Toolbar`, `Spinner`, `LoadingOverlay`, `Alert`, help on any component (`HelpTopic`, `HelpButton`, `HelpScope`, `HelpAssistant`: a `?` with an offline topic, and context for an app's assistant); call `ferrox_ui::init()` before mounting to add their CSS), feature `overlay` (`Dialog`, `ConfirmDialog`, `Sheet`), feature `forms` (`Field`, `Input`, `Textarea`, `PasswordInput`, `Checkbox`, `Switch`, `Select`, `Combobox`), feature `navigation` (`NavList`, `NavLink`, `NavGroup`, `SideMenu`, `SideMenuButton`), feature `layout` (`Panel`, also collapsible, and `Card`), feature `data` (`DataView` with infinite scroll, `DataLayoutSwitch`, `Pagination`), feature `menus` (`Menu`, `SplitButton`, `ContextMenu`). *Every group done: core, overlay, forms, navigation, layout, data and menus.* [README](ferrox-ui/README.md) |
| `ferrox-icons` | SVG icons (Lucide), checked at build time: each icon a const (`CLOSE`), drawn by one `Icon` component. 27 icons so far, added as components need them. [README](ferrox-icons/README.md) |
| `ferrox-charts` | Charts on the theme's chart tokens (later) |
| `starter/` | The smallest app with everything wired: copy it to start (not published; not a workspace member, it depends on ferrox-ui by git). `scripts/starter-local.sh` builds it against the working tree for the tests. |
| `demo/` | Showcase of every theme and component, with Playwright end-to-end tests (not published). Pages: themes, buttons, icons, feedback, overlay, forms, help, navigation, layout, data, menus. [README](demo/README.md) |

All crates share one version and are released together under one git tag.

## Start here

**A new app:** copy [`starter/`](starter/), rename it (`name` in its
`Cargo.toml`, the `<title>` in its `index.html`), and run `trunk serve` in
it (http://127.0.0.1:3300). It's the smallest app with everything wired,
and one page built from ferrox-ui: a list of items with add, rename,
delete-with-confirm, loading and empty states. You need
[Trunk](https://trunkrs.dev) and the `wasm32-unknown-unknown` target
(`rustup target add wasm32-unknown-unknown`).

**An existing Leptos CSR app:** the whole setup is in five of the
starter's files. Copy each part into yours:

| Starter file | What to take |
|---|---|
| [`Cargo.toml`](starter/Cargo.toml) | the ferrox dependencies (pinned to one commit until `v0.1.0`), the feature groups you need |
| [`build.rs`](starter/build.rs) | the `write_css` call: it writes `style/ferrox/` (stylesheet, fonts, pre-paint script) |
| [`index.html`](starter/index.html) | the `<link data-trunk>` lines in `<head>`, in that order; `class="ferrox-app"` on `<body>` (the page in the theme's background, text colour and font); the loading screen in `<body>` |
| [`Trunk.toml`](starter/Trunk.toml) | the `pre_build` hook and the `[watch]` ignore |
| [`src/main.rs`](starter/src/main.rs) | `ferrox_theme::init()` and `ferrox_ui::init()` before mounting |

…and `style/ferrox/` in `.gitignore` ([`starter/.gitignore`](starter/.gitignore)): it's generated.

**With an agent:** install the agent kit into your app:

```sh
ferrox-ui/agent-kit/install/install.sh --claude path/to/app   # or --copilot, --agents-md, --generic; install.ps1 on Windows
```

With an adapter flag the agent finds the kit by itself; with `--generic`,
paste the paragraph in `path/to/app/.ferrox/BOOTSTRAP.md`. From then on,
"use ferrox-ui" is the whole UI directive: the kit tells the agent how to
set up, build screens from recipes, pick components, and check its work.
The kit works with any agent that reads files and runs commands; see
[its README](ferrox-ui/agent-kit/README.md).

**Versions.** Every crate here shares one version and one git reference.
Use the same reference (the starter's `rev`, later a release tag) for all
of them: two references, even to the same commit (`ssh://` and
`https://`), make Cargo build two copies of every type.

### Your own themes

The built-in themes need nothing more. To add your own (or replace a
built-in with your own file of the same id), put theme files in a folder,
embed and write them from `build.rs`, and register them before `init()`:

```rust
// build.rs
fn main() {
    ferrox_theme_build::embed_themes("style/themes");
    ferrox_theme_build::write_css("style/ferrox", Some("style/themes"));
}
```

```rust
// main.rs, before mounting
ferrox_theme::register(include!(concat!(env!("OUT_DIR"), "/ferrox_themes.rs")));
ferrox_theme::init();
```

[`demo/`](demo/) does this with its own themes.

## Developing

```sh
cd demo && trunk serve          # http://127.0.0.1:3200
scripts/starter-local.sh serve  # the starter, built against this tree (http://127.0.0.1:3300)
cd demo/e2e && npm ci && npm test  # also serves and tests the starter
scripts/release-check.sh        # the gate before any tag
```

The release check needs `trunk`, Node (for Playwright) and
[cargo-hack](https://github.com/taiki-e/cargo-hack) (`cargo install cargo-hack`),
which builds every crate with each of its features alone.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT licence ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 licence, shall
be dual licensed as above, without any additional terms or conditions.
