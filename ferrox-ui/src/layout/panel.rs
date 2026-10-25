use ferrox_icons::{Icon, CHEVRON_RIGHT};
use crate::once::built_once;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::heading::{heading, HeadingLevel};
use crate::core::component_help;
use crate::ids::unique_id;
use crate::MaybeHelp;

// ---------------------------------------------------------------------------
// PanelTone
// ---------------------------------------------------------------------------

/// What a panel holds, said in its colour.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum PanelTone {
    #[default]
    Default,
    /// Something to act on: the band and border in the warning colour.
    Warning,
    /// Something that can't be undone: the danger colour, the title too.
    Danger,
}

impl PanelTone {
    pub fn class(self) -> Option<&'static str> {
        match self {
            Self::Default => None,
            Self::Warning => Some("panel--warning"),
            Self::Danger  => Some("panel--danger"),
        }
    }
}

/// The class list for a `.panel`.
pub fn panel_classes(collapsible: bool, compact: bool, flush: bool, tone: PanelTone, extra: &str) -> String {
    let mut classes = vec!["panel"];
    if collapsible { classes.push("panel--collapsible"); }
    if compact { classes.push("panel--compact"); }
    if flush { classes.push("panel--flush"); }
    if let Some(tone) = tone.class() { classes.push(tone); }
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

// ---------------------------------------------------------------------------
// Panel
// ---------------------------------------------------------------------------

/// A titled box: a header band with the title (a real heading) and the
/// actions, then the body. For a page's own title (`heading_level` H1) and for
/// a section inside a page alike.
///
/// - The title sits on the left and the actions on the right, as in a
///   `Toolbar`. What the actions are (add, search, export) is the app's.
/// - `help` puts a `?` right after the title (after the toggle, on a
///   collapsible panel).
/// - `collapsible`: the title is a button inside the heading that opens and
///   closes the body. It starts shut unless `start_expanded`; pass
///   `expanded` to own the state. The body isn't rendered until it's first
///   opened (a shut panel loads nothing), then stays and is `hidden` while
///   shut, so typed text survives. While shut, `summary` says one line in
///   the band, and the actions are hidden.
/// - `compact` slims the band and the body for one panel among several;
///   `flush` drops the body's padding for a table or a grid; `tone` marks a
///   panel holding something to act on, or something that can't be undone.
///
/// ```ignore
/// <Panel title="Shopping Lists" heading_level=HeadingLevel::H1>
///     …
/// </Panel>
/// <Panel title="Items" actions=move || view! {
///     <IconButton label="Search items" …>…</IconButton>
///     <Show when=move || can_add.get()><IconButton label="Add item" …>…</IconButton></Show>
/// }>
///     …
/// </Panel>
/// ```
#[component]
pub fn Panel(
    #[prop(into)] title: Signal<String>,
    /// H2 by default; H1 for the page's own panel.
    #[prop(optional)] heading_level: HeadingLevel,
    /// Buttons at the right of the band.
    #[prop(optional, into)] actions: Option<ViewFnOnce>,
    /// The title is a button that opens and closes the body.
    #[prop(optional)] collapsible: bool,
    /// Where a collapsible panel starts: shut unless this is true.
    #[prop(optional)] start_expanded: bool,
    /// Two-way open state, owned by the app (overrides `start_expanded`).
    #[prop(optional)] expanded: Option<RwSignal<bool>>,
    /// Said in the band while a collapsible panel is shut: "3 overdue".
    #[prop(optional, into)] summary: MaybeProp<String>,
    #[prop(optional)] compact: bool,
    #[prop(optional)] flush: bool,
    #[prop(optional)] tone: PanelTone,
    /// A help topic: a `?` after the title.
    #[prop(optional, into)] help: MaybeHelp,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let title_id = unique_id("ferrox-panel-title");
    let body_id = unique_id("ferrox-panel-body");
    let labelled_by = title_id.clone();
    let expanded = expanded.unwrap_or_else(|| RwSignal::new(start_expanded || !collapsible));
    let open = Signal::derive(move || !collapsible || expanded.get());
    let opened = opened_once(open);
    // Built on first opening, then kept: closures inside may move values.
    let body = built_once(ViewFnOnce::from(children));
    let title_view = if collapsible {
        heading(heading_level, "panel__title", None, panel_toggle(title, title_id, body_id.clone(), expanded, summary))
    } else {
        heading(heading_level, "panel__title", Some(title_id), (move || title.get()).into_any())
    };
    let (help, help_id) = component_help(help, "panel", title, Signal::stored(None), false, "panel__help", false).unzip();

    view! {
        <section
            class=move || panel_classes(collapsible, compact, flush, tone, &class.get().unwrap_or_default())
            data-help=help_id
            aria-labelledby=labelled_by
        >
            <div class="panel__header">
                <div class="panel__heading">{title_view}{help}</div>
                <div class="panel__actions" hidden=move || !open.get()>{actions.map(|actions| actions.run())}</div>
            </div>
            <div class="panel__body" id=body_id hidden=move || !open.get()>
                <Show when=move || opened.get()>{body()}</Show>
            </div>
        </section>
    }
}

/// A collapsible panel's title: a button (inside the heading) holding the
/// title, the summary while shut, and the chevron. The section is named by
/// the title alone, so the summary never becomes part of its name.
fn panel_toggle(
    title: Signal<String>,
    title_id: String,
    body_id: String,
    expanded: RwSignal<bool>,
    summary: MaybeProp<String>,
) -> AnyView {
    let summary_view = move || {
        if expanded.get() { return None; }
        summary.get().filter(|s| !s.is_empty()).map(|s| view! { <span class="panel__summary">{s}</span> })
    };
    view! {
        <button
            type="button"
            class="panel__toggle"
            aria-expanded=move || expanded.get().to_string()
            aria-controls=body_id
            on:click=move |_| expanded.update(|e| *e = !*e)
        >
            <span class="panel__title-text" id=title_id>{move || title.get()}</span>
            {summary_view}
            <Icon icon=CHEVRON_RIGHT class="panel__chevron" />
        </button>
    }
    .into_any()
}

/// True from the first time `open` is true, and from then on: the body is
/// built on first opening and kept.
fn opened_once(open: Signal<bool>) -> Memo<bool> {
    Memo::new(move |before: Option<&bool>| before.copied().unwrap_or(false) || open.get())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(panel_classes(false, false, false, PanelTone::Default, ""), "panel");
        assert_eq!(panel_classes(false, true, true, PanelTone::Warning, ""), "panel panel--compact panel--flush panel--warning");
        assert_eq!(panel_classes(true, false, false, PanelTone::Danger, "extra"), "panel panel--collapsible panel--danger extra");
    }
}
