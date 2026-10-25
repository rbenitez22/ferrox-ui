use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::core::{component_help, reveals_while_active, with_reveal, HelpReveal};
use crate::MaybeHelp;

/// The class list for a `.switch`.
pub fn switch_classes(extra: &str) -> String {
    if extra.is_empty() { "switch".to_string() } else { format!("switch {extra}") }
}

/// A setting said as a switch: on or off, taking effect as it is flipped
/// (`<button role="switch" aria-checked>`), bound two-way to `checked`.
///
/// **A switch, not a checkbox.** A checkbox states an intention that a later
/// Save sends; a switch *is* the act. Use it for a filter over what a screen
/// shows, or a preference written straight back; a field a dialog's Save
/// sends stays a [`Checkbox`](crate::Checkbox).
///
/// The words are inside the button: they name it for a screen reader and
/// are part of the target, with no `<label>` to keep in step. `help` puts a
/// `?` beside the button (not in it: a button in a button).
///
/// ```ignore
/// <Switch checked=show_archived label="Show archived accounts" />
/// ```
#[component]
pub fn Switch(
    checked: RwSignal<bool>,
    #[prop(into)] label: Signal<String>,
    /// Off and unusable: a setting still loading, or one this user may not change.
    #[prop(optional, into)] disabled: Signal<bool>,
    /// A help topic: a `?` beside the switch.
    #[prop(optional, into)] help: MaybeHelp,
    #[prop(optional)] help_skip_tab: bool,
    /// When the `?` shows (unset: as its `HelpScope` says, else always).
    #[prop(optional)] help_reveal: Option<HelpReveal>,
    /// Extra classes, appended to `switch`. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let reveal = help.0.is_some() && reveals_while_active(help_reveal);
    let help = component_help(help, "switch", label, Signal::stored(None), help_skip_tab, "switch__help", reveal);
    let button = view! {
        <button
            type="button"
            role="switch"
            class=move || switch_classes(&class.get().unwrap_or_default())
            aria-checked=move || if checked.get() { "true" } else { "false" }
            disabled=move || disabled.get()
            on:click=move |_| checked.update(|on| *on = !*on)
        >
            <span class="switch__track" aria-hidden="true"><span class="switch__knob"></span></span>
            {move || label.get()}
        </button>
    };
    match help {
        Some(help) => view! { <span class=with_reveal("switch-line".to_string(), reveal)>{button}{help}</span> }.into_any(),
        None => button.into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(switch_classes(""), "switch");
        assert_eq!(switch_classes("x"), "switch x");
    }
}
