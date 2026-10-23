//! The `overlay` group (feature `overlay`): modals on the native `<dialog>`
//! element, opened with `showModal()`.

mod confirm_dialog;
mod dialog;
mod modal;

pub use confirm_dialog::ConfirmDialog;
pub use dialog::{dialog_classes, Dialog, DialogAction, DialogSize};
