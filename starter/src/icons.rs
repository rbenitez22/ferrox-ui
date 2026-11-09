//! The app's own icons: ones ferrox-icons doesn't have, each a const drawn
//! from an SVG in `icons/` and checked when the app compiles (same rules as
//! ferrox's icons; a Lucide SVG passes unchanged). Use them like ferrox's:
//! `<Icon icon=icons::EMPTY />`.

use ferrox_icons::{icon, IconData};

/// An empty list. `icons/empty.svg` is Lucide's `inbox`.
pub const EMPTY: IconData = icon!("icons/empty.svg");
