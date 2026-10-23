use leptos::prelude::*;
use leptos::{component, view, IntoView};

// ---------------------------------------------------------------------------
// SpinnerKind
//
// Chosen by what stays usable while the work runs, not by how long it takes.
// The full-page case is `LoadingOverlay`; a committing action is
// `Button saving=…`.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpinnerKind {
    /// A pane fetching data whose shape it cannot promise. 28px, centred,
    /// with room around it. (A pane that knows its shape uses a skeleton.)
    #[default]
    Region,
    /// An icon-only row action: the spinner takes the icon's place. 1rem.
    Inline,
    /// Async work that belongs to one field (a lookup, a typeahead). 1rem.
    Field,
}

impl SpinnerKind {
    pub fn class(self) -> &'static str {
        match self {
            Self::Region => "spinner--region",
            Self::Inline => "spinner--inline",
            Self::Field  => "spinner--field",
        }
    }
}

/// The class list for a spinner's status wrapper.
pub fn spinner_status_classes(kind: SpinnerKind, extra: &str) -> String {
    let mut classes = vec!["spinner-status"];
    if kind == SpinnerKind::Region { classes.push("spinner-status--region"); }
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

// ---------------------------------------------------------------------------
// Spinner
// ---------------------------------------------------------------------------

/// A spinner for work that leaves the rest of the screen usable
/// (`role="status"`). It appears only after `--loading-delay` (400ms), so
/// work that finishes faster never shows one.
///
/// - `label` says what is loading, for screen readers; it is not shown.
/// - `kind` is where the work belongs: a region (default), an inline row
///   action, or a field.
///
/// ```ignore
/// <Spinner label="Loading accounts" />
/// <Spinner kind=SpinnerKind::Inline label="Deleting the row" />
/// ```
#[component]
pub fn Spinner(
    #[prop(into)] label: Signal<String>,
    #[prop(optional)] kind: SpinnerKind,
    /// Extra classes on the status wrapper. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <span role="status" class=move || spinner_status_classes(kind, &class.get().unwrap_or_default())>
            <span class=format!("spinner {}", kind.class())></span>
            <span class="sr-only">{move || label.get()}</span>
        </span>
    }
}

// ---------------------------------------------------------------------------
// LoadingOverlay
// ---------------------------------------------------------------------------

/// The full-page blocking overlay: a scrim with a large spinner.
///
/// **Only** for when nothing on screen is valid any more: boot, resolving
/// auth, the post-login redirect. Never for a route change or a data fetch:
/// loading one region must not grey out the rest; use [`Spinner`].
///
/// `text` is shown under the spinner and read out; without it, screen
/// readers hear "Loading…". Like the spinner, both appear after
/// `--loading-delay`.
///
/// ```ignore
/// {move || signing_in.get().then(|| view! { <LoadingOverlay text="Signing in…" /> })}
/// ```
#[component]
pub fn LoadingOverlay(
    #[prop(optional, into)] text: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class="spinner-overlay" role="status" aria-live="polite">
            <span class="spinner"></span>
            {move || match text.get() {
                Some(text) => view! { <span class="spinner-overlay__text">{text}</span> }.into_any(),
                None => view! { <span class="sr-only">"Loading…"</span> }.into_any(),
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_classes() {
        assert_eq!(SpinnerKind::default(), SpinnerKind::Region);
        assert_eq!(SpinnerKind::Inline.class(), "spinner--inline");
        assert_eq!(SpinnerKind::Field.class(), "spinner--field");
    }

    #[test]
    fn status_classes() {
        assert_eq!(spinner_status_classes(SpinnerKind::Region, ""), "spinner-status spinner-status--region");
        assert_eq!(spinner_status_classes(SpinnerKind::Inline, "extra"), "spinner-status extra");
    }
}
