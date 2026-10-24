use ferrox_icons::{Icon, IconData, CHEVRON_DOWN};
use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, FocusEvent, HtmlElement, KeyboardEvent};

use super::keys::{move_to, type_ahead};
use crate::core::anchor::{follow, place, Align};
use crate::ids::unique_id;
use crate::{button_classes, icon_button_classes, ButtonFamily, ButtonSize};

/// What opens a [`Menu`].
#[derive(Clone)]
pub enum MenuTrigger {
    /// A button with the text and a chevron: "Export ▾".
    Button { text: String, family: ButtonFamily },
    /// An icon button; `label` is its only name. Row actions, "More".
    Icon { icon: IconData, label: String },
    /// The caller's view inside a plain button; `label` names it.
    Custom { view: ViewFn, label: String },
}

impl MenuTrigger {
    /// A secondary button with the text and a chevron.
    pub fn button(text: impl Into<String>) -> Self {
        Self::Button { text: text.into(), family: ButtonFamily::Secondary }
    }

    pub fn icon(icon: IconData, label: impl Into<String>) -> Self {
        Self::Icon { icon, label: label.into() }
    }

    pub fn custom(view: impl Into<ViewFn>, label: impl Into<String>) -> Self {
        Self::Custom { view: view.into(), label: label.into() }
    }
}

/// Which edge of the trigger a [`Menu`]'s panel lines up with.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuAlign {
    /// The right edges: a menu at a row's or a bar's end.
    #[default]
    End,
    /// The left edges: a menu read left to right from its button.
    Start,
}

impl From<MenuAlign> for Align {
    fn from(align: MenuAlign) -> Self {
        match align {
            MenuAlign::End => Align::End,
            MenuAlign::Start => Align::Start,
        }
    }
}

/// What a menu's items need from it: closing it, focus back on the trigger.
#[derive(Clone, Copy)]
pub(crate) struct MenuContext {
    close: Callback<()>,
}

impl MenuContext {
    pub(crate) fn close(&self) {
        self.close.run(());
    }
}

/// The menu's items, in order.
fn items_of(menu: &Element) -> Vec<HtmlElement> {
    let Ok(found) = menu.query_selector_all("[role=menuitem]") else { return Vec::new() };
    (0..found.length()).filter_map(|i| found.item(i)).filter_map(|n| n.dyn_into::<HtmlElement>().ok()).collect()
}

