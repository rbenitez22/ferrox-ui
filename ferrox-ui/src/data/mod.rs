//! The `data` group (feature `data`): record lists rendered through one item
//! template and laid out as the screen needs.

mod data_view;
mod layout;
mod layout_switch;

pub use data_view::{data_view_classes, DataView};
pub use layout::{CardSize, DataLayout};
pub use layout_switch::{layout_icon, DataLayoutLabels, DataLayoutSwitch};
