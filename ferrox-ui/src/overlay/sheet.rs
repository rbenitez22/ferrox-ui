use ferrox_icons::{Icon, CLOSE};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};

use super::modal::Modal;
use crate::ids::unique_id;
use crate::IconButton;

// ---------------------------------------------------------------------------
// SheetSide
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum SheetSide {
    /// Full height at the left edge, `min(20rem, 85vw)` wide: a side menu.
    #[default]
    Left,
    /// Full height at the right edge: details, filters.
    Right,
    /// Full width at the bottom, at most 85vh tall.
    Bottom,
}

impl SheetSide {
    pub fn class(self) -> &'static str {
        match self {
            Self::Left   => "sheet--left",
            Self::Right  => "sheet--right",
            Self::Bottom => "sheet--bottom",
        }
    }
}

/// The class list for a `.sheet`.
pub fn sheet_classes(side: SheetSide, extra: &str) -> String {
    let mut classes = vec!["sheet", side.class()];
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

// ---------------------------------------------------------------------------
// Sheet
// ---------------------------------------------------------------------------

/// A modal panel that slides in from an edge: a title bar with a close
/// button, then the body. Like `Dialog`, it is the native `<dialog>` opened
/// with `showModal()`: the page behind is inert, focus stays inside and
/// returns to the opener, the page does not scroll.
///
/// - `open` drives it; a click on the backdrop, the × and Esc call
///   `on_close`. (A `Dialog` ignores the backdrop, so a stray click cannot
///   throw away typed input; a sheet holds navigation or details.)
/// - It slides in over `--react-time`; under reduced motion it just appears.
///   Closing never waits on an animation.
/// - The body is mounted only while open.
///
/// ```ignore
/// <Sheet open=menu_open title="Menu" on_close=move || menu_open.set(false)>
///     <nav>…</nav>
/// </Sheet>
/// ```
#[component]
pub fn Sheet(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_close: UnsyncCallback<()>,
    #[prop(optional)] side: SheetSide,
    /// Shown in the title bar; also the sheet's accessible name.
    #[prop(into)] title: Signal<String>,
    /// The × button's label.
    #[prop(into, default = "Close".to_string())] close_label: String,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let modal = Modal::new(open, on_close);
    let (on_cancel, on_closed) = (modal.on_cancel(), modal.on_closed());
    let title_id = unique_id("ferrox-sheet-title");
    let labelled_by = title_id.clone();

    // A click closes only if it both started and ended on the backdrop, so a
    // text selection dragged out of the sheet does not close it.
    let pressed_outside = StoredValue::new(false);
    let on_mousedown = move |ev: MouseEvent| pressed_outside.set_value(on_backdrop(&ev));
    let on_click = move |ev: MouseEvent| {
        if pressed_outside.get_value() && on_backdrop(&ev) {
            on_close.run(());
        }
        pressed_outside.set_value(false);
    };

    view! {
        <dialog
            node_ref=modal.node_ref
            class=move || sheet_classes(side, &class.get().unwrap_or_default())
            aria-labelledby=labelled_by
            on:cancel=on_cancel
            on:close=on_closed
            on:mousedown=on_mousedown
            on:click=on_click
        >
            <div class="sheet__title-bar">
                <h2 class="sheet__title" id=title_id>{move || title.get()}</h2>
                <IconButton title=close_label class="sheet__close" on_click=move |_| on_close.run(())>
                    <Icon icon=CLOSE />
                </IconButton>
            </div>
            <Show when=move || open.get()>
                <div class="sheet__body">{children()}</div>
            </Show>
        </dialog>
    }
}

/// Whether the event is on the backdrop: aimed at the `<dialog>` itself (not
/// something inside it) and outside the panel's box.
fn on_backdrop(ev: &MouseEvent) -> bool {
    let (Some(target), Some(dialog)) = (ev.target(), ev.current_target()) else { return false };
    if target != dialog {
        return false;
    }
    let Ok(dialog) = dialog.dyn_into::<Element>() else { return false };
    let panel = dialog.get_bounding_client_rect();
    let (x, y) = (f64::from(ev.client_x()), f64::from(ev.client_y()));
    x < panel.left() || x > panel.right() || y < panel.top() || y > panel.bottom()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(SheetSide::default(), SheetSide::Left);
        assert_eq!(sheet_classes(SheetSide::Left, ""), "sheet sheet--left");
        assert_eq!(sheet_classes(SheetSide::Bottom, "extra"), "sheet sheet--bottom extra");
    }
}
