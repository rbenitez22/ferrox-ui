use ferrox_icons::{Icon, CLOSE};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{Element, KeyboardEvent};

use super::modal::Modal;
use crate::ids::unique_id;
use crate::core::component_help;
use crate::{Button, ButtonFamily, IconButton, MaybeHelp};

// ---------------------------------------------------------------------------
// DialogSize / DialogAction
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum DialogSize {
    /// 32rem: forms and messages.
    #[default]
    Md,
    /// 52rem: a body that is a table of its own.
    Wide,
}

impl DialogSize {
    pub fn class(self) -> Option<&'static str> {
        match self {
            Self::Md => None,
            Self::Wide => Some("dialog--wide"),
        }
    }
}

/// One footer button of a [`Dialog`]: its label, what it does, and its state.
///
/// ```ignore
/// DialogAction::new("Save", move || save()).saving(saving)
/// DialogAction::new("Delete", move || delete()).danger()
/// ```
#[derive(Clone)]
#[non_exhaustive]
pub struct DialogAction {
    pub label: Signal<String>,
    pub on_click: UnsyncCallback<()>,
    pub family: ButtonFamily,
    pub saving: Signal<bool>,
    pub disabled: Signal<bool>,
}

impl DialogAction {
    pub fn new(label: impl Into<Signal<String>>, on_click: impl Fn() + 'static) -> Self {
        Self {
            label: label.into(),
            on_click: UnsyncCallback::new(move |()| on_click()),
            family: ButtonFamily::Primary,
            saving: Signal::stored(false),
            disabled: Signal::stored(false),
        }
    }

    /// A destructive commit: the affirm button in the danger family. (The
    /// alternate is always secondary.)
    pub fn danger(mut self) -> Self {
        self.family = ButtonFamily::Danger;
        self
    }

    /// A request is running: the button sweeps and refuses clicks, and Enter
    /// does not fire it.
    pub fn saving(mut self, saving: impl Into<Signal<bool>>) -> Self {
        self.saving = saving.into();
        self
    }

    pub fn disabled(mut self, disabled: impl Into<Signal<bool>>) -> Self {
        self.disabled = disabled.into();
        self
    }

    /// Whether the action can run now (Enter checks this).
    fn ready(&self) -> bool {
        !self.saving.get_untracked() && !self.disabled.get_untracked()
    }

    fn button(self, family: ButtonFamily) -> impl IntoView {
        let Self { label, on_click, saving, disabled, .. } = self;
        view! {
            <Button family=family saving=saving disabled=disabled on_click=move |_| on_click.run(())>
                {move || label.get()}
            </Button>
        }
    }
}

