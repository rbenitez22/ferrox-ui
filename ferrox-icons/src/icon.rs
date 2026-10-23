use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::IconData;

/// The class list for an `.icon`.
pub fn icon_classes(extra: &str) -> String {
    if extra.is_empty() { "icon".to_string() } else { format!("icon {extra}") }
}

/// Draws an icon as an inline `<svg class="icon">`: 1em square, stroked in
/// `currentColor`, hidden from assistive technology (the control around it
/// carries the label). `data-icon` holds the icon's name.
///
/// ```ignore
/// <Icon icon=CLOSE/>
/// <Icon icon=CHEVRON_DOWN class="select__chevron"/>
/// <Icon icon=move || if open.get() { CHEVRON_DOWN } else { CHEVRON_RIGHT }/>
/// ```
#[component]
pub fn Icon(
    /// The icon: a const such as `CLOSE`, or a signal or closure choosing one.
    #[prop(into)] icon: Signal<IconData>,
    /// Extra classes, appended to `icon`. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <svg
            class=move || icon_classes(&class.get().unwrap_or_default())
            data-icon=move || icon.get().name
            viewBox="0 0 24 24"
            width="1em"
            height="1em"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            focusable="false"
            inner_html=move || icon.get().body
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CLOSE;

    #[test]
    fn classes() {
        assert_eq!(icon_classes(""), "icon");
        assert_eq!(icon_classes("select__chevron"), "icon select__chevron");
    }

    /// Type-checked, not run (drawing needs a browser): `icon` takes a const,
    /// a closure or a signal, and `class` is optional.
    #[allow(dead_code)]
    fn icon_takes_a_const_closure_or_signal() {
        let open = RwSignal::new(false);
        let chosen = RwSignal::new(CLOSE);
        let _ = view! { <Icon icon=CLOSE/> };
        let _ = view! { <Icon icon=move || if open.get() { CLOSE } else { chosen.get() } class="x"/> };
        let _ = view! { <Icon icon=chosen class=Signal::derive(move || if open.get() { "open".to_string() } else { String::new() })/> };
    }

    #[test]
    fn close_is_generated_from_its_file() {
        assert_eq!(CLOSE.name, "close");
        assert_eq!(CLOSE.body, r#"<path d="M18 6 6 18"/><path d="m6 6 12 12"/>"#);
    }
}
