//! Drives one native `<dialog>` as a modal from an `open` signal. Dialog,
//! ConfirmDialog and Sheet all go through here, so they open, close, take
//! focus and lock the page the same way.
//!
//! The signal is the source of truth: `open` turning true shows the dialog
//! (`showModal`, which makes the rest of the page inert and keeps focus
//! inside), turning false closes it. When the browser closes it instead (Esc,
//! or Chrome closing anyway on a repeated Esc), the app hears it through
//! `on_close`; its state never changes behind its back.

use std::cell::Cell;

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlDialogElement, HtmlElement};

/// The class `<body>` carries while any modal is open: no page scroll.
const SCROLL_LOCK_CLASS: &str = "dialog-open";

/// Where focus goes when a modal opens, after any `[autofocus]` element.
const FOCUSABLE: &str = "a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex='-1'])";

thread_local! {
    /// How many modals hold the scroll lock (a confirm opened from a dialog
    /// makes two).
    static LOCKS: Cell<usize> = const { Cell::new(0) };
    /// The last id handed out by `unique_id`.
    static LAST_ID: Cell<usize> = const { Cell::new(0) };
}

/// One modal's wiring. Put `node_ref` on the `<dialog>`, `on_cancel` on its
/// `cancel` event and `on_closed` on its `close` event.
#[derive(Clone, Copy)]
pub(crate) struct Modal {
    pub node_ref: NodeRef<html::Dialog>,
    open: Signal<bool>,
    on_close: UnsyncCallback<()>,
    /// The element that had focus before opening; it gets focus back.
    opener: StoredValue<Option<HtmlElement>, LocalStorage>,
    /// Whether this modal holds one of the page's scroll locks.
    locked: StoredValue<bool>,
}

impl Modal {
    /// Creates the wiring and the effect that follows `open`.
    pub fn new(open: Signal<bool>, on_close: UnsyncCallback<()>) -> Self {
        let modal = Self {
            node_ref: NodeRef::new(),
            open,
            on_close,
            opener: StoredValue::new_local(None),
            locked: StoredValue::new(false),
        };
        Effect::new(move |_| modal.follow_open());
        // A modal removed while open must not leave the page locked.
        let locked = modal.locked;
        on_cleanup(move || unlock_scroll(locked));
        modal
    }

    /// Esc: the browser would close the dialog itself; the app decides instead.
    pub fn on_cancel(self) -> impl Fn(Event) + 'static {
        let on_close = self.on_close;
        move |ev: Event| {
            ev.prevent_default();
            on_close.run(());
        }
    }

    /// The dialog has closed, by `close()` or by the browser: give back the
    /// scroll lock and focus, and tell the app if it still thinks it is open.
    pub fn on_closed(self) -> impl Fn(Event) + 'static {
        move |_: Event| {
            unlock_scroll(self.locked);
            if let Some(opener) = self.opener.try_update_value(Option::take).flatten() {
                let _ = opener.focus();
            }
            if self.open.get_untracked() {
                self.on_close.run(());
            }
        }
    }

    /// Shows or closes the element to match `open`.
    fn follow_open(self) {
        let open = self.open.get();
        let Some(dialog) = self.node_ref.get() else { return };
        if open && !dialog.open() {
            self.show(dialog);
        } else if !open && dialog.open() {
            dialog.close();
        }
    }

    fn show(self, dialog: HtmlDialogElement) {
        self.opener.set_value(focused_element());
        if dialog.show_modal().is_err() {
            return;
        }
        lock_scroll(self.locked);
        // After this frame, so content mounted on open (`autofocus`) is there.
        request_animation_frame(move || focus_first(&dialog));
    }
}

/// A page-unique id, `{prefix}-{n}`, so two modals never share one.
pub(crate) fn unique_id(prefix: &str) -> String {
    let n = LAST_ID.with(|last| {
        last.set(last.get() + 1);
        last.get()
    });
    format!("{prefix}-{n}")
}

/// Focus the `[autofocus]` element, else the first control, else the dialog.
fn focus_first(dialog: &HtmlDialogElement) {
    let target = dialog
        .query_selector("[autofocus]")
        .ok()
        .flatten()
        .or_else(|| dialog.query_selector(FOCUSABLE).ok().flatten())
        .and_then(|el| el.dyn_into::<HtmlElement>().ok());
    match target {
        Some(el) => { let _ = el.focus(); }
        None => { let _ = dialog.focus(); }
    }
}

fn focused_element() -> Option<HtmlElement> {
    document().active_element().and_then(|el| el.dyn_into::<HtmlElement>().ok())
}

fn lock_scroll(locked: StoredValue<bool>) {
    if locked.get_value() {
        return;
    }
    locked.set_value(true);
    let count = LOCKS.with(|locks| {
        locks.set(locks.get() + 1);
        locks.get()
    });
    if count == 1 {
        set_body_class(true);
    }
}

fn unlock_scroll(locked: StoredValue<bool>) {
    if !locked.try_get_value().unwrap_or(false) {
        return;
    }
    locked.set_value(false);
    let count = LOCKS.with(|locks| {
        locks.set(locks.get().saturating_sub(1));
        locks.get()
    });
    if count == 0 {
        set_body_class(false);
    }
}

fn set_body_class(on: bool) {
    let Some(body) = document().body() else { return };
    let classes = body.class_list();
    let _ = if on { classes.add_1(SCROLL_LOCK_CLASS) } else { classes.remove_1(SCROLL_LOCK_CLASS) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_ids_differ() {
        let a = unique_id("ferrox-dialog-title");
        let b = unique_id("ferrox-dialog-title");
        assert_ne!(a, b);
        assert!(a.starts_with("ferrox-dialog-title-"));
    }
}
