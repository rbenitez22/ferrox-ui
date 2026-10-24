use ferrox_icons::{Icon, IconData};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::components::A;

/// The class list for a `.nav-link`.
pub fn nav_link_classes(extra: &str) -> String {
    if extra.is_empty() { "nav-link".to_string() } else { format!("nav-link {extra}") }
}

/// One link in a [`NavList`](crate::NavList): a list item holding a router
/// link (`.nav-link`), with an optional icon before the words.
///
/// The router marks the link to the page the user is on with
/// `aria-current="page"`, which is also what tints it. A link counts as
/// current on its own path and on every path below it (`/accounts` on
/// `/accounts/42`); `exact` limits it to its own path. `/` is current only on
/// `/`.
///
/// It renders an `<li>`, so it goes inside a `NavList`.
///
/// ```ignore
/// <NavLink href="/" icon=HOME>"Home"</NavLink>
/// <NavLink href="/accounts" exact=true>"All accounts"</NavLink>
/// ```
#[component]
pub fn NavLink(
    #[prop(into)] href: String,
    /// Current only on this exact path, not on the paths below it.
    #[prop(optional)] exact: bool,
    /// An icon before the words, hidden from screen readers.
    #[prop(optional)] icon: Option<IconData>,
    /// Extra classes on the link, appended to `nav-link`. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <li>
            <A href=href exact=exact attr:class=move || nav_link_classes(&class.get().unwrap_or_default())>
                {icon.map(|icon| view! { <Icon icon=icon class="nav-link__icon" /> })}
                {children()}
            </A>
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(nav_link_classes(""), "nav-link");
        assert_eq!(nav_link_classes("app-link"), "nav-link app-link");
    }
}
