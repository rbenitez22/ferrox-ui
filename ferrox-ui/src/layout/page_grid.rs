use leptos::prelude::*;
use leptos::{component, view, IntoView};

/// How wide a page's content may grow.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PageWidth {
    /// 40rem: a form, a settings page, reading.
    Narrow,
    /// 64rem: most pages.
    #[default]
    Medium,
    /// 80rem: grids of cards, wide tables.
    Wide,
    /// The whole window.
    Full,
}

impl PageWidth {
    fn suffix(self) -> &'static str {
        match self {
            Self::Narrow => "narrow",
            Self::Medium => "medium",
            Self::Wide => "wide",
            Self::Full => "full",
        }
    }
}

/// Which side of the main column a `PageGrid`'s side column takes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SidePosition {
    Start,
    #[default]
    End,
}

/// The class list for a `.page-grid`.
pub fn page_grid_classes(width: PageWidth, side: Option<SidePosition>, extra: &str) -> String {
    let mut classes = vec!["page-grid".to_string(), format!("page-grid--{}", width.suffix())];
    match side {
        Some(SidePosition::End) => classes.push("page-grid--side-end".into()),
        Some(SidePosition::Start) => classes.push("page-grid--side-start".into()),
        None => {}
    }
    if !extra.is_empty() { classes.push(extra.to_string()); }
    classes.join(" ")
}

/// A page: its content capped at a width and centred, with the page's
/// padding, and an optional side column (filters, a summary, related
/// links) beside the main one, which moves under it on a phone. The main
/// column spaces its children (panels) by `--space-lg`, as does the side.
///
/// - The main column is the page's `<main>` landmark, the side an `<aside>`.
///   Inside an `AppShell`, which has its own `<main>`, the main column is a
///   plain block (the side stays an `<aside>`). `landmark=false` makes both
///   plain blocks; `true`, both landmarks.
/// - `side` is built once.
///
/// ```ignore
/// <PageGrid width=PageWidth::Wide side=move || view! { <Panel title="Filters">…</Panel> }>
///     <Panel title="Recipes" heading_level=HeadingLevel::H1>…</Panel>
/// </PageGrid>
/// ```
#[component]
pub fn PageGrid(
    #[prop(optional)] width: PageWidth,
    /// A side column beside the main one (under it on a phone).
    #[prop(optional, into)] side: Option<ViewFnOnce>,
    #[prop(optional)] side_position: SidePosition,
    /// The main column is `<main>` and the side `<aside>`. Unset: both,
    /// except no `<main>` inside an `AppShell`, which has its own.
    #[prop(optional)] landmark: Option<bool>,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The main column.
    children: Children,
) -> impl IntoView {
    let main_landmark = landmark.unwrap_or_else(|| super::shell().is_none());
    let side_landmark = landmark.unwrap_or(true);
    let has_side = side.as_ref().map(|_| side_position);
    let main = if main_landmark {
        view! { <main class="page-grid__main">{children()}</main> }.into_any()
    } else {
        view! { <div class="page-grid__main">{children()}</div> }.into_any()
    };
    let side = side.map(|side| {
        if side_landmark {
            view! { <aside class="page-grid__side">{side.run()}</aside> }.into_any()
        } else {
            view! { <div class="page-grid__side">{side.run()}</div> }.into_any()
        }
    });
    view! {
        <div class=move || page_grid_classes(width, has_side, &class.get().unwrap_or_default())>
            {main}
            {side}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(page_grid_classes(PageWidth::default(), None, ""), "page-grid page-grid--medium");
        assert_eq!(
            page_grid_classes(PageWidth::Wide, Some(SidePosition::Start), "x"),
            "page-grid page-grid--wide page-grid--side-start x"
        );
    }
}
