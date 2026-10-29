//! UI primitives for Leptos CSR apps, styled by class-only CSS on
//! ferrox-theme's tokens, so every theme reaches every component.
//!
//! Components come in feature groups. `core` is always compiled:
//! [`Button`], [`IconButton`], [`Toolbar`] / [`ToolbarGroup`], [`Spinner`],
//! [`LoadingOverlay`], [`Alert`], [`HelpButton`] / [`HelpTopic`]. Feature
//! `overlay`: `Dialog`, `ConfirmDialog`, `Sheet`. Feature `forms`: `Field`,
//! `Input`, `Textarea`, `PasswordInput`, `Checkbox`, `Switch`, `Select`,
//! `Combobox`. Feature `navigation`: `NavList`, `NavLink`,
//! `NavGroup`, `SideMenu`, `SideMenuButton`. Feature `layout`: `AppBar`, `PageGrid`,
//! `Stack`, `Cluster`, `Panel` (also collapsible), `Card`, `Facts`. Feature `data`: `DataView`,
//! `DataLayoutSwitch`, `Pagination`. Feature `menus`: `Menu`, `MenuItem`,
//! `MenuLink`, `MenuDivider`, `SplitButton`, `ContextMenu`. Feature `media`:
//! `Carousel`.
//!
//! Call [`init`] once at startup, before mounting, to add the components'
//! CSS. The theme itself (tokens, themes, fonts) comes from ferrox-theme.
//!
//! ```rust,ignore
//! ferrox_theme::init();
//! ferrox_ui::init();
//! leptos::mount::mount_to_body(App);
//! ```

mod core;
mod css;
#[cfg(feature = "data")]
mod data;
#[cfg(feature = "forms")]
mod forms;
mod ids;
#[cfg(any(feature = "layout", feature = "menus"))]
mod once;
#[cfg(feature = "layout")]
mod layout;
#[cfg(feature = "media")]
mod media;
#[cfg(feature = "menus")]
mod menus;
#[cfg(feature = "navigation")]
mod navigation;
#[cfg(feature = "overlay")]
mod overlay;

pub use core::*;
#[cfg(feature = "data")]
pub use data::*;
#[cfg(feature = "forms")]
pub use forms::*;
#[cfg(feature = "layout")]
pub use layout::*;
#[cfg(feature = "media")]
pub use media::*;
#[cfg(feature = "menus")]
pub use menus::*;
#[cfg(feature = "navigation")]
pub use navigation::*;
#[cfg(feature = "overlay")]
pub use overlay::*;

/// Adds the CSS of every compiled group to the page. Call once, before
/// mounting; a second call adds nothing.
pub fn init() {
    for (id, sheet) in css::sheets() {
        css::inject(id, sheet);
    }
}
