//! The `layout` group (feature `layout`): boxes that title and hold a page's
//! content.

mod heading;
mod panel;

pub use heading::HeadingLevel;
pub use panel::{panel_classes, Panel, PanelTone};
