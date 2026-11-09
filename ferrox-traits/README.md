# ferrox-traits

Small shared traits for records, with their derives: `HasId`, `HasName`,
`HasParentId`. Std only, so a crate shared by a server and a WASM app (an
app's `shared` DTO crate) can use them. webapp-lib re-exports them as
`webapp_lib::reactive::{HasId, HasName, HasParentId}`; an app on webapp-lib
uses that path and doesn't list this crate.

Part of [ferrox-ui](../README.md). Replaces the separate `ferrox-traits` and
`ferrox-webapp-macros` repos (same trait names, same derives and attributes).

## Use

```toml
# at the same tag as the app's other ferrox crates (and webapp-lib's)
ferrox-traits = { git = "https://github.com/rbenitez22/ferrox-ui", tag = "v0.2.2" }
```

```rust
use ferrox_traits::{HasId, HasName, HasParentId};   // the traits and their derives

#[derive(HasId, HasName, HasParentId)]
pub struct Store { pub id: String, pub name: String, pub parent_id: Option<String> }

#[derive(HasId, HasName)]
#[has_id(field = "email")]
#[has_name(field = "display_name")]
pub struct Invite { pub email: String, pub display_name: String }

impl HasId for Plain { fn get_id(&self) -> String { … } }   // or by hand
```

## Public items

| Item | What it is |
|---|---|
| `HasId` | `get_id(&self) -> String`. Derive: clones the field `id`, or `#[has_id(field = "…")]`. |
| `HasName` | `get_name(&self) -> String`. Derive: clones `name`, or `#[has_name(field = "…")]`. |
| `HasParentId` | `get_parent_id(&self) -> Option<String>`. Derive: clones `parent_id`, or `#[has_parent_id(field = "…")]`. |

The derives (from [`ferrox-macros`](../ferrox-macros), feature `traits`)
write `impl HasId for T` with the trait unqualified: import the derive and
the trait comes with it (same name), so it is in scope.

## Features

| Feature | Default | |
|---|---|---|
| `derive` | on | the three derives. Off: the traits only, no proc-macro build. |

## Requirements

- The field a derive reads must be `Clone` and of the trait's return type
  (`String`, or `Option<String>` for `HasParentId`).
- One copy per build (`links = "ferrox-traits"`): name the same ferrox tag
  as everything else, or Cargo refuses to resolve.

## Moving from the old repos

```toml
# before
ferrox-traits        = { git = "https://github.com/rbenitez22/ferrox-traits", tag = "v0.1.0" }
ferrox-webapp-macros = { git = "https://github.com/rbenitez22/ferrox-webapp-macros", tag = "v0.1.0" }
# after
ferrox-traits = { git = "https://github.com/rbenitez22/ferrox-ui", tag = "v0.2.2" }
```

```rust
// before
use ferrox_traits::{HasId, HasName};
use ferrox_webapp_macros::{HasId, HasName};
// after
use ferrox_traits::{HasId, HasName};
```

`FormModel` (the old macros repo's fourth derive) is not carried over: no
app used it.

## Testing

`cargo test -p ferrox-traits` (derives end to end, hand-written impls, the
doc example) and `cargo test -p ferrox-macros --all-features` (each derive's
expansion and errors).
