use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::field::{control_classes, wiring};

/// A multi-line text field (`.form-textarea`), bound two-way to `value`.
/// It grows by hand (resize: vertical), and Enter is a newline, never a
/// submit (`Dialog` leaves it alone).
///
/// ```ignore
/// <Field label="Notes"><Textarea value=notes rows=6 /></Field>
/// ```
#[component]
pub fn Textarea(
    value: RwSignal<String>,
    #[prop(default = 4)] rows: u32,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] readonly: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] autofocus: bool,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let wired = wiring("ferrox-textarea");
    let invalid = wired.invalid;
    view! {
        <textarea
            id=wired.id
            class=move || control_classes("form-textarea", invalid.get(), &class.get().unwrap_or_default())
            aria-describedby=move || wired.described_by.get()
            aria-invalid=move || invalid.get().then_some("true")
            rows=rows
            placeholder=move || placeholder.get()
            name=move || name.get()
            required=required
            readonly=move || readonly.get()
            disabled=move || disabled.get()
            autofocus=autofocus
            bind:value=value
        ></textarea>
    }
}
