//! Contents built once, on first showing, then kept: a collapsible
//! `Panel`'s body, a menu's items. The caller's view is run-once
//! (`ViewFnOnce`), so closures inside it may move values; the `Show` that
//! mounts it is gated by a latch that turns true once and never back.

use leptos::prelude::*;

/// Wraps a run-once view for a latch-gated `Show`: the first call runs it;
/// a later call (which the latch never makes) renders nothing.
pub(crate) fn built_once(view: ViewFnOnce) -> impl Fn() -> AnyView + Copy + Send + Sync + 'static {
    let slot = StoredValue::new_local(Some(view));
    move || {
        slot.try_update_value(Option::take)
            .flatten()
            .map(ViewFnOnce::run)
            .unwrap_or_else(|| ().into_any())
    }
}