/// The class list for a `.dialog`.
pub fn dialog_classes(size: DialogSize, extra: &str) -> String {
    let mut classes = vec!["dialog"];
    if let Some(size) = size.class() { classes.push(size); }
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

// ---------------------------------------------------------------------------
// Dialog
// ---------------------------------------------------------------------------

/// A modal dialog: a title bar with a close button, the body, and a footer
/// band. It is the native `<dialog>` opened with `showModal()`, so the page
/// behind is inert and focus stays inside.
///
/// - `open` drives it. Esc, the × and the dismiss button all call `on_close`,
///   which should set `open` to false; they are the same act.
/// - **The footer is not a slot.** Pass `affirm` (and `alternate`); the order
///   is fixed: dismiss, alternate, affirm — the heaviest button last.
/// - Enter fires `affirm` (`submit_on_enter`), except from a textarea, a
///   button or a link, and not while it is saving or disabled.
/// - The body is mounted only while open, so each opening starts fresh. Mark
///   a form's first field `autofocus`; otherwise focus goes to the close
///   button. On close, focus returns to what opened the dialog.
///
/// ```ignore
/// <Dialog
///     open=editing
///     title="Edit account"
///     on_close=move || editing.set(false)
///     affirm=DialogAction::new("Save", move || save()).saving(saving)
/// >
///     <input autofocus=true … />
/// </Dialog>
/// ```
#[component]
pub fn Dialog(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] on_close: UnsyncCallback<()>,
    /// The dismissive button's label.
    #[prop(into, default = "Cancel".into())] dismiss_label: Signal<String>,
    /// No dismiss button. The × and Esc still close.
    #[prop(optional)] no_dismiss: bool,
    #[prop(optional)] affirm: Option<DialogAction>,
    /// A lighter way to finish, before `affirm`; always secondary.
    #[prop(optional)] alternate: Option<DialogAction>,
    #[prop(default = true)] submit_on_enter: bool,
    #[prop(optional)] size: DialogSize,
    /// `role="alertdialog"`: a dialog that interrupts to ask for a decision
    /// (`ConfirmDialog` sets it).
    #[prop(optional)] alert: bool,
    /// A help topic: a `?` in the title bar, before the ×.
    #[prop(optional, into)] help: MaybeHelp,
    /// The × button's label.
    #[prop(into, default = "Close".to_string())] close_label: String,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let modal = Modal::new(open, on_close);
    let (on_cancel, on_closed) = (modal.on_cancel(), modal.on_closed());
    let title_id = unique_id("ferrox-dialog-title");
    let labelled_by = title_id.clone();

    let enter_action = affirm.clone().filter(|_| submit_on_enter);
    let on_keydown = move |ev: KeyboardEvent| {
        let Some(action) = &enter_action else { return };
        if ev.key() == "Enter" && action.ready() && !takes_enter(&ev) {
            ev.prevent_default();
            action.on_click.run(());
        }
    };

    let kind = if alert { "alertdialog" } else { "dialog" };
    let help = component_help(help, kind, title, Signal::stored(None), false, "dialog__help");

    let has_footer = !no_dismiss || alternate.is_some() || affirm.is_some();
    let footer = has_footer.then(|| view! {
        <div class="dialog__footer">
            {(!no_dismiss).then(|| view! {
                <Button family=ButtonFamily::Cancel on_click=move |_| on_close.run(())>
                    {move || dismiss_label.get()}
                </Button>
            })}
            {alternate.map(|action| action.button(ButtonFamily::Secondary))}
            {affirm.map(|action| { let family = action.family; action.button(family) })}
        </div>
    });

    view! {
        <dialog
            node_ref=modal.node_ref
            class=move || dialog_classes(size, &class.get().unwrap_or_default())
            role=alert.then_some("alertdialog")
            aria-labelledby=labelled_by
            on:cancel=on_cancel
            on:close=on_closed
            on:keydown=on_keydown
        >
            <div class="dialog__title-bar">
                <h2 class="dialog__title" id=title_id>{move || title.get()}</h2>
                {help}
                <IconButton title=close_label class="dialog__close" on_click=move |_| on_close.run(())>
                    <Icon icon=CLOSE />
                </IconButton>
            </div>
            <Show when=move || open.get()>
                <div class="dialog__body">{children()}</div>
            </Show>
            {footer}
        </dialog>
    }
}

/// Whether the key's target uses Enter itself: a newline in a textarea, a
/// press on a focused button or link.
fn takes_enter(ev: &KeyboardEvent) -> bool {
    ev.target()
        .and_then(|t| t.dyn_into::<Element>().ok())
        .is_some_and(|el| matches!(el.tag_name().as_str(), "TEXTAREA" | "BUTTON" | "A"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(dialog_classes(DialogSize::default(), ""), "dialog");
        assert_eq!(dialog_classes(DialogSize::Wide, "extra"), "dialog dialog--wide extra");
    }

    #[test]
    fn an_action_is_primary_and_ready_until_told_otherwise() {
        let owner = Owner::new();
        owner.with(|| {
            let action = DialogAction::new("Save", || {});
            assert_eq!(action.family, ButtonFamily::Primary);
            assert!(action.ready());
            assert_eq!(action.clone().danger().family, ButtonFamily::Danger);
            assert!(!action.clone().saving(true).ready());
            assert!(!action.disabled(true).ready());
        });
    }
}
