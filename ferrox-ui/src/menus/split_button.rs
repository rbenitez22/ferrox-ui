use leptos::prelude::*;
use leptos::{component, view, IntoView};
use web_sys::MouseEvent;

use super::menu::{Menu, MenuAlign, MenuTrigger};
use crate::{Button, ButtonFamily};

/// A main action and, joined to it, a chevron that opens related actions:
/// "Save ▾" with "Save as draft", "Save and close".
///
/// Two buttons, two tab stops, two names: the main one says what it does,
/// the chevron is named `menu_label`. The menu hangs from the group's end.
/// `family`, `saving` and `disabled` reach both halves: while saving, the
/// main button sweeps and refuses clicks, and the menu can't open.
///
/// ```ignore
/// <SplitButton text="Save" on_click=move |_| save() menu_label="More save options">
///     <MenuItem on_select=move || save_as_draft()>"Save as draft"</MenuItem>
///     <MenuItem on_select=move || save_and_close()>"Save and close"</MenuItem>
/// </SplitButton>
/// ```
#[component]
pub fn SplitButton(
    /// The main button's text.
    #[prop(into)] text: Signal<String>,
    on_click: impl Fn(MouseEvent) + 'static,
    /// The chevron's name, which is also the menu's: "More save options".
    #[prop(into)] menu_label: Signal<String>,
    #[prop(optional)] family: ButtonFamily,
    #[prop(optional, into)] saving: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Extra classes on the group. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The menu's `MenuItem`s, `MenuLink`s and `MenuDivider`s.
    children: ChildrenFn,
) -> impl IntoView {
    let menu_disabled = Signal::derive(move || disabled.get() || saving.get());
    view! {
        <div class=move || match class.get() {
            Some(extra) if !extra.is_empty() => format!("split-button {extra}"),
            _ => "split-button".to_string(),
        }>
            <Button family=family saving=saving disabled=disabled class="split-button__main" on_click=on_click>
                {text}
            </Button>
            <Menu
                label=menu_label
                trigger=MenuTrigger::chevron(menu_label, family)
                align=MenuAlign::End
                disabled=menu_disabled
                class="split-button__menu"
            >
                {children()}
            </Menu>
        </div>
    }
}
