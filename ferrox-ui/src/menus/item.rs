use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::menu::MenuContext;

/// One thing a [`crate::Menu`] offers: a `<button role="menuitem">`.
/// Choosing it closes the menu, hands focus back to the trigger, then runs
/// `on_select`, so an action that opens a dialog or navigates takes focus
/// from there.
///
/// ```ignore
/// <MenuItem on_select=move || duplicate()>"Duplicate"</MenuItem>
/// ```
#[component]
pub fn MenuItem(
    #[prop(into)] on_select: UnsyncCallback<()>,
    children: Children,
) -> impl IntoView {
    let menu = use_context::<MenuContext>();
    let choose = move |_| {
        if let Some(menu) = menu {
            menu.close();
        }
        on_select.run(());
    };
    view! {
        <button type="button" role="menuitem" class="menu__item" tabindex="-1" on:click=choose>
            {children()}
        </button>
    }
}

/// A line between groups of items (`role="separator"`).
#[component]
pub fn MenuDivider() -> impl IntoView {
    view! { <div role="separator" class="menu__divider"></div> }
}
