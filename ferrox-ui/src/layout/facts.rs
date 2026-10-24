use leptos::prelude::*;
use leptos::{component, view, IntoView};

/// A record's facts, label beside value: a `<dl>` in two columns, labels
/// muted, one under the other on a phone. Each line is a [`Fact`].
///
/// ```ignore
/// <Facts>
///     <Fact label="Time">"30 minutes"</Fact>
///     <Fact label="Serves">"4"</Fact>
/// </Facts>
/// ```
#[component]
pub fn Facts(
    /// Extra classes on the `<dl>`. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    /// `Fact`s.
    children: Children,
) -> impl IntoView {
    view! {
        <dl class=move || match class.get() {
            Some(extra) if !extra.is_empty() => format!("facts {extra}"),
            _ => "facts".to_string(),
        }>
            {children()}
        </dl>
    }
}

/// One line of [`Facts`]: its `label` (`<dt>`) and its value, the children
/// (`<dd>`).
#[component]
pub fn Fact(
    #[prop(into)] label: Signal<String>,
    /// Extra classes on the line. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || match class.get() {
            Some(extra) if !extra.is_empty() => format!("facts__row {extra}"),
            _ => "facts__row".to_string(),
        }>
            <dt class="facts__label">{move || label.get()}</dt>
            <dd class="facts__value">{children()}</dd>
        </div>
    }
}
