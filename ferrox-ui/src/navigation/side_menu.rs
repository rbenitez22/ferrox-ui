use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_location;
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};

use crate::{MaybeHelp, NavList, Sheet, SheetSide};

/// The class list for a side menu's sheet: `.side-menu`, then any extra
/// classes.
pub fn side_menu_classes(extra: &str) -> String {
    if extra.is_empty() { "side-menu".to_string() } else { format!("side-menu {extra}") }
}

/// Navigation that slides in from an edge: a [`Sheet`](crate::Sheet) holding
/// a [`NavList`](crate::NavList) of the children, and an optional `footer`
/// pinned to the bottom.
///
/// - `open` is two-way: [`SideMenuButton`](crate::SideMenuButton) opens it,
///   and it closes itself when the user follows a link inside it (any
///   `<a href>`, not only `NavLink`, and the current page's link too) or the
///   route changes while it's open (the back button). Esc, the × and the
///   backdrop close it, as any sheet.
/// - Focus returns to whatever opened it.
/// - It must be inside a `<Router>`.
///
/// ```ignore
/// let menu = RwSignal::new(false);
/// view! {
///     <SideMenuButton open=menu />
///     <SideMenu open=menu title="Groceries">
///         <NavLink href="/">"Lists"</NavLink>
///         <NavGroup label="Product database" collapsible=true> … </NavGroup>
///     </SideMenu>
/// }
/// ```
#[component]
pub fn SideMenu(
    open: RwSignal<bool>,
    /// Shown in the title bar; the sheet's accessible name.
    #[prop(into)] title: Signal<String>,
    /// The name of the `<nav>` inside.
    #[prop(into, default = "Sections".into())] nav_label: Signal<String>,
    #[prop(optional)] side: SheetSide,
    /// Pinned to the bottom of the menu, below the links.
    #[prop(optional, into)] footer: Option<ViewFn>,
    /// A help topic: a `?` in the title bar, before the ×.
    #[prop(optional, into)] help: MaybeHelp,
    /// The × button's label.
    #[prop(into, default = "Close".into())] close_label: Signal<String>,
    /// Extra classes on the sheet, appended to `side-menu`. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    close_on_route_change(open);
    let on_click = move |ev: MouseEvent| {
        if follows_link(&ev) {
            // After the router has handled this same click.
            queue_microtask(move || open.set(false));
        }
    };
    // Stored, so the sheet can build its body afresh on each opening.
    let children = StoredValue::new(children);
    let footer = StoredValue::new(footer);

    view! {
        <Sheet
            open=open
            on_close=move || open.set(false)
            side=side
            title=title
            close_label=close_label
            help=help
            class=Signal::derive(move || side_menu_classes(&class.get().unwrap_or_default()))
        >
            <div class="side-menu__content" on:click=on_click>
                <NavList label=nav_label>{children.with_value(|children| children())}</NavList>
                {footer.get_value().map(|footer| view! { <div class="side-menu__footer">{footer.run()}</div> })}
            </div>
        </Sheet>
    }
}

/// Closes the menu whenever the route changes (not on the first run), so the
/// back button closes it too.
fn close_on_route_change(open: RwSignal<bool>) {
    let location = use_location();
    Effect::watch(move || location.pathname.get(), move |_, _, _| open.set(false), false);
}

/// Whether the click follows a link the way the router handles one: a plain
/// left click on an `<a href>` with no `target`. A ctrl-click opens a new tab
/// and leaves the menu as it was.
fn follows_link(ev: &MouseEvent) -> bool {
    if ev.default_prevented() || ev.button() != 0 || ev.meta_key() || ev.alt_key() || ev.ctrl_key() || ev.shift_key() {
        return false;
    }
    let Some(target) = ev.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return false };
    let Ok(Some(link)) = target.closest("a[href]") else { return false };
    link.get_attribute("target").is_none_or(|t| t.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(side_menu_classes(""), "side-menu");
        assert_eq!(side_menu_classes("app-menu"), "side-menu app-menu");
    }
}
