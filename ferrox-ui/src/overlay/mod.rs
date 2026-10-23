//! The `overlay` group (feature `overlay`): modals on the native `<dialog>`
//! element, opened with `showModal()`.

mod confirm_dialog;
mod dialog;
mod modal;
mod sheet;

pub use confirm_dialog::ConfirmDialog;
pub use dialog::{dialog_classes, Dialog, DialogAction, DialogSize};
pub use sheet::{sheet_classes, Sheet, SheetSide};
