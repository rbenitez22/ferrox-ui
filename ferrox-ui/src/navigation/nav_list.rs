use leptos::prelude::*;
use leptos::{component, view, IntoView};

/// The class list for a `.nav-list`.
pub fn nav_list_classes(extra: &str) -> String {
    if extra.is_empty() { "nav-list".to_string() } else { format!("nav-list {extra}") }
}

/// A labelled `<nav>` landmark holding a list of [`NavLink`](crate::NavLink)s,
/// so a screen reader hears the landmark's name and how many links it holds.
///
/// ```ignore
/// <NavList label="Sections">
///     <NavLink href="/">"Home"</NavLink>
///     <NavLink href="/accounts">"Accounts"</NavLink>
/// </NavList>
/// ```
#[component]
pub fn NavList(
    /// The landmark's name: "Sections", "Settings".
    #[prop(into)] label: Signal<String>,
    /// Extra classes, appended to `nav-list`. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <nav class=move || nav_list_classes(&class.get().unwrap_or_default()) aria-label=move || label.get()>
            <ul class="nav-list__items">{children()}</ul>
        </nav>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(nav_list_classes(""), "nav-list");
        assert_eq!(nav_list_classes("app-nav"), "nav-list app-nav");
    }
}
