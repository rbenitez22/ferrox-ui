use leptos::prelude::*;
use leptos::{component, view, IntoView};
use web_sys::MouseEvent;

// ---------------------------------------------------------------------------
// ButtonFamily / ButtonSize
//
// What the button means, not how heavy it looks. Weight is `quiet`, a separate
// axis (`.btn--quiet`): an outline secondary button is a secondary
// action rendered quietly, not a family of its own.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ButtonFamily {
    /// Affirmative. The main thing a screen exists for.
    #[default]
    Primary,
    /// A real, committing action that is not the primary one.
    Secondary,
    /// Negative / dismissive. Always an outline.
    Cancel,
    /// Destructive. The affirmative button of a destructive confirmation.
    Danger,
}

impl ButtonFamily {
    pub fn class(self) -> &'static str {
        match self {
            Self::Primary   => "btn--primary",
            Self::Secondary => "btn--secondary",
            Self::Cancel    => "btn--cancel",
            Self::Danger    => "btn--danger",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ButtonSize {
    pub fn class(self) -> Option<&'static str> {
        match self {
            Self::Sm => Some("btn--sm"),
            Self::Md => None,
            Self::Lg => Some("btn--lg"),
        }
    }
}

/// The class list for a `.btn`: base, family, emphasis, size, state, extra.
pub fn button_classes(family: ButtonFamily, quiet: bool, size: ButtonSize, saving: bool, extra: &str) -> String {
    let mut classes = vec!["btn", family.class()];
    if quiet { classes.push("btn--quiet"); }
    if let Some(size) = size.class() { classes.push(size); }
    if saving { classes.push("is-saving"); }
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

// ---------------------------------------------------------------------------
// Button
// ---------------------------------------------------------------------------

/// A standard padded button (`.btn`).
///
/// - `saving` sets `aria-disabled` and `.is-saving`, never `disabled`: the
///   button stays focusable mid-request, and the click is refused.
/// - `button_type` defaults to `"button"`, not HTML's `"submit"`. Pass
///   `"submit"` for a form's submit button.
///
/// ```ignore
/// <Button on_click=move |_| save()>"Save"</Button>
/// <Button family=ButtonFamily::Cancel on_click=move |_| close()>"Cancel"</Button>
/// <Button family=ButtonFamily::Secondary quiet=true size=ButtonSize::Sm>"Export"</Button>
/// <Button saving=saving button_type="submit">"Create"</Button>
/// ```
#[component]
pub fn Button(
    #[prop(optional)] family: ButtonFamily,
    #[prop(optional)] quiet: bool,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional, into)] saving: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = "button")] button_type: &'static str,
    #[prop(optional, into)] on_click: Option<UnsyncCallback<MouseEvent>>,
    #[prop(optional, into)] title: Option<String>,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let on_click = move |ev: MouseEvent| {
        if saving.get_untracked() {
            ev.prevent_default();
            return;
        }
        if let Some(cb) = &on_click { cb.run(ev); }
    };

    view! {
        <button
            type=button_type
            class=move || button_classes(family, quiet, size, saving.get(), &class.get().unwrap_or_default())
            title=title
            disabled=move || disabled.get()
            aria-disabled=move || saving.get().then_some("true")
            on:click=on_click
        >
            {children()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_primary_medium() {
        assert_eq!(button_classes(ButtonFamily::default(), false, ButtonSize::default(), false, ""), "btn btn--primary");
    }

    #[test]
    fn every_axis_adds_its_class() {
        assert_eq!(
            button_classes(ButtonFamily::Danger, true, ButtonSize::Sm, true, "extra"),
            "btn btn--danger btn--quiet btn--sm is-saving extra",
        );
    }
}
