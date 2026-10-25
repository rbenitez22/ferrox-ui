use std::sync::Arc;

use leptos::context::Provider;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use web_sys::{Event, FocusEvent};

use crate::core::{component_help, reveals_while_active, with_reveal, HelpReveal};
use crate::{help_signals, HelpSignalKind};
use crate::ids::unique_id;
use crate::MaybeHelp;

/// What a [`Field`] hands its control: the id its label points at, the id
/// of its message, and whether it is invalid. Controls read it with
/// `use_context`; a newtype, so no other context of a plain type collides.
#[derive(Clone)]
pub(crate) struct FieldContext {
    id: Arc<str>,
    message_id: Arc<str>,
    has_message: Signal<bool>,
    invalid: Signal<bool>,
    /// The control's value, for a help request that shares it. Filled by
    /// controls that offer theirs (`PasswordInput` never does).
    value: StoredValue<Option<Signal<String>>>,
    /// The field's label and its help topic, for a control's help signals.
    label: Signal<String>,
    topic_id: Option<Arc<str>>,
}

/// What a control inside a `Field` reports a help signal about: the
/// field's label and topic. Outside a field, nothing to name it by.
pub(crate) fn signal_about() -> (String, Option<String>) {
    use_context::<FieldContext>()
        .map(|field| (field.label.get_untracked(), field.topic_id.as_deref().map(str::to_string)))
        .unwrap_or_default()
}

/// Offers a control's value to its `Field`, for help requests that share
/// it. Does nothing outside a field.
pub(crate) fn offer_value(value: Signal<String>) {
    if let Some(field) = use_context::<FieldContext>() {
        field.value.set_value(Some(value));
    }
}

/// How a control is wired: its id, what describes it, whether it is invalid.
/// Inside a `Field`, the field's; outside one, a fresh id and nothing else.
pub(crate) struct Wiring {
    pub id: String,
    pub described_by: Signal<Option<String>>,
    pub invalid: Signal<bool>,
}

/// The wiring for a control, from the enclosing `Field` if there is one.
pub(crate) fn wiring(prefix: &str) -> Wiring {
    match use_context::<FieldContext>() {
        Some(field) => {
            let message_id = field.message_id.clone();
            let has_message = field.has_message;
            Wiring {
                id: field.id.to_string(),
                described_by: Signal::derive(move || has_message.get().then(|| message_id.to_string())),
                invalid: field.invalid,
            }
        }
        None => Wiring {
            id: unique_id(prefix),
            described_by: Signal::stored(None),
            invalid: Signal::stored(false),
        },
    }
}

/// The class list for a `.field`.
pub fn field_classes(extra: &str) -> String {
    if extra.is_empty() { "field".to_string() } else { format!("field {extra}") }
}

