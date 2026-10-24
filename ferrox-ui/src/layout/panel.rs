use ferrox_icons::{Icon, PLUS};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::heading::{heading, HeadingLevel};
use crate::core::component_help;
use crate::ids::unique_id;
use crate::{IconButton, MaybeHelp};

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
pub fn panel_classes(compact: bool, flush: bool, tone: PanelTone, extra: &str) -> String {
    let mut classes = vec!["panel"];
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
///   `Toolbar`. `on_add` adds a `+` as the last action.
/// - `can_add` follows a permission: false removes the `+` (a user who may
///   not add is not shown the button).
/// - `help` puts a `?` right after the title.
/// - `compact` slims the band and the body for one panel among several;
///   `flush` drops the body's padding for a table or a grid; `tone` marks a
///   panel holding something to act on, or something that can't be undone.
///
/// ```ignore
/// <Panel title="Shopping Lists" heading_level=HeadingLevel::H1 on_add=move || new_list()>
///     …
/// </Panel>
/// <Panel title="Items" can_add=can_add_items on_add=move || new_item()
///        actions=move || view! { <IconButton title="Search items" …>…</IconButton> }>
///     …
/// </Panel>
/// ```
#[component]
pub fn Panel(
    #[prop(into)] title: Signal<String>,
    /// H2 by default; H1 for the page's own panel.
    #[prop(optional)] heading_level: HeadingLevel,
    /// Buttons at the right of the band, before the `+`.
    #[prop(optional, into)] actions: Option<ViewFn>,
    /// Adds a `+` as the last action, labelled `add_label`.
    #[prop(optional, into)] on_add: Option<UnsyncCallback<()>>,
    #[prop(into, default = "Add".to_string())] add_label: String,
    /// Whether the `+` shows. Hidden, not disabled.
    #[prop(into, default = true.into())] can_add: Signal<bool>,
    #[prop(optional)] compact: bool,
    #[prop(optional)] flush: bool,
    #[prop(optional)] tone: PanelTone,
    /// A help topic: a `?` after the title.
    #[prop(optional, into)] help: MaybeHelp,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let title_id = unique_id("ferrox-panel-title");
    let labelled_by = title_id.clone();
    let title_view = heading(heading_level, "panel__title", title_id, (move || title.get()).into_any());
    let help = component_help(help, "panel", title, Signal::stored(None), false, "panel__help");
    let add = on_add.map(|on_add| {
        let add_label = StoredValue::new(add_label);
        move || can_add.get().then(|| view! {
            <IconButton title=add_label.get_value() class="panel__add" on_click=move |_| on_add.run(())>
                <Icon icon=PLUS />
            </IconButton>
        })
    });

    view! {
        <section
            class=move || panel_classes(compact, flush, tone, &class.get().unwrap_or_default())
            aria-labelledby=labelled_by
        >
            <div class="panel__header">
                <div class="panel__heading">{title_view}{help}</div>
                <div class="panel__actions">{actions.map(|actions| actions.run())}{add}</div>
            </div>
            <div class="panel__body">{children()}</div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(panel_classes(false, false, PanelTone::Default, ""), "panel");
        assert_eq!(panel_classes(true, true, PanelTone::Warning, ""), "panel panel--compact panel--flush panel--warning");
        assert_eq!(panel_classes(false, false, PanelTone::Danger, "extra"), "panel panel--danger extra");
    }
}
