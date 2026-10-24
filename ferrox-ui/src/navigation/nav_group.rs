use ferrox_icons::{Icon, CHEVRON_RIGHT};
use leptos::html::Ul;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_location;
use web_sys::Element;

use crate::ids::unique_id;

/// The class list for a `.nav-group`.
pub fn nav_group_classes(collapsible: bool, extra: &str) -> String {
    let mut classes = vec!["nav-group"];
    if collapsible { classes.push("nav-group--collapsible"); }
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

/// Links that belong together inside a [`NavList`](crate::NavList), under a
/// heading that names them.
///
/// - **Plain:** the heading is quiet text, and a hairline splits the group
///   from what comes before it.
/// - **`collapsible`:** the heading is a button (`aria-expanded`) that shows
///   and hides the links. It starts collapsed unless `start_expanded`; pass
///   `expanded` to own the state (to remember it, say).
/// - **The current page is never hidden:** a collapsed group expands itself
///   when the route moves to one of its links.
///
/// One level only: a group holds `NavLink`s, not other groups.
///
/// ```ignore
/// <NavList label="Sections">
///     <NavLink href="/">"Lists"</NavLink>
///     <NavGroup label="Product database" collapsible=true>
///         <NavLink href="/units">"Units"</NavLink>
///         <NavLink href="/products">"Products"</NavLink>
///     </NavGroup>
///     <NavGroup label="Account">
///         <NavLink href="/account">"Account"</NavLink>
///     </NavGroup>
/// </NavList>
/// ```
#[component]
pub fn NavGroup(
    /// The heading, and the name of the group's list.
    #[prop(into)] label: Signal<String>,
    /// The heading is a button that shows and hides the links.
    #[prop(optional)] collapsible: bool,
    /// Where a collapsible group starts: collapsed unless this is true.
    #[prop(optional)] start_expanded: bool,
    /// Two-way expanded state, owned by the app (overrides `start_expanded`).
    #[prop(optional)] expanded: Option<RwSignal<bool>>,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let heading_id = unique_id("ferrox-nav-group");
    let list_id = unique_id("ferrox-nav-group-list");
    let expanded = expanded.unwrap_or_else(|| RwSignal::new(start_expanded));
    let list_ref = NodeRef::<Ul>::new();
    if collapsible {
        expand_on_arrival(list_ref, expanded);
    }

    let heading = if collapsible {
        view! {
            <button
                type="button"
                class="nav-group__toggle"
                id=heading_id.clone()
                aria-expanded=move || expanded.get().to_string()
                aria-controls=list_id.clone()
                on:click=move |_| expanded.update(|e| *e = !*e)
            >
                <span class="nav-group__label">{move || label.get()}</span>
                <Icon icon=CHEVRON_RIGHT class="nav-group__chevron" />
            </button>
        }
        .into_any()
    } else {
        view! { <span class="nav-group__heading" id=heading_id.clone()>{move || label.get()}</span> }.into_any()
    };

    view! {
        <li class=move || nav_group_classes(collapsible, &class.get().unwrap_or_default())>
            {heading}
            <ul
                class="nav-group__list"
                id=list_id.clone()
                aria-labelledby=heading_id
                hidden=move || collapsible && !expanded.get()
                node_ref=list_ref
            >
                {children()}
            </ul>
        </li>
    }
}

/// Expands the group whenever the route lands on one of its links, so the
/// current page is never hidden. It acts on arrival only: the user can still
/// collapse the group while on one of its pages.
fn expand_on_arrival(list_ref: NodeRef<Ul>, expanded: RwSignal<bool>) {
    let location = use_location();
    Effect::new(move |_| {
        location.pathname.track();
        // A frame later, once the router has moved `aria-current`.
        request_animation_frame(move || {
            if list_ref.get_untracked().is_some_and(|list| holds_current_page(&list)) {
                expanded.set(true);
            }
        });
    });
}

/// Whether `list` holds the link to the page the user is on.
fn holds_current_page(list: &Element) -> bool {
    list.query_selector("[aria-current='page']").ok().flatten().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(nav_group_classes(false, ""), "nav-group");
        assert_eq!(nav_group_classes(true, ""), "nav-group nav-group--collapsible");
        assert_eq!(nav_group_classes(true, "app-group"), "nav-group nav-group--collapsible app-group");
    }
}
