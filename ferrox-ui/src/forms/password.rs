use ferrox_icons::{Icon, EYE, EYE_OFF};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::field::{control_classes, wiring};
use super::strength::{password_strength, StrengthLabels};
use crate::ids::unique_id;
use crate::IconButton;

/// A password field (`.form-input` in a `.password-input`), bound two-way to
/// `value`, with a button that shows and hides what was typed.
///
/// - The button is a real `<button>` (`aria-pressed` while shown), labelled
///   by `show_label`; the eye crosses out while the password is visible.
/// - `autocomplete`: `"current-password"` (default) to sign in,
///   `"new-password"` to set one, so password managers offer the right thing.
/// - `strength` adds a meter under the field while it has text: a bar and a
///   word (`strength_labels`), against `min_bits` for "strong". The field is
///   described by it.
///
/// ```ignore
/// <Field label="Password"><PasswordInput value=password /></Field>
/// <Field label="New password">
///     <PasswordInput value=new_password autocomplete="new-password" strength=true />
/// </Field>
/// ```
#[component]
pub fn PasswordInput(
    value: RwSignal<String>,
    #[prop(into, default = "current-password".to_string())] autocomplete: String,
    #[prop(optional)] strength: bool,
    /// The entropy, in bits, that counts as strong.
    #[prop(default = 80.0)] min_bits: f64,
    #[prop(optional)] strength_labels: StrengthLabels,
    /// The show/hide button's label.
    #[prop(into, default = "Show password".into())] show_label: Signal<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] autofocus: bool,
    /// Extra classes on the input. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let wired = wiring("ferrox-password");
    let invalid = wired.invalid;
    let shown = RwSignal::new(false);

    let meter_id = unique_id("ferrox-password-strength");
    let meter_shown = Signal::derive(move || strength && !value.get().is_empty());
    let described_by = {
        let meter_id = meter_id.clone();
        let field = wired.described_by;
        move || {
            let ids: Vec<String> = field.get().into_iter()
                .chain(meter_shown.get().then(|| meter_id.clone()))
                .collect();
            (!ids.is_empty()).then(|| ids.join(" "))
        }
    };

    let meter = move || meter_shown.get().then(|| {
        let measured = password_strength(&value.get(), min_bits);
        view! {
            <div class=format!("password-strength {}", measured.level.class()) id=meter_id.clone() data-bits=format!("{:.0}", measured.bits)>
                <span class="password-strength__bar" aria-hidden="true"><span class="password-strength__fill"></span></span>
                <span class="password-strength__label">{strength_labels.for_level(measured.level).to_string()}</span>
            </div>
        }
    });

    view! {
        <div class="password-input">
            <input
                type=move || if shown.get() { "text" } else { "password" }
                id=wired.id
                class=move || control_classes("form-input", invalid.get(), &class.get().unwrap_or_default())
                aria-describedby=described_by
                aria-invalid=move || invalid.get().then_some("true")
                autocomplete=autocomplete
                placeholder=move || placeholder.get()
                name=move || name.get()
                required=required
                disabled=move || disabled.get()
                autofocus=autofocus
                spellcheck="false"
                bind:value=value
            />
            <IconButton
                label=show_label
                class="password-input__toggle"
                pressed=shown
                disabled=disabled
                on_click=move |_| shown.update(|s| *s = !*s)
            >
                <Icon icon=move || if shown.get() { EYE_OFF } else { EYE } />
            </IconButton>
        </div>
        {meter}
    }
}
