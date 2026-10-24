use std::collections::HashMap;

use leptos::html;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

use super::layout::DataLayout;
use crate::{Alert, AlertSeverity, Button, ButtonFamily, ButtonSize, Spinner, SpinnerKind};

/// How far outside the viewport the end of the list may be and still count
/// as reached, so the next page starts loading before the user gets there.
const MORE_MARGIN: &str = "300px";

/// The class list for a `.data-view`.
pub fn data_view_classes(layout: DataLayout, extra: &str) -> String {
    let mut classes = vec!["data-view", layout.class()];
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

/// A record list rendered through one item template and laid out as the
/// screen needs: a grid of cards, a list of rows, a sideways strip, or
/// wrapping chips (`layout`).
///
/// - **Keyed:** a `<ul>` with one `<li>` per item from a keyed `<For>`.
///   `key_of` is the item's identity (an id). An item whose key and value
///   stay keeps its DOM node and its state (an image that has loaded, text
///   being typed) across updates, appends, re-orders and layout changes. An
///   item whose value changed (compared with `PartialEq`) is drawn again in
///   its place; the others are left alone.
/// - **States beside the list, never instead of items:** the first load (no
///   items yet) shows a spinner; a refresh keeps the items and marks the list
///   `aria-busy`. An error is an `Alert` above the list, with Retry when
///   `on_retry` is given. No items, not loading, no error: `empty_view` if
///   given, else the `empty_label` text.
/// - **Infinite scroll** (`on_more`): a marker after the last item calls
///   `on_more` when it comes within 300px of the viewport and `has_more`:
///   once per sighting. It doesn't care what scrolls (inside an `overflow:
///   auto` box it counts once the marker shows there). When a page lands
///   and the marker is still in sight, it calls again, so a short page pulls
///   the next. Never while `loading` or `loading_more`; while `loading_more`
///   the marker shows a spinner.
///
/// It doesn't fetch, sort or filter: whatever loads `items` owns that.
///
/// ```ignore
/// <DataView
///     items=recipes
///     key_of=|r: &Recipe| r.id.clone()
///     item=|r: Recipe| view! { <Card title=r.name.clone() href=format!("/recipes/{}", r.id)>…</Card> }
///     label="Recipes"
///     loading=loading
///     error=error
///     on_retry=move || reload()
/// />
/// ```
#[component]
pub fn DataView<T, IV>(
    #[prop(into)] items: Signal<Vec<T>>,
    /// Stable identity per item.
    key_of: impl Fn(&T) -> String + Clone + Send + Sync + 'static,
    /// Renders one item, usually a `Card` (grid) or a row (list).
    item: impl Fn(T) -> IV + Clone + Send + Sync + 'static,
    /// The list's accessible name: "Recipes".
    #[prop(into)] label: Signal<String>,
    #[prop(into, default = DataLayout::default().into())] layout: Signal<DataLayout>,
    /// Loading: a spinner while there are no items yet, `aria-busy` after.
    #[prop(optional, into)] loading: Signal<bool>,
    #[prop(into, default = "Loading…".into())] loading_label: Signal<String>,
    /// Why loading failed: an alert above the list.
    #[prop(optional, into)] error: MaybeProp<String>,
    /// Adds Retry to the error.
    #[prop(optional, into)] on_retry: Option<UnsyncCallback<()>>,
    #[prop(into, default = "Retry".into())] retry_label: Signal<String>,
    /// Shown when there is nothing to list.
    #[prop(into, default = "Nothing here yet.".into())] empty_label: Signal<String>,
    /// Shown instead of `empty_label`: a view, e.g. a line and an Add button.
    #[prop(optional, into)] empty_view: Option<ViewFn>,
    /// Called when the end of the list comes into view and `has_more`.
    #[prop(optional, into)] on_more: Option<UnsyncCallback<()>>,
    #[prop(optional, into)] has_more: Signal<bool>,
    /// A page is on its way: a spinner at the end, and no new calls.
    #[prop(optional, into)] loading_more: Signal<bool>,
    #[prop(into, default = "Loading more…".into())] loading_more_label: Signal<String>,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let has_items = Memo::new(move |_| items.with(|items| !items.is_empty()));
    let drawn = draw_keys(items, key_of);
    let first_load = move || loading.get() && !has_items.get();
    let is_empty = move || !loading.get() && error.get().is_none() && !has_items.get();

    let alert = move || {
        error.get().map(|message| {
            let retry = on_retry.map(|on_retry| view! {
                <Button family=ButtonFamily::Secondary size=ButtonSize::Sm class="data-view__retry" on_click=move || on_retry.run(())>
                    {retry_label}
                </Button>
            });
            view! {
                <Alert severity=AlertSeverity::Error class="data-view__error">
                    <span>{message}</span>
                    {retry}
                </Alert>
            }
        })
    };
    let spinner = move || first_load().then(|| view! { <Spinner label=loading_label /> });
    let empty_state = move || {
        is_empty().then(|| match &empty_view {
            Some(view) => view! { <div class="data-view__empty">{view.run()}</div> }.into_any(),
            None => view! { <p class="data-view__empty">{move || empty_label.get()}</p> }.into_any(),
        })
    };

    let more = on_more.map(|on_more| {
        let marker = NodeRef::<html::Li>::new();
        infinite_scroll(marker, on_more, has_more, loading, loading_more, Signal::derive(move || items.with(Vec::len)));
        view! {
            <li class="data-view__more" role="none" node_ref=marker>
                {move || loading_more.get().then(|| view! { <Spinner kind=SpinnerKind::Inline label=loading_more_label /> })}
            </li>
        }
    });

    view! {
        <div class=move || data_view_classes(layout.get(), &class.get().unwrap_or_default())>
            {alert}
            {spinner}
            <ul
                class="data-view__list"
                aria-label=move || label.get()
                aria-busy=move || (loading.get() && has_items.get()).then_some("true")
                hidden=move || !has_items.get()
            >
                <For each=move || drawn.get() key=|(key, _)| key.clone() let:entry>
                    <li class="data-view__item">{item(entry.1)}</li>
                </For>
                {more}
            </ul>
            {empty_state}
        </div>
    }
}

/// What the keyed `<For>` draws: each item with a key made of its identity
/// (`key_of`) and a generation, which goes up when the item's value differs
/// from the last one drawn under that identity. So an unchanged item keeps
/// its node, and a changed one is drawn again.
fn draw_keys<T>(items: Signal<Vec<T>>, key_of: impl Fn(&T) -> String + Send + Sync + 'static) -> Memo<Vec<(String, T)>>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    // The last value and generation drawn under each identity.
    let seen = StoredValue::new(HashMap::<String, (T, u64)>::new());
    Memo::new(move |_| {
        items.with(|items| {
            seen.try_update_value(|seen| {
                let mut next = HashMap::with_capacity(items.len());
                let drawn = items
                    .iter()
                    .map(|item| {
                        let id = key_of(item);
                        let generation = next_generation(seen.get(&id), item);
                        next.insert(id.clone(), (item.clone(), generation));
                        (draw_key(&id, generation), item.clone())
                    })
                    .collect();
                *seen = next;
                drawn
            })
            .unwrap_or_default()
        })
    })
}

/// An item's generation: the same while its value is, one more when it changed.
pub(crate) fn next_generation<T: PartialEq>(last: Option<&(T, u64)>, item: &T) -> u64 {
    match last {
        Some((value, generation)) if value == item => *generation,
        Some((_, generation)) => generation + 1,
        None => 0,
    }
}

/// The `<For>` key for an identity at a generation.
pub(crate) fn draw_key(id: &str, generation: u64) -> String {
    format!("{id}\u{1f}{generation}")
}

/// Infinite scroll: an `IntersectionObserver` on the end marker. Each
/// report with the marker within [`MORE_MARGIN`] of the viewport asks for at
/// most one page. When a page lands the marker is observed afresh, which
/// makes the browser report again from the new layout: a marker still in
/// sight (a short page) asks for the next one. The observer is disconnected
/// when the view unmounts.
fn infinite_scroll(
    marker: NodeRef<html::Li>,
    on_more: UnsyncCallback<()>,
    has_more: Signal<bool>,
    loading: Signal<bool>,
    loading_more: Signal<bool>,
    count: Signal<usize>,
) {
    let observer = StoredValue::new_local(None::<IntersectionObserver>);
    // Set on every report, equal or not, so each one is acted on.
    let in_sight = RwSignal::new(false);

    Effect::new(move |_| {
        let Some(marker) = marker.get() else { return };
        if observer.with_value(Option::is_some) {
            return;
        }
        let on_report = Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
            if let Some(entry) = entries.iter().last().and_then(|e| e.dyn_into::<IntersectionObserverEntry>().ok()) {
                // The view may be gone by the time the browser reports.
                let _ = in_sight.try_set(entry.is_intersecting());
            }
        });
        let options = IntersectionObserverInit::new();
        options.set_root_margin(MORE_MARGIN);
        let Ok(created) = IntersectionObserver::new_with_options(on_report.into_js_value().unchecked_ref(), &options) else { return };
        created.observe(&marker);
        observer.set_value(Some(created));
    });

    Effect::new(move |_| {
        if in_sight.get() && untrack(|| should_call(has_more.get(), loading.get(), loading_more.get())) {
            untrack(|| on_more.run(()));
        }
    });

    // A page landed: ask the browser where the marker is now.
    Effect::new(move |previous: Option<usize>| {
        let now = count.get();
        if previous.is_some_and(|previous| previous != now) {
            observer.with_value(|o| {
                if let (Some(o), Some(m)) = (o, marker.get_untracked()) {
                    o.unobserve(&m);
                    o.observe(&m);
                }
            });
        }
        now
    });

    on_cleanup(move || {
        let _ = observer.try_with_value(|o| o.as_ref().map(IntersectionObserver::disconnect));
    });
}

/// Whether a sighting of the end marker may ask for a page.
pub(crate) fn should_call(has_more: bool, loading: bool, loading_more: bool) -> bool {
    has_more && !loading && !loading_more
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CardSize;

    #[test]
    fn classes() {
        assert_eq!(data_view_classes(DataLayout::default(), ""), "data-view data-view--grid-md");
        assert_eq!(data_view_classes(DataLayout::Grid(CardSize::Sm), "recipes"), "data-view data-view--grid-sm recipes");
        assert_eq!(data_view_classes(DataLayout::Wrap, ""), "data-view data-view--wrap");
    }

    #[test]
    fn an_item_is_redrawn_only_when_its_value_changed() {
        assert_eq!(next_generation::<&str>(None, &"a"), 0);
        assert_eq!(next_generation(Some(&("a", 3)), &"a"), 3);
        assert_eq!(next_generation(Some(&("a", 3)), &"b"), 4);
        assert_ne!(draw_key("7", 0), draw_key("7", 1));
    }

    #[test]
    fn a_sighting_asks_only_when_more_may_be_fetched() {
        assert!(should_call(true, false, false));
        assert!(!should_call(false, false, false));
        assert!(!should_call(true, true, false));
        assert!(!should_call(true, false, true));
    }
}
