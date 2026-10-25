use leptos::context::Provider;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{FocusOptions, HtmlElement};

use super::{shell, HeadingLevel, PageGrid, PageWidth, Panel};
use crate::ids::unique_id;
use crate::MaybeHelp;

/// Marks the inside of a `Page`, to catch a second one in it.
#[derive(Clone, Copy)]
struct InPage;

/// A page: a [`PageGrid`] holding the page's own [`Panel`] (its `<h1>`,
/// with `actions` and `help` beside it, the children as its body), then
/// `more`. No styling of its own: the grid's and the panel's.
///
/// - **`document.title`** follows `title`: "{title} · {app}" inside an
///   `AppShell`, "{title}" outside. It's reactive: a detail page's title can
///   arrive with its data.
/// - **After a navigation** (inside an `AppShell`: a new path, not the
///   first load), focus moves to the page's `<h1>`, so a screen reader
///   announces the new page and Tab goes on from it.
/// - One per route: a `Page` inside a `Page` is a mistake.
///
/// ```ignore
/// <Page title="Recipes" width=PageWidth::Wide actions=move || view! { <IconButton …/> }>
///     <DataView … />
/// </Page>
/// ```
#[component]
pub fn Page(
    /// The page's title: its `<h1>`, and `document.title`.
    #[prop(into)] title: Signal<String>,
    #[prop(optional)] width: PageWidth,
    /// Buttons beside the title.
    #[prop(optional, into)] actions: Option<ViewFnOnce>,
    /// A side column (under the main one on a phone).
    #[prop(optional, into)] side: Option<ViewFnOnce>,
    /// A help topic: a `?` after the title.
    #[prop(optional, into)] help: MaybeHelp,
    /// More panels under the page's own (a settings page's sections).
    #[prop(optional, into)] more: Option<ViewFnOnce>,
    /// The page's own panel's body.
    children: Children,
) -> impl IntoView {
    if use_context::<InPage>().is_some() {
        leptos::logging::warn!("ferrox-ui: a Page inside a Page; one per route");
    }
    let shell = shell();
    let page_id = unique_id("ferrox-page");

    Effect::new(move |_| {
        let app = shell.map(|shell| shell.title.get());
        document().set_title(&document_title(&title.get(), app.as_deref()));
    });

    if let Some(shell) = shell
        && shell.arrived_by_navigation()
    {
        let page_id = page_id.clone();
        // After it's in the document.
        request_animation_frame(move || focus_heading(&page_id));
    }

    let panel = match actions {
        Some(actions) => view! {
            <Panel title=title heading_level=HeadingLevel::H1 actions=actions help=help>{children()}</Panel>
        }.into_any(),
        None => view! {
            <Panel title=title heading_level=HeadingLevel::H1 help=help>{children()}</Panel>
        }.into_any(),
    };
    let body = view! {
        <Provider value=InPage>
            {panel}
            {more.map(|more| more.run())}
        </Provider>
    };
    match side {
        Some(side) => view! { <PageGrid width=width side=side attr:data-ferrox-page=page_id>{body}</PageGrid> }.into_any(),
        None => view! { <PageGrid width=width attr:data-ferrox-page=page_id>{body}</PageGrid> }.into_any(),
    }
}

/// The browser tab's title: the page's, then the app's.
fn document_title(page: &str, app: Option<&str>) -> String {
    match app {
        Some(app) if !app.is_empty() => format!("{page} · {app}"),
        _ => page.to_string(),
    }
}

/// Focuses the page's `<h1>` (made focusable from script only), without
/// scrolling: the router already put the page at its top.
fn focus_heading(page_id: &str) {
    let selector = format!("[data-ferrox-page=\"{page_id}\"] h1");
    let Ok(Some(heading)) = document().query_selector(&selector) else { return };
    let Ok(heading) = heading.dyn_into::<HtmlElement>() else { return };
    let _ = heading.set_attribute("tabindex", "-1");
    let options = FocusOptions::new();
    options.set_prevent_scroll(true);
    let _ = heading.focus_with_options(&options);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_tab_title_is_the_page_then_the_app() {
        assert_eq!(document_title("Recipes", Some("Cookbook")), "Recipes · Cookbook");
        assert_eq!(document_title("Recipes", None), "Recipes");
        assert_eq!(document_title("Recipes", Some("")), "Recipes");
    }
}
