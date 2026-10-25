# Starter

The smallest [ferrox-ui](https://github.com/rbenitez22/ferrox-ui) app with
everything wired: copy this folder to start one.

```sh
rustup target add wasm32-unknown-unknown   # once
cargo install trunk                         # once
trunk serve                                 # http://127.0.0.1:3300
```

Rename it: `name` in `Cargo.toml`, `<title>` in `index.html`.

| File | Its job |
|---|---|
| `Cargo.toml` | ferrox-ui and its sibling crates, all pinned to one commit (with the lines for a local clone instead, commented); the feature groups in use |
| `build.rs` | writes `style/ferrox/`: the stylesheet, fonts and pre-paint script (generated, git-ignored) |
| `index.html` | inlines the pre-paint script, links the stylesheet and fonts, puts the page in the theme (`class="ferrox-app"` on `<body>`), shows a loading screen until the app mounts |
| `Trunk.toml` | runs the build script before Trunk reads `index.html`, so a clean checkout builds |
| `src/main.rs` | `ferrox_theme::init()` and `ferrox_ui::init()`, then mount |
| `src/app.rs` | one page: a list of items in a `Panel`, `Card`s in a `DataView`, each with a `Menu` (rename, delete), an add `Dialog`, a `ConfirmDialog`, loading and empty states |

Everything on the page comes from ferrox-ui and its theme: the app has no
CSS of its own. Replace `Item` and the in-memory store with your data.
