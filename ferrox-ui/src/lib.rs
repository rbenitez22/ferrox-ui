//! UI primitives for Leptos CSR apps, styled by class-only CSS on
//! ferrox-theme's tokens, so every theme reaches every component.
//!
//! Components come in feature groups. `core` is always compiled:
//! [`Button`], [`IconButton`], [`Toolbar`] / [`ToolbarGroup`], [`Spinner`],
//! [`LoadingOverlay`], [`Alert`], [`HelpButton`] / [`HelpTopic`]. Feature
//! `overlay`: `Dialog`, `ConfirmDialog`, `Sheet`. Feature `forms`: `Field`,
//! `Input`, `Textarea`, `PasswordInput`, `Checkbox`, `Switch`, `Select`,
//! `Combobox`.
//! Further groups (`menus`, `navigation`, `layout`, `data`) arrive as they
//! are built.
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
#[cfg(feature = "forms")]
mod forms;
mod ids;
#[cfg(feature = "overlay")]
mod overlay;

pub use core::*;
#[cfg(feature = "forms")]
pub use forms::*;
#[cfg(feature = "overlay")]
pub use overlay::*;

/// Adds the CSS of every compiled group to the page. Call once, before
/// mounting; a second call adds nothing.
pub fn init() {
    for (id, sheet) in css::sheets() {
        css::inject(id, sheet);
    }
}
