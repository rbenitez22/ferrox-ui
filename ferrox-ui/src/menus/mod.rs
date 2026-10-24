//! The `menus` group (feature `menus`): lists of actions that hang off the
//! control that offered them.

mod item;
mod keys;
mod menu;

pub use item::{MenuDivider, MenuItem};
pub use menu::{Menu, MenuAlign, MenuTrigger};
