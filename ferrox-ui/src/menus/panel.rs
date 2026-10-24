//! The panel both `Menu` and `ContextMenu` open: a top-layer popover holding
//! an optional header and the `role="menu"` of items, with the keyboard
//! inside it.

use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, FocusEvent, HtmlElement, KeyboardEvent};

use super::keys::{move_to, type_ahead};
use crate::once::built_once;

/// What a menu's items need from it: closing it, focus back where it
/// belongs (the trigger, or what had focus before a context menu).
#[derive(Clone, Copy)]
pub(crate) struct MenuContext {
    pub(crate) close: Callback<()>,
}

impl MenuContext {
    pub(crate) fn close(&self) {
        self.close.run(());
    }
}

/// The panel element, as an `HtmlElement` (for the popover calls).
pub(crate) fn panel_element(panel: NodeRef<html::Div>) -> Option<HtmlElement> {
    panel.get_untracked().map(|p| p.unchecked_into())
}

/// Whether the panel is open.
pub(crate) fn is_open(panel: NodeRef<html::Div>) -> bool {
    panel_element(panel).is_some_and(|p| p.matches(":popover-open").unwrap_or(false))
}

/// The menu's items, in order.
fn items_of(menu: &Element) -> Vec<HtmlElement> {
    let Ok(found) = menu.query_selector_all("[role=menuitem]") else { return Vec::new() };
    (0..found.length()).filter_map(|i| found.item(i)).filter_map(|n| n.dyn_into::<HtmlElement>().ok()).collect()
}

/// Focuses the first item, or the last.
pub(crate) fn focus_end(menu: NodeRef<html::Div>, last: bool) {
    let Some(menu) = menu.get_untracked() else { return };
    let items = items_of(&menu);
    if let Some(item) = if last { items.last() } else { items.first() } {
        let _ = item.focus();
    }
}

/// The panel: `header` above a `role="menu"` of `items`, built only once
/// `built` turns true and kept after. `popover` is `"auto"` (the browser
/// closes it on a press outside) or `"manual"` (the owner does). Inside it: ↑ / ↓ (wrapping), Home /
/// End, a letter; Esc closes through `context` (and stops there, so an
/// enclosing dialog stays open); focus leaving it closes it.
#[component]
pub(crate) fn MenuPanel(
    id: String,
    menu_id: String,
    popover: &'static str,
    label: Signal<String>,
    panel_ref: NodeRef<html::Div>,
    menu_ref: NodeRef<html::Div>,
    header: Option<ViewFnOnce>,
    built: Signal<bool>,
    context: MenuContext,
    on_toggle: impl Fn(Event) + 'static,
    items: ViewFnOnce,
) -> impl IntoView {
    // Built on first opening, then kept: closures inside may move values.
    let items = built_once(items);
    let on_key = move |ev: KeyboardEvent| {
        let Some(menu) = menu_ref.get_untracked() else { return };
        let key = ev.key();
        if key == "Escape" {
            ev.prevent_default();
            ev.stop_propagation();
            context.close();
            return;
        }
        let items = items_of(&menu);
        let active = document().active_element();
        let at = items.iter().position(|item| Some(item.unchecked_ref::<Element>()) == active.as_ref());
        let to = match move_to(at, items.len(), &key) {
            Some(to) => Some(to),
            None => {
                let mut chars = key.chars();
                match (chars.next(), chars.next()) {
                    (Some(ch), None) if !ch.is_whitespace() && !ev.ctrl_key() && !ev.meta_key() && !ev.alt_key() => {
                        let labels: Vec<String> = items.iter().map(|i| i.text_content().unwrap_or_default()).collect();
                        type_ahead(&labels, at, ch)
                    }
                    _ => return,
                }
            }
        };
        ev.prevent_default();
        if let Some(item) = to.and_then(|to| items.get(to)) {
            let _ = item.focus();
        }
    };

    // Tab, or any other way focus leaves the panel, closes it.
    let on_focus_out = move |ev: FocusEvent| {
        let Some(panel) = panel_element(panel_ref) else { return };
        let inside = ev.related_target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()).is_some_and(|n| panel.contains(Some(&n)));
        if !inside {
            let _ = panel.hide_popover();
        }
    };

    view! {
        <div
            popover=popover
            id=id
            class="menu__panel"
            node_ref=panel_ref
            on:toggle=on_toggle
            on:keydown=on_key
            on:focusout=on_focus_out
        >
            {header.map(|header| view! { <div class="menu__header">{header.run()}</div> })}
            <div role="menu" id=menu_id class="menu__items" aria-label=move || label.get() node_ref=menu_ref>
                // A `Provider`, not `provide_context`: a component has no owner
                // of its own, so sibling menus would overwrite each other's.
                <Show when=move || built.get()>
                    <Provider value=context>{items()}</Provider>
                </Show>
            </div>
        </div>
    }
}
