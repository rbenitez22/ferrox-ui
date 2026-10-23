use std::sync::Arc;

use leptos::context::Provider;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::ids::unique_id;

/// What a [`Field`] hands its control: the id its label points at, the id
/// of its message, and whether it is invalid. Controls read it with
/// `use_context`; a newtype, so no other context of a plain type collides.
#[derive(Clone)]
pub(crate) struct FieldContext {
    id: Arc<str>,
    message_id: Arc<str>,
    has_message: Signal<bool>,
    invalid: Signal<bool>,
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
    let context = FieldContext { id: id.clone(), message_id: message_id.clone(), has_message, invalid };

    let message = move || match (error.get(), hint.get()) {
        (Some(text), _) => Some(view! { <span class="field__error" id=message_id.to_string()>{text}</span> }.into_any()),
        (None, Some(text)) => Some(view! { <span class="field__hint" id=message_id.to_string()>{text}</span> }.into_any()),
        (None, None) => None,
    };

    view! {
        <div class=move || field_classes(&class.get().unwrap_or_default())>
            <label class="field__label" for=id.to_string()>{move || label.get()}</label>
            <Provider value=context>{children()}</Provider>
            {message}
        </div>
    }
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
