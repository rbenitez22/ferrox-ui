use std::time::Duration;

use leptos::ev::pointerdown;
use leptos::html;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, KeyboardEvent, MouseEvent, TouchEvent};

use super::panel::{focus_end, is_open, panel_element, MenuContext, MenuPanel};
use crate::core::anchor::{place, place_at, Align};
use crate::ids::unique_id;

/// How long a touch must rest to open the menu.
const LONG_PRESS: Duration = Duration::from_millis(500);
/// How far a touch may wander and still count as resting, in px.
const PRESS_SLOP: f64 = 10.0;

/// Where a context menu opens.
#[derive(Clone)]
enum Spot {
    /// At the pointer (right-click, long press).
    Point(f64, f64),
    /// Under the focused element (the Menu key, Shift+F10).
    Below(HtmlElement),
}

/// Whether a key opens a context menu: the Menu key, or Shift+F10.
pub(crate) fn opens_context_menu(key: &str, shift: bool) -> bool {
    key == "ContextMenu" || (shift && key == "F10")
}

/// Whether a touch that started at `from` and is now at `to` still rests.
pub(crate) fn still_resting(from: (f64, f64), to: (f64, f64)) -> bool {
    (to.0 - from.0).hypot(to.1 - from.1) <= PRESS_SLOP
}

