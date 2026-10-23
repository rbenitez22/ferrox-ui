use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::field::{control_classes, offer_value, wiring};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum InputKind {
    #[default]
    Text,
    Email,
    Search,
    Tel,
    Url,
    Number,
}

impl InputKind {
    /// The `type` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text   => "text",
            Self::Email  => "email",
            Self::Search => "search",
            Self::Tel    => "tel",
            Self::Url    => "url",
            Self::Number => "number",
        }
    }
}

/// A one-line text field (`.form-input`), bound two-way to `value`.
///
/// Inside a `Field` it takes the field's id, description and invalid state;
/// outside one it gets an id of its own. Passwords are `PasswordInput`.
///
/// ```ignore
/// <Field label="Name"><Input value=name autofocus=true /></Field>
/// <Input value=query kind=InputKind::Search placeholder="Search accounts" />
/// ```
#[component]
pub fn Input(
    value: RwSignal<String>,
    #[prop(optional)] kind: InputKind,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    /// The `autocomplete` hint (`"email"`, `"off"`, …).
    #[prop(optional, into)] autocomplete: MaybeProp<String>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] readonly: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Where a `Dialog` puts focus when it opens.
    #[prop(optional)] autofocus: bool,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let wired = wiring("ferrox-input");
    let invalid = wired.invalid;
    offer_value(value.into());
    view! {
        <input
            type=kind.as_str()
            id=wired.id
            class=move || control_classes("form-input", invalid.get(), &class.get().unwrap_or_default())
            aria-describedby=move || wired.described_by.get()
            aria-invalid=move || invalid.get().then_some("true")
            placeholder=move || placeholder.get()
            name=move || name.get()
            autocomplete=move || autocomplete.get()
            required=required
            readonly=move || readonly.get()
            disabled=move || disabled.get()
            autofocus=autofocus
            bind:value=value
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_are_input_types() {
        assert_eq!(InputKind::default().as_str(), "text");
        assert_eq!(InputKind::Email.as_str(), "email");
        assert_eq!(InputKind::Number.as_str(), "number");
    }
}
