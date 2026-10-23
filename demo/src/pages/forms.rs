//! Forms: fields and their controls, each with its value echoed below, so
//! labels, messages, invalid states and two-way binding can be checked.

use ferrox_ui::{Button, ButtonFamily, Field, Input, InputKind, Textarea};
use leptos::prelude::*;

use crate::section::Section;

#[component]
pub fn FormsPage() -> impl IntoView {
    view! {
        <Fields />
    }
}

/// A hint, a live error, a textarea, and an input outside any field.
#[component]
fn Fields() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let notes = RwSignal::new(String::new());
    let query = RwSignal::new(String::new());
    let email_error = Signal::derive(move || {
        let email = email.get();
        (!email.is_empty() && !email.contains('@')).then(|| "An email address has an @ in it.".to_string())
    });
    view! {
        <Section title="Fields">
            <div class="demo-form">
                <Field label="Name" hint="As it appears on statements">
                    <Input value=name name="name" />
                </Field>
                <Field label="Email" hint="Where receipts go" error=email_error>
                    <Input value=email kind=InputKind::Email name="email" autocomplete="email" />
                </Field>
                <Field label="Notes">
                    <Textarea value=notes name="notes" placeholder="Anything to remember" />
                </Field>
                <Input value=query kind=InputKind::Search name="query" placeholder="Search (no field)" />
            </div>
            <div class="demo-row">
                <Button family=ButtonFamily::Secondary on_click=move |_| { name.set("Groceries".into()); notes.set("Line one\nLine two".into()); }>
                    "Fill from code"
                </Button>
            </div>
            <dl class="demo-echo">
                <dt>"name"</dt><dd data-echo="name">{move || name.get()}</dd>
                <dt>"email"</dt><dd data-echo="email">{move || email.get()}</dd>
                <dt>"notes"</dt><dd data-echo="notes">{move || notes.get()}</dd>
                <dt>"query"</dt><dd data-echo="query">{move || query.get()}</dd>
            </dl>
        </Section>
    }
}
