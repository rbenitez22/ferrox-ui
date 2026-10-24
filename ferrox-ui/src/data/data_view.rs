use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::layout::DataLayout;
use crate::{Alert, AlertSeverity, Button, ButtonFamily, ButtonSize, Spinner};

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
/// - **Keyed, rendered once:** a `<ul>` with one `<li>` per item from a keyed
///   `<For>`. An item whose key stays keeps its DOM node and its state (an
///   image that has loaded, text being typed) across updates, appends,
///   re-orders and layout changes. It is not rendered again either: to show
///   a record that changed, give it a key that changes with it (an id plus a
///   version), or have the template read signals.
/// - **States beside the list, never instead of items:** the first load (no
///   items yet) shows a spinner; a refresh keeps the items and marks the list
///   `aria-busy`. An error is an `Alert` above the list, with Retry when
///   `on_retry` is given. No items, not loading, no error: `empty_view` if
///   given, else the `empty` text.
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
    #[prop(into, default = "Retry".to_string())] retry_label: String,
    /// Shown when there is nothing to list.
    #[prop(into, default = "Nothing here yet.".into())] empty: Signal<String>,
    /// Shown instead of `empty`: a view, e.g. a line and an Add button.
    #[prop(optional, into)] empty_view: Option<ViewFn>,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let has_items = Memo::new(move |_| items.with(|items| !items.is_empty()));
    let first_load = move || loading.get() && !has_items.get();
    let is_empty = move || !loading.get() && error.get().is_none() && !has_items.get();

    let alert = move || {
        error.get().map(|message| {
            let retry_label = retry_label.clone();
            let retry = on_retry.map(|on_retry| view! {
                <Button family=ButtonFamily::Secondary size=ButtonSize::Sm class="data-view__retry" on_click=move |_| on_retry.run(())>
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
            None => view! { <p class="data-view__empty">{move || empty.get()}</p> }.into_any(),
        })
    };

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
                <For each=move || items.get() key=key_of let:record>
                    <li class="data-view__item">{item(record)}</li>
                </For>
            </ul>
            {empty_state}
        </div>
    }
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
}