/// A list of actions behind a trigger, as ARIA's menu button has it.
///
/// - **Opening:** a click, Enter or Space on the trigger opens it with the
///   first item focused; ↓ opens on the first, ↑ on the last.
/// - **In the menu:** ↑ / ↓ move, wrapping; Home / End; a letter moves to
///   the next item starting with it; Enter or Space chooses.
/// - **Closing:** choosing an item, Esc (focus goes back to the trigger,
///   and an enclosing dialog stays open), Tab (focus moves on), or a press
///   outside.
/// - **The panel** is a popover in the top layer, under the trigger (above
///   when there's no room), lined up with its end or start (`align`), kept
///   inside the viewport, following the trigger while open. No `overflow`
///   parent clips it. Its items are built on first opening, then kept.
///
/// ```ignore
/// <Menu label="Recipe actions" trigger=MenuTrigger::icon(MORE_VERTICAL, "More actions")>
///     <MenuItem on_select=move || edit()>"Edit"</MenuItem>
///     <MenuDivider />
///     <MenuItem on_select=move || ask_delete()>"Delete"</MenuItem>
/// </Menu>
/// ```
#[component]
pub fn Menu(
    /// The menu's accessible name: "Recipe actions".
    #[prop(into)] label: Signal<String>,
    /// What opens it.
    trigger: MenuTrigger,
    #[prop(optional)] align: MenuAlign,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Extra classes on the wrapper. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    /// `MenuItem`s and `MenuDivider`s.
    children: ChildrenFn,
) -> impl IntoView {
    let panel_id = unique_id("ferrox-menu");
    let menu_id = format!("{panel_id}-items");
    let trigger_ref = NodeRef::<html::Button>::new();
    let panel_ref = NodeRef::<html::Div>::new();
    let menu_ref = NodeRef::<html::Div>::new();
    let open = RwSignal::new(false);
    let opened_once = RwSignal::new(false);
    // Which item opening focuses: the first, or the last (↑ on the trigger).
    let focus_last = StoredValue::new(false);

    let panel = move || panel_ref.get_untracked().map(|p| -> HtmlElement { p.unchecked_into() });
    let focus_item = move |last: bool| {
        let Some(menu) = menu_ref.get_untracked() else { return };
        let items = items_of(&menu);
        let target = if last { items.last() } else { items.first() };
        if let Some(item) = target {
            let _ = item.focus();
        }
    };
    let close_to_trigger = move || {
        if let Some(panel) = panel() {
            let _ = panel.hide_popover();
        }
        if let Some(trigger) = trigger_ref.get_untracked() {
            let _ = trigger.focus();
        }
    };
    // Given to the items through a `Provider`, not `provide_context`: a
    // component has no owner of its own, so sibling menus would overwrite
    // each other's.
    let context = MenuContext { close: Callback::new(move |_| close_to_trigger()) };

    let on_toggle = move |_: Event| {
        let (Some(trigger), Some(panel)) = (trigger_ref.get_untracked(), panel()) else { return };
        let is_open = panel.matches(":popover-open").unwrap_or(false);
        open.set(is_open);
        if is_open {
            let anchor: Element = trigger.unchecked_into();
            place(&anchor, &panel, false, align.into());
            follow(anchor, panel, false, align.into());
            let last = focus_last.get_value();
            focus_last.set_value(false);
            if opened_once.get_untracked() {
                focus_item(last);
            } else {
                // The first opening builds the items: focus once they exist.
                opened_once.set(true);
                request_animation_frame(move || focus_item(last));
            }
        }
    };

    let on_trigger_key = move |ev: KeyboardEvent| {
        let key = ev.key();
        if key != "ArrowDown" && key != "ArrowUp" {
            return;
        }
        ev.prevent_default();
        let Some(panel) = panel() else { return };
        if panel.matches(":popover-open").unwrap_or(false) {
            focus_item(key == "ArrowUp");
        } else {
            focus_last.set_value(key == "ArrowUp");
            let _ = panel.show_popover();
        }
    };

    let on_menu_key = move |ev: KeyboardEvent| {
        let Some(menu) = menu_ref.get_untracked() else { return };
        let key = ev.key();
        if key == "Escape" {
            // Handled here, so an enclosing dialog does not close too.
            ev.prevent_default();
            ev.stop_propagation();
            close_to_trigger();
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
        let Some(panel) = panel() else { return };
        let inside = ev.related_target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()).is_some_and(|n| panel.contains(Some(&n)));
        if !inside {
            let _ = panel.hide_popover();
        }
    };

    let expanded = move || if open.get() { "true" } else { "false" };
    let (controls, target) = (menu_id.clone(), panel_id.clone());
    let trigger_view = match trigger {
        MenuTrigger::Button { text, family } => view! {
            <button
                type="button"
                node_ref=trigger_ref
                class=button_classes(family, false, ButtonSize::Md, false, "menu__trigger")
                aria-haspopup="menu"
                aria-expanded=expanded
                aria-controls=controls
                popovertarget=target
                disabled=move || disabled.get()
                on:keydown=on_trigger_key
            >
                <span>{text}</span>
                <Icon icon=CHEVRON_DOWN class="menu__chevron" />
            </button>
        }.into_any(),
        MenuTrigger::Icon { icon, label: name } => view! {
            <button
                type="button"
                node_ref=trigger_ref
                class=icon_button_classes(false, "menu__trigger")
                aria-label=name.clone()
                title=name
                aria-haspopup="menu"
                aria-expanded=expanded
                aria-controls=controls
                popovertarget=target
                disabled=move || disabled.get()
                on:keydown=on_trigger_key
            >
                <Icon icon=icon />
            </button>
        }.into_any(),
        MenuTrigger::Custom { view: content, label: name } => view! {
            <button
                type="button"
                node_ref=trigger_ref
                class="menu__trigger menu__trigger--custom"
                aria-label=name
                aria-haspopup="menu"
                aria-expanded=expanded
                aria-controls=controls
                popovertarget=target
                disabled=move || disabled.get()
                on:keydown=on_trigger_key
            >
                {content.run()}
            </button>
        }.into_any(),
    };

    view! {
        <div class=move || match class.get() {
            Some(extra) if !extra.is_empty() => format!("menu {extra}"),
            _ => "menu".to_string(),
        }>
            {trigger_view}
            <div
                popover="auto"
                id=panel_id
                class="menu__panel"
                node_ref=panel_ref
                on:toggle=on_toggle
                on:keydown=on_menu_key
                on:focusout=on_focus_out
            >
                <div role="menu" id=menu_id class="menu__items" aria-label=move || label.get() node_ref=menu_ref>
                    <Show when=move || opened_once.get()>
                        {let children = children.clone(); view! { <Provider value=context>{children()}</Provider> }}
                    </Show>
                </div>
            </div>
        </div>
    }
}
