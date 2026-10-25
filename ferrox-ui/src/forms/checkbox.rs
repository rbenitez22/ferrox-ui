use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::core::{component_help, reveals_while_active, with_reveal, HelpReveal};
use crate::ids::unique_id;
use crate::MaybeHelp;

/// The class list for a `.checkbox`.
pub fn checkbox_classes(invalid: bool, extra: &str) -> String {
    let mut classes = vec!["checkbox"];
    if invalid { classes.push("is-invalid"); }
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

/// A checkbox and its words, as one `<label>` (the words are part of the
/// target), bound two-way to `checked`, with at most one message under it.
///
/// A checkbox states an intention that a later Save carries out. A setting
/// that takes effect the moment it flips is a [`Switch`](crate::Switch).
/// It carries its own label, so it does not go in a `Field`; `hint` and
/// `error` work as there (never both).
///
/// `help` puts a `?` after the words, outside the `<label>`; a help request
/// says whether it is ticked only with `help_shares_value`.
///
/// ```ignore
/// <Checkbox checked=recurring label="Repeat every month" />
/// <Checkbox checked=agreed label="I accept the terms" error=terms_error />
/// ```
#[component]
pub fn Checkbox(
    checked: RwSignal<bool>,
    #[prop(into)] label: Signal<String>,
    #[prop(optional, into)] hint: MaybeProp<String>,
    /// The problem; present (and not empty) means invalid.
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// A help topic: a `?` after the words.
    #[prop(optional, into)] help: MaybeHelp,
    #[prop(optional)] help_skip_tab: bool,
    /// When the `?` shows (unset: as its `HelpScope` says, else always).
    #[prop(optional)] help_reveal: Option<HelpReveal>,
    /// Send whether it is ticked ("true" / "false") with a help request.
    #[prop(optional)] help_shares_value: bool,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let id = unique_id("ferrox-checkbox");
    let message_id = format!("{id}-message");

    let error = Signal::derive(move || error.get().filter(|e| !e.is_empty()));
    let hint = Signal::derive(move || hint.get().filter(|h| !h.is_empty()));
    let invalid = Signal::derive(move || error.get().is_some());
    let described_by = {
        let message_id = message_id.clone();
        move || (error.get().is_some() || hint.get().is_some()).then(|| message_id.clone())
    };
    let message = move || match (error.get(), hint.get()) {
        (Some(text), _) => Some(view! { <span class="field__error" id=message_id.clone()>{text}</span> }.into_any()),
        (None, Some(text)) => Some(view! { <span class="field__hint" id=message_id.clone()>{text}</span> }.into_any()),
        (None, None) => None,
    };

    let shared = Signal::derive(move || help_shares_value.then(|| checked.get().to_string()));
    let reveal = help.0.is_some() && reveals_while_active(help_reveal);
    let (help, help_id) = component_help(help, "checkbox", label, shared, help_skip_tab, "checkbox__help", reveal).unzip();

    let row = view! {
        <label class="checkbox__row">
            <input
                type="checkbox"
                class="checkbox__input"
                id=id
                aria-describedby=described_by
                aria-invalid=move || invalid.get().then_some("true")
                name=move || name.get()
                required=required
                disabled=move || disabled.get()
                bind:checked=checked
            />
            <span class="checkbox__label">{move || label.get()}</span>
        </label>
    };
    // With help, the `?` follows the words, outside the `<label>`: a button
    // inside one would also tick the box.
    let line = match help {
        Some(help) => view! { <div class="checkbox__line">{row}{help}</div> }.into_any(),
        None => row.into_any(),
    };

    view! {
        <div class=move || with_reveal(checkbox_classes(invalid.get(), &class.get().unwrap_or_default()), reveal) data-help=help_id>
            {line}
            {message}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(checkbox_classes(false, ""), "checkbox");
        assert_eq!(checkbox_classes(true, "x"), "checkbox is-invalid x");
    }
}