/// A label, one control, and at most one message under it.
///
/// - **A problem this field can fix goes here.** Anything cross-field goes in
///   an `Alert` at the top of the form instead; never both for one problem.
/// - `hint` is guidance while the field is fine; `error` is what is wrong.
///   Never both: while there is an error, the hint is hidden.
/// - The control inside (an `Input`, `Textarea`, …) picks up the field's id
///   (so the label names it), `aria-describedby` (the message) and, while
///   there is an error, `aria-invalid` and `.is-invalid`. One control per
///   field. Only the field's own children see this; a control after it gets
///   an id of its own.
/// - `help` puts a `?` after the label (outside the `<label>`). A help
///   request names the field by its label; it carries the control's value
///   only with `help_shares_value` (a password's, never). `help_skip_tab`
///   keeps the `?` out of the Tab order, for a field in a row;
///   `help_reveal` shows it only while the field is in use.
///
/// ```ignore
/// <Field label="Email" hint="Where receipts go" error=email_error>
///     <Input value=email kind=InputKind::Email />
/// </Field>
/// ```
#[component]
pub fn Field(
    #[prop(into)] label: Signal<String>,
    #[prop(optional, into)] hint: MaybeProp<String>,
    /// The field's problem; present (and not empty) means invalid.
    #[prop(optional, into)] error: MaybeProp<String>,
    /// A help topic: a `?` after the label.
    #[prop(optional, into)] help: MaybeHelp,
    #[prop(optional)] help_skip_tab: bool,
    /// When the `?` shows (unset: as its `HelpScope` says, else always).
    #[prop(optional)] help_reveal: Option<HelpReveal>,
    /// Send the control's value with a help request.
    #[prop(optional)] help_shares_value: bool,
    /// Extra classes, appended to `field`. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let id: Arc<str> = unique_id("ferrox-field").into();
    let message_id: Arc<str> = format!("{id}-message").into();

    let error = Signal::derive(move || error.get().filter(|e| !e.is_empty()));
    let hint = Signal::derive(move || hint.get().filter(|h| !h.is_empty()));
    let invalid = Signal::derive(move || error.get().is_some());
    let has_message = Signal::derive(move || error.get().is_some() || hint.get().is_some());

    // Through `Provider`, which gives the children an owner of their own: a
    // component does not, so a plain `provide_context` here would reach the
    // controls *after* this field too.
    let value = StoredValue::new(None::<Signal<String>>);
    let topic_id: Option<Arc<str>> = help.0.as_ref().map(|topic| topic.id().into());
    let context = FieldContext {
        id: id.clone(),
        message_id: message_id.clone(),
        has_message,
        invalid,
        value,
        label,
        topic_id: topic_id.clone(),
    };
    let (on_focus, on_change) = error_signals(error, label, topic_id);

    let shared = Signal::derive(move || {
        if help_shares_value { value.get_value().map(|v| v.get()) } else { None }
    });
    let reveal = help.0.is_some() && reveals_while_active(help_reveal);
    let (help, help_id) = component_help(help, "field", label, shared, help_skip_tab, "field__help", reveal).unzip();
    let label_tag = view! { <label class="field__label" for=id.to_string()>{move || label.get()}</label> };
    // With help, the label and its `?` share a row; the `?` stays outside the
    // `<label>` (a button inside one is a control inside a control).
    let label_row = match help {
        Some(help) => view! { <div class="field__label-row">{label_tag}{help}</div> }.into_any(),
        None => label_tag.into_any(),
    };

    let message = move || match (error.get(), hint.get()) {
        (Some(text), _) => Some(view! { <span class="field__error" id=message_id.to_string()>{text}</span> }.into_any()),
        (None, Some(text)) => Some(view! { <span class="field__hint" id=message_id.to_string()>{text}</span> }.into_any()),
        (None, None) => None,
    };

    view! {
        <div class=move || with_reveal(field_classes(&class.get().unwrap_or_default()), reveal) data-help=help_id on:focusin=on_focus on:change=on_change>
            {label_row}
            <Provider value=context>{children()}</Provider>
            {message}
        </div>
    }
}

/// Inside a `HelpSignals`: reports the error appearing and going away (an
/// effect), and each committed edit that leaves it there: the returned
/// `focusin` and `change` handlers, on the field's container, so they need
/// no control's value. An edit counts only if the error was there before it
/// (when the field got focus, or at the last change): the edit that makes
/// an error isn't one that failed to fix it.
fn error_signals(
    error: Signal<Option<String>>,
    label: Signal<String>,
    topic_id: Option<Arc<str>>,
) -> (impl Fn(FocusEvent) + 'static, impl Fn(Event) + 'static) {
    let signals = help_signals();
    let edits = StoredValue::new(0_u32);
    let had_error = StoredValue::new(false);
    let topic = move || topic_id.as_deref().map(str::to_string);
    let report = move |kind: HelpSignalKind| signals.emit(kind, "field", label.get_untracked(), topic(), None);
    if signals.is_observed() {
        let report = report.clone();
        // Transitions only: an error already there when the field appears
        // isn't something the user did.
        Effect::new(move |was: Option<bool>| {
            let is = error.with(Option::is_some);
            match (was, is) {
                (Some(false), true) => {
                    edits.set_value(0);
                    report(HelpSignalKind::ErrorShown);
                }
                (Some(true), false) => report(HelpSignalKind::ErrorCleared),
                _ => {}
            }
            is
        });
    }
    let on_focus = move |_: FocusEvent| had_error.set_value(error.with_untracked(Option::is_some));
    let on_change = move |_: Event| {
        let has_error = error.with_untracked(Option::is_some);
        if signals.is_observed() && has_error && had_error.get_value() {
            edits.update_value(|n| *n += 1);
            report(HelpSignalKind::ErrorPersisted { edits: edits.get_value() });
        }
        had_error.set_value(has_error);
    };
    (on_focus, on_change)
}

/// The class list for a form control: its base class, `.is-invalid` while
/// invalid, then any extra classes.
pub(crate) fn control_classes(base: &str, invalid: bool, extra: &str) -> String {
    let mut classes = vec![base];
    if invalid { classes.push("is-invalid"); }
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(field_classes(""), "field");
        assert_eq!(field_classes("wide"), "field wide");
        assert_eq!(control_classes("form-input", false, ""), "form-input");
        assert_eq!(control_classes("form-input", true, "x"), "form-input is-invalid x");
    }

    #[test]
    fn outside_a_field_a_control_gets_its_own_id_and_no_description() {
        Owner::new().with(|| {
            let a = wiring("ferrox-input");
            let b = wiring("ferrox-input");
            assert_ne!(a.id, b.id);
            assert_eq!(a.described_by.get_untracked(), None);
            assert!(!a.invalid.get_untracked());
        });
    }
}
