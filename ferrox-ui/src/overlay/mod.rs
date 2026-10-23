//! The `overlay` group (feature `overlay`): modals on the native `<dialog>`
//! element, opened with `showModal()`.

mod dialog;
mod modal;

pub use dialog::{dialog_classes, Dialog, DialogAction, DialogSize};
