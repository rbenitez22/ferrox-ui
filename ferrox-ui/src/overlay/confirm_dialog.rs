use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::dialog::{Dialog, DialogAction};
use crate::{ButtonFamily, MaybeHelp};

/// The standard destructive confirmation (`role="alertdialog"`).
///
/// Every destructive action opens one: no "undo toast" instead. The confirm
/// button is always in the danger family and is the last, heaviest button.
///
/// - `confirm_label` is the verb, not "OK": Delete, Void, Archive. A footer
///   is read at a glance.
/// - The body names exactly what is about to be lost.
/// - `on_confirm` runs the action; set `saving` while the request runs (the
///   button sweeps and refuses clicks) and close the dialog when it is done.
///   Cancel, the × and Esc call `on_cancel`.
///
/// ```ignore
/// <ConfirmDialog
///     open=confirming
///     title="Delete account?"
///     confirm_label="Delete"
///     saving=deleting
///     on_confirm=move || delete_account()
///     on_cancel=move || confirming.set(false)
/// >
///     <p>"This deletes “Groceries” and its 214 transactions."</p>
/// </ConfirmDialog>
/// ```
#[component]
pub fn ConfirmDialog(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] confirm_label: Signal<String>,
    #[prop(into)] on_confirm: UnsyncCallback<()>,
    #[prop(into)] on_cancel: UnsyncCallback<()>,
    #[prop(optional, into)] saving: Signal<bool>,
    /// The dismissive button's label.
    #[prop(into, default = "Cancel".into())] cancel_label: Signal<String>,
    /// The × button's label.
    #[prop(into, default = "Close".to_string())] close_label: String,
    /// A help topic: a `?` in the title bar, before the ×.
    #[prop(optional, into)] help: MaybeHelp,
    children: ChildrenFn,
) -> impl IntoView {
    let confirm = DialogAction {
        label: confirm_label,
        on_click: on_confirm,
        family: ButtonFamily::Danger,
        saving,
        disabled: Signal::stored(false),
    };
    view! {
        <Dialog
            open=open
            title=title
            on_close=on_cancel
            dismiss_label=cancel_label
            close_label=close_label
            alert=true
            help=help
            affirm=confirm
        >
            {children()}
        </Dialog>
    }
}
