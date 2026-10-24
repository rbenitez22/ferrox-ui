use ferrox_icons::{Icon, IconData, CHECK};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use web_sys::KeyboardEvent;

use super::panel::MenuContext;

/// The class list for a `.menu__item`.
pub fn menu_item_classes(danger: bool, current: bool) -> String {
    let mut classes = vec!["menu__item"];
    if danger { classes.push("menu__item--danger"); }
    if current { classes.push("menu__item--current"); }
    classes.join(" ")
}

/// One thing a [`crate::Menu`] offers: a `<button role="menuitem">`.
/// Choosing it closes the menu, hands focus back to the trigger, then runs
/// `on_select`, so an action that opens a dialog or navigates takes focus
/// from there.
///
/// - `icon` is drawn before the text.
/// - `danger`: a destructive action, in the danger colour.
/// - `disabled`: still shown and reachable with the arrows (as ARIA asks of
///   menu items), announced as unavailable (`aria-disabled`), and does
///   nothing; the menu stays open.
/// - `current`: the one already in force (the open book). Marked with a
///   check and `aria-current`, and still choosable.
///
/// ```ignore
/// <MenuItem icon=EDIT on_select=move || edit()>"Edit"</MenuItem>
/// <MenuItem icon=DELETE danger=true on_select=move || ask_delete()>"Delete"</MenuItem>
/// ```
#[component]
pub fn MenuItem(
    #[prop(into)] on_select: UnsyncCallback<()>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional)] danger: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] current: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let menu = use_context::<MenuContext>();
    let choose = move |_| {
        if disabled.get_untracked() {
            return;
        }
        if let Some(menu) = menu {
            menu.close();
        }
        on_select.run(());
    };
    view! {
        <button
            type="button"
            role="menuitem"
            class=move || menu_item_classes(danger, current.get())
            tabindex="-1"
            aria-disabled=move || disabled.get().then_some("true")
            aria-current=move || current.get().then_some("true")
            on:click=choose
        >
            {icon.map(|icon| view! { <Icon icon=icon class="menu__icon" /> })}
            <span class="menu__label">{children()}</span>
            {move || current.get().then(|| view! { <Icon icon=CHECK class="menu__check" /> })}
        </button>
    }
}

/// A [`crate::Menu`] item that goes somewhere: an `<a role="menuitem"
/// href>`. Inside a `<Router>` the router follows it, without a page load.
/// Choosing it (a click, Enter or Space) closes the menu first.
///
/// ```ignore
/// <MenuLink href="/settings">"Settings"</MenuLink>
/// ```
#[component]
pub fn MenuLink(
    #[prop(into)] href: String,
    #[prop(optional)] icon: Option<IconData>,
    children: Children,
) -> impl IntoView {
    let menu = use_context::<MenuContext>();
    // The click goes on to the link (and the router); only the menu closes.
    let on_click = move |_| {
        if let Some(menu) = menu {
            menu.close();
        }
    };
    // Space presses a button but not a link: a menu item answers both.
    let on_keydown = move |ev: KeyboardEvent| {
        if ev.key() == " " {
            ev.prevent_default();
            if let Some(link) = ev.current_target().and_then(|t| wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(t).ok()) {
                link.click();
            }
        }
    };
    view! {
        <a href=href role="menuitem" class="menu__item" tabindex="-1" on:click=on_click on:keydown=on_keydown>
            {icon.map(|icon| view! { <Icon icon=icon class="menu__icon" /> })}
            <span class="menu__label">{children()}</span>
        </a>
    }
}

/// A line between groups of items (`role="separator"`).
#[component]
pub fn MenuDivider() -> impl IntoView {
    view! { <div role="separator" class="menu__divider"></div> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(menu_item_classes(false, false), "menu__item");
        assert_eq!(menu_item_classes(true, true), "menu__item menu__item--danger menu__item--current");
    }
}
