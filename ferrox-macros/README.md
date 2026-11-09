# ferrox-macros

Procedural macros for the ferrox-ui crates. Each macro family sits behind its
own Cargo feature and is re-exported by the crate it serves, so an app uses
`ferrox_icons::icon!` and never depends on this crate directly.

Part of [ferrox-ui](../README.md). All ferrox procedural macros go here, so
new ones don't each need a crate.

## Macros

| Feature | Macro | Re-exported as | Docs |
|---|---|---|---|
| `icons` | `icon!` | `ferrox_icons::icon!` | [ferrox-icons, "App icons"](../ferrox-icons/README.md#app-icons) |

### `icon!` (feature `icons`)

An `IconData` const from the app's own SVG, checked at compile time by
[`ferrox-icons-build`](../ferrox-icons-build)'s rules:

```rust
use ferrox_icons::{icon, IconData};

pub const PIN: IconData = icon!("icons/pin.svg");                      // file, from the crate root; name = stem
pub const SPARK: IconData = icon!(name = "spark", r#"<svg …>…</svg>"#);  // inline, named
```

- A string starting with `<` is markup and needs `name = "…"`; anything else
  is a file path (relative to `CARGO_MANIFEST_DIR`) and takes no name.
- A broken drawing is a compile error at the string, naming the rule.
- A file is also `include_bytes!`'d, so editing it rebuilds the crate.
- Expands to `::ferrox_icons::IconData::__checked(name, body)`: the caller
  must depend on `ferrox-icons` under that name.

## Adding a macro family

1. A module in `src/` behind a new feature, with an `expand(TokenStream) ->
   syn::Result<TokenStream>` that the unit tests call directly.
2. The `#[proc_macro…]` entry point in `src/lib.rs`, gated on the feature.
3. The serving crate depends on `ferrox-macros` with that feature and
   re-exports the macro; generated code names that crate by absolute path
   (`::ferrox_icons::…`), and the crate adds `extern crate self as …;` so the
   path works in its own tests.
4. A row in the table above.

## Requirements

- `syn` 2, `quote`, `proc-macro2`; `ferrox-icons-build` with `icons`.
- At the same ferrox tag as the app's other ferrox crates (it comes in
  through them).

## Planned

- Derives for general traits (e.g. `HasId`, `HasName`), if they move into
  ferrox (see patina `plans/web-to-ferrox.md`, "Afterwards").

## Testing

`cargo test -p ferrox-macros --all-features`: each macro's parsing, errors
and expansion. ferrox-icons' tests use `icon!` end to end.
