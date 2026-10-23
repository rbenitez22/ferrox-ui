//! Places a floating element (a top-layer popover, `position: fixed`) next
//! to the element it belongs to: below it when there is room, above when
//! there is not, always inside the viewport, and following it while the page
//! scrolls or resizes. The help popover uses it, and the Combobox list.

use leptos::prelude::request_animation_frame;
use web_sys::{Element, HtmlElement};

/// Space between the anchor and the floating element, in px.
const GAP: f64 = 4.0;
/// The least space kept between the floating element and the viewport's edges, in px.
const MARGIN: f64 = 8.0;

/// A box in viewport coordinates, in px.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Rect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

/// Which side of the anchor the floating element went to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Side {
    Below,
    Above,
}

/// Where a floating box of `size` goes for `anchor` in a `viewport`:
/// below if it fits, else above if it fits, else the side with more room;
/// start-aligned with the anchor, then nudged inside the viewport.
pub(crate) fn position(anchor: Rect, width: f64, height: f64, viewport_width: f64, viewport_height: f64) -> (f64, f64, Side) {
    let below = anchor.top + anchor.height + GAP;
    let above = anchor.top - GAP - height;
    let fits_below = below + height <= viewport_height - MARGIN;
    let fits_above = above >= MARGIN;
    let room_below = viewport_height - below;
    let room_above = anchor.top - GAP;
    let side = if fits_below || (!fits_above && room_below >= room_above) { Side::Below } else { Side::Above };
    let top = match side {
        Side::Below => below,
        Side::Above => above.max(MARGIN),
    };
    let left = anchor.left.min(viewport_width - MARGIN - width).max(MARGIN);
    (left, top, side)
}

/// Places `floating` next to `anchor` now, and records the side it took in
/// `data-side` (`below` / `above`) for CSS and tests. With `match_width` it
/// also takes the anchor's width (a combobox's list under its field).
pub(crate) fn place(anchor: &Element, floating: &HtmlElement, match_width: bool) {
    let Some(window) = web_sys::window() else { return };
    let viewport_width = window.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(0.0);
    let viewport_height = window.inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(0.0);
    let a = anchor.get_bounding_client_rect();
    if match_width {
        let _ = floating.style().set_property("width", &format!("{}px", a.width()));
    }
    let f = floating.get_bounding_client_rect();
    let anchor = Rect { left: a.left(), top: a.top(), width: a.width(), height: a.height() };
    let (left, top, side) = position(anchor, f.width(), f.height(), viewport_width, viewport_height);
    let style = floating.style();
    let _ = style.set_property("left", &format!("{left}px"));
    let _ = style.set_property("top", &format!("{top}px"));
    let _ = floating.set_attribute("data-side", match side { Side::Below => "below", Side::Above => "above" });
}

/// Keeps `floating` placed next to `anchor`, once per frame, for as long as
/// it is an open popover. Per frame rather than on scroll events, so it
/// follows every way the anchor can move: page scroll, a scrolling parent,
/// a resize, the layout shifting.
pub(crate) fn follow(anchor: Element, floating: HtmlElement, match_width: bool) {
    if !floating.matches(":popover-open").unwrap_or(false) {
        return;
    }
    place(&anchor, &floating, match_width);
    request_animation_frame(move || follow(anchor, floating, match_width));
}

#[cfg(test)]
mod tests {
    use super::*;

    const VIEWPORT: (f64, f64) = (1000.0, 800.0);

    fn at(left: f64, top: f64) -> Rect {
        Rect { left, top, width: 20.0, height: 20.0 }
    }

    #[test]
    fn below_and_start_aligned_when_it_fits() {
        assert_eq!(position(at(100.0, 100.0), 300.0, 200.0, VIEWPORT.0, VIEWPORT.1), (100.0, 124.0, Side::Below));
    }

    #[test]
    fn above_when_there_is_no_room_below() {
        assert_eq!(position(at(100.0, 700.0), 300.0, 200.0, VIEWPORT.0, VIEWPORT.1), (100.0, 496.0, Side::Above));
    }

    #[test]
    fn the_roomier_side_when_it_fits_neither() {
        let (_, top, side) = position(at(100.0, 300.0), 300.0, 700.0, VIEWPORT.0, VIEWPORT.1);
        assert_eq!(side, Side::Below);
        assert_eq!(top, 324.0);
        let (_, top, side) = position(at(100.0, 500.0), 300.0, 700.0, VIEWPORT.0, VIEWPORT.1);
        assert_eq!(side, Side::Above);
        assert_eq!(top, MARGIN);
    }

    #[test]
    fn kept_inside_the_viewport_at_the_edges() {
        let (left, _, _) = position(at(950.0, 100.0), 300.0, 200.0, VIEWPORT.0, VIEWPORT.1);
        assert_eq!(left, 1000.0 - MARGIN - 300.0);
        let (left, _, _) = position(at(-50.0, 100.0), 300.0, 200.0, VIEWPORT.0, VIEWPORT.1);
        assert_eq!(left, MARGIN);
    }
}