/// A menu of actions on an area (a row, a card), opened from the area
/// itself rather than a trigger: the same panel, items and keyboard as
/// [`crate::Menu`].
///
/// - **Opening:** a right-click in the area, at the pointer (the browser's
///   own menu is suppressed there only); the Menu key or Shift+F10 while
///   focus is in it, under the focused element; a long press on touch
///   (half a second; a touch that moves, as in scrolling, never opens it).
/// - The first item takes focus. Closing returns focus to what had it
///   before the menu opened.
/// - **Nothing shows that it exists**, so offer the same actions somewhere
///   visible too (a row's ⋮ `Menu`): this is a shortcut, not the only way.
///
/// ```ignore
/// <ContextMenu label="Recipe actions" menu=move || view! {
///     <MenuItem icon=EDIT on_select=move || edit()>"Edit"</MenuItem>
///     <MenuItem icon=DELETE danger=true on_select=move || ask_delete()>"Delete"</MenuItem>
/// }>
///     <Card title=recipe.name …>…</Card>
/// </ContextMenu>
/// ```
#[component]
pub fn ContextMenu(
    /// The menu's accessible name: "Recipe actions".
    #[prop(into)] label: Signal<String>,
    /// `MenuItem`s, `MenuLink`s and `MenuDivider`s.
    #[prop(into)] menu: ViewFnOnce,
    /// Shown above the items, outside the menu itself.
    #[prop(optional, into)] header: Option<ViewFnOnce>,
    /// Leaves the browser's own menu in place.
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Extra classes on the area's wrapper. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The area the menu belongs to.
    children: Children,
) -> impl IntoView {
    let panel_id = unique_id("ferrox-context-menu");
    let menu_id = format!("{panel_id}-items");
    let panel_ref = NodeRef::<html::Div>::new();
    let menu_ref = NodeRef::<html::Div>::new();
    let built = RwSignal::new(false);
    let spot = StoredValue::new_local(None::<Spot>);
    let returns_to = StoredValue::new_local(None::<HtmlElement>);
    // A resting touch: where it started, and its timer.
    let press_from = StoredValue::new(None::<(f64, f64)>);
    let press_timer = StoredValue::new(None::<TimeoutHandle>);
    let opened_by_press = StoredValue::new(false);

    let place_panel = move || {
        let Some(panel) = panel_element(panel_ref) else { return };
        spot.with_value(|spot| match spot {
            Some(Spot::Point(x, y)) => place_at(*x, *y, &panel),
            Some(Spot::Below(element)) => place(element, &panel, false, Align::Start),
            None => {}
        });
    };
    let open = move |at: Spot| {
        if disabled.get_untracked() || is_open(panel_ref) {
            return;
        }
        returns_to.set_value(document().active_element().and_then(|e| e.dyn_into::<HtmlElement>().ok()));
        spot.set_value(Some(at));
        if let Some(panel) = panel_element(panel_ref) {
            let _ = panel.show_popover();
            place_panel();
        }
    };
    let close = move || {
        if let Some(panel) = panel_element(panel_ref) {
            let _ = panel.hide_popover();
        }
        if let Some(element) = returns_to.get_value().filter(|e| e.is_connected()) {
            let _ = element.focus();
        }
    };
    let context = MenuContext { close: Callback::new(move |_| close()) };

    // A press outside closes it, without taking focus back. The panel is a
    // manual popover: as an auto one, the browser would take the release of
    // the very right-click that opened it (Firefox opens on mousedown) for a
    // press outside, and close it at once.
    let outside = window_event_listener(pointerdown, move |ev| {
        let Some(panel) = panel_element(panel_ref).filter(|_| is_open(panel_ref)) else { return };
        let inside = ev.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()).is_some_and(|n| panel.contains(Some(&n)));
        if !inside {
            let _ = panel.hide_popover();
        }
    });
    on_cleanup(move || outside.remove());

    let on_toggle = move |_: Event| {
        if !is_open(panel_ref) {
            return;
        }
        if built.get_untracked() {
            focus_end(menu_ref, false);
        } else {
            // The first opening builds the items: place and focus once they exist.
            built.set(true);
            request_animation_frame(move || {
                place_panel();
                focus_end(menu_ref, false);
            });
        }
    };

    let on_context_menu = move |ev: MouseEvent| {
        if disabled.get_untracked() {
            return;
        }
        ev.prevent_default();
        open(Spot::Point(f64::from(ev.client_x()), f64::from(ev.client_y())));
    };

    let on_keydown = move |ev: KeyboardEvent| {
        if !opens_context_menu(&ev.key(), ev.shift_key()) || disabled.get_untracked() || is_open(panel_ref) {
            return;
        }
        ev.prevent_default();
        let focused = document().active_element().and_then(|e| e.dyn_into::<HtmlElement>().ok());
        if let Some(element) = focused {
            open(Spot::Below(element));
        }
    };

    let cancel_press = move || {
        if let Some(timer) = press_timer.get_value() {
            timer.clear();
        }
        press_timer.set_value(None);
        press_from.set_value(None);
    };
    let on_touch_start = move |ev: TouchEvent| {
        cancel_press();
        let touches = ev.touches();
        if touches.length() != 1 || disabled.get_untracked() {
            return;
        }
        let Some(touch) = touches.get(0) else { return };
        let (x, y) = (f64::from(touch.client_x()), f64::from(touch.client_y()));
        press_from.set_value(Some((x, y)));
        let timer = set_timeout_with_handle(
            move || {
                press_timer.set_value(None);
                opened_by_press.set_value(true);
                open(Spot::Point(x, y));
            },
            LONG_PRESS,
        );
        press_timer.set_value(timer.ok());
    };
    let on_touch_move = move |ev: TouchEvent| {
        let (Some(from), Some(touch)) = (press_from.get_value(), ev.touches().get(0)) else { return };
        if !still_resting(from, (f64::from(touch.client_x()), f64::from(touch.client_y()))) {
            cancel_press();
        }
    };
    let on_touch_end = move |ev: TouchEvent| {
        cancel_press();
        // The press opened the menu: the lift must not click what was under it.
        if opened_by_press.get_value() {
            opened_by_press.set_value(false);
            ev.prevent_default();
        }
    };

    view! {
        <div
            class=move || match class.get() {
                Some(extra) if !extra.is_empty() => format!("context-menu {extra}"),
                _ => "context-menu".to_string(),
            }
            on:contextmenu=on_context_menu
            on:keydown=on_keydown
            on:touchstart=on_touch_start
            on:touchmove=on_touch_move
            on:touchend=on_touch_end
            on:touchcancel=move |_| cancel_press()
        >
            {children()}
            <MenuPanel
                id=panel_id
                menu_id=menu_id
                popover="manual"
                label=label
                panel_ref=panel_ref
                menu_ref=menu_ref
                header=header
                built=built.into()
                context=context
                on_toggle=on_toggle
                items=menu
            />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_menu_key_and_shift_f10_open_it() {
        assert!(opens_context_menu("ContextMenu", false));
        assert!(opens_context_menu("F10", true));
        assert!(!opens_context_menu("F10", false));
        assert!(!opens_context_menu("Enter", true));
    }

    #[test]
    fn a_touch_rests_within_the_slop() {
        assert!(still_resting((100.0, 100.0), (106.0, 106.0)));
        assert!(!still_resting((100.0, 100.0), (100.0, 115.0)));
    }
}
