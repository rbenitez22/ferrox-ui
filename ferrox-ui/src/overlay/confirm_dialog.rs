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
///   button sweeps and refuses clicks) and set `open` false when it is done.
///   Cancel, the × and Esc close it themselves, then call `on_cancel`, if
///   given.
///
/// ```ignore
/// <ConfirmDialog
///     open=confirming
///     title="Delete account?"
///     confirm_label="Delete"
///     saving=deleting
///     on_confirm=move || delete_account()
/// >
///     <p>"This deletes “Groceries” and its 214 transactions."</p>
/// </ConfirmDialog>
/// ```
#[component]
pub fn ConfirmDialog(
    open: RwSignal<bool>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] confirm_label: Signal<String>,
    #[prop(into)] on_confirm: UnsyncCallback<()>,
    /// Told after a dismissive close (Cancel, the ×, Esc).
    #[prop(optional, into)] on_cancel: Option<UnsyncCallback<()>>,
    #[prop(optional, into)] saving: Signal<bool>,
    /// The Cancel button's label.
    #[prop(into, default = "Cancel".into())] cancel_label: Signal<String>,
    /// The × button's label.
    #[prop(into, default = "Close".into())] close_label: Signal<String>,
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
    let on_cancel = UnsyncCallback::new(move |_| {
        if let Some(on_cancel) = on_cancel {
            on_cancel.run(());
        }
    });
    view! {
        <Dialog
            open=open
            title=title
            on_close=on_cancel
            cancel_label=cancel_label
            close_label=close_label
            alert=true
            help=help
            affirm=confirm
        >
            {children()}
        </Dialog>
    }
}
