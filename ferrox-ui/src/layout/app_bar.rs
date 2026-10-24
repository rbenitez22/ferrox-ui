use leptos::prelude::*;
use leptos::{component, view, IntoView};

/// The class list for an `.app-bar`.
pub fn app_bar_classes(sticky: bool, extra: &str) -> String {
    let mut classes = vec!["app-bar"];
    if sticky { classes.push("app-bar--sticky"); }
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

/// The app's top band: what opens its menu, its name, and its actions (a
/// user menu, a search). A `<header>`, so at the top of the page it's the
/// banner landmark; the name is plain text, since the page's own `Panel`
/// carries its `<h1>`.
///
/// - `start` sits before the name: usually a `SideMenuButton`.
/// - `href` makes the name a link (home).
/// - The children sit at the end.
/// - It sticks to the top of the window as the page scrolls (`sticky`).
///
/// ```ignore
/// <AppBar title="Cookbook" href="/" start=move || view! { <SideMenuButton open=menu /> }>
///     <Menu label="Account" trigger=MenuTrigger::custom(…, "Account")>…</Menu>
/// </AppBar>
/// ```
#[component]
pub fn AppBar(
    /// The app's name.
    #[prop(into)] title: Signal<String>,
    /// Makes the name a link, usually home.
    #[prop(optional, into)] href: MaybeProp<String>,
    /// Before the name: what opens the app's menu.
    #[prop(optional, into)] start: Option<ViewFnOnce>,
    /// Stays at the top of the window while the page scrolls.
    #[prop(default = true)] sticky: bool,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    /// At the end: the app's actions.
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let title_view = if href.get_untracked().is_some() {
        view! { <a class="app-bar__title" href=move || href.get()>{move || title.get()}</a> }.into_any()
    } else {
        view! { <span class="app-bar__title">{move || title.get()}</span> }.into_any()
    };
    view! {
        <header class=move || app_bar_classes(sticky, &class.get().unwrap_or_default())>
            {start.map(|start| view! { <div class="app-bar__start">{start.run()}</div> })}
            {title_view}
            {children.map(|children| view! { <div class="app-bar__end">{children()}</div> })}
        </header>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(app_bar_classes(true, ""), "app-bar app-bar--sticky");
        assert_eq!(app_bar_classes(false, "x"), "app-bar x");
    }
}
