use ferrox_icons::{Icon, IconData, CHEVRON_DOWN};
use leptos::html;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, KeyboardEvent};

use super::panel::{focus_end, is_open, panel_element, MenuContext, MenuPanel};
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
    /// A button of the family showing only a chevron; `label` is its name.
    /// `SplitButton`'s second half.
    Chevron { label: String, family: ButtonFamily },
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

    pub fn chevron(label: impl Into<String>, family: ButtonFamily) -> Self {
        Self::Chevron { label: label.into(), family }
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

/// A list of actions behind a trigger, as ARIA's menu button has it.
///
/// - **Opening:** a click, Enter or Space on the trigger opens it with the
///   first item focused; ↓ opens on the first, ↑ on the last.
/// - **In the menu:** ↑ / ↓ move, wrapping; Home / End; a letter moves to
///   the next item starting with it; Enter or Space chooses.
/// - **Closing:** choosing an item, Esc (focus goes back to the trigger,
///   and an enclosing dialog stays open), Tab (focus moves on), or a press
///   outside.
/// - `header` sits above the items, outside the menu (who is signed in).
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
    /// Shown above the items, outside the menu itself: who is signed in.
    #[prop(optional, into)] header: Option<ViewFn>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Extra classes on the wrapper. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    /// `MenuItem`s, `MenuLink`s and `MenuDivider`s.
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

    let focus_item = move |last: bool| focus_end(menu_ref, last);
    let close_to_trigger = move || {
        if let Some(panel) = panel_element(panel_ref) {
            let _ = panel.hide_popover();
        }
        if let Some(trigger) = trigger_ref.get_untracked() {
            let _ = trigger.focus();
        }
    };
    let context = MenuContext { close: Callback::new(move |_| close_to_trigger()) };

    let on_toggle = move |_: Event| {
        let (Some(trigger), Some(panel)) = (trigger_ref.get_untracked(), panel_element(panel_ref)) else { return };
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
        if is_open(panel_ref) {
            focus_item(key == "ArrowUp");
        } else if let Some(panel) = panel_element(panel_ref) {
            focus_last.set_value(key == "ArrowUp");
            let _ = panel.show_popover();
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
        MenuTrigger::Chevron { label: name, family } => view! {
            <button
                type="button"
                node_ref=trigger_ref
                class=button_classes(family, false, ButtonSize::Md, false, "menu__trigger menu__trigger--chevron")
                aria-label=name.clone()
                title=name
                aria-haspopup="menu"
                aria-expanded=expanded
                aria-controls=controls
                popovertarget=target
                disabled=move || disabled.get()
                on:keydown=on_trigger_key
            >
                <Icon icon=CHEVRON_DOWN class="menu__chevron" />
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
            <MenuPanel
                id=panel_id
                menu_id=menu_id
                popover="auto"
                label=label
                panel_ref=panel_ref
                menu_ref=menu_ref
                header=header
                built=opened_once.into()
                context=context
                on_toggle=on_toggle
                items=ViewFn::from(move || children())
            />
        </div>
    }
}
