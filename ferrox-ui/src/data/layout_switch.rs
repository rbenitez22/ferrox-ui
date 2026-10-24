use ferrox_icons::{Icon, IconData, GALLERY_HORIZONTAL, LAYOUT_GRID, LIST, TAGS};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::layout::{CardSize, DataLayout};
use crate::IconButton;

/// The words of a [`DataLayoutSwitch`]: the group's name and each layout's.
/// Change some with `DataLayoutLabels { list: "Rows".into(), ..Default::default() }`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataLayoutLabels {
    pub group: String,
    pub grid_sm: String,
    pub grid: String,
    pub grid_lg: String,
    pub list: String,
    pub row: String,
    pub wrap: String,
}

impl Default for DataLayoutLabels {
    fn default() -> Self {
        Self {
            group: "Layout".into(),
            grid_sm: "Small grid".into(),
            grid: "Grid".into(),
            grid_lg: "Large grid".into(),
            list: "List".into(),
            row: "Strip".into(),
            wrap: "Cloud".into(),
        }
    }
}

impl DataLayoutLabels {
    pub fn for_layout(&self, layout: DataLayout) -> &str {
        match layout {
            DataLayout::Grid(CardSize::Sm) => &self.grid_sm,
            DataLayout::Grid(CardSize::Md) => &self.grid,
            DataLayout::Grid(CardSize::Lg) => &self.grid_lg,
            DataLayout::List => &self.list,
            DataLayout::Row => &self.row,
            DataLayout::Wrap => &self.wrap,
        }
    }
}

/// The icon a layout's button draws. Every grid size draws the same grid.
pub fn layout_icon(layout: DataLayout) -> IconData {
    match layout {
        DataLayout::Grid(_) => LAYOUT_GRID,
        DataLayout::List => LIST,
        DataLayout::Row => GALLERY_HORIZONTAL,
        DataLayout::Wrap => TAGS,
    }
}

/// Lets the user pick a `DataView`'s layout: a labelled group of icon
/// buttons, one per choice, the current one pressed (`aria-pressed`).
///
/// It sits wherever the app wants, typically a `Panel`'s actions.
/// `layout` is two-way: the app owns it, so remembering the choice (in
/// storage, in the URL) is the app's.
///
/// ```ignore
/// let layout = RwSignal::new(DataLayout::default());
/// <Panel title="Recipes" actions=move || view! { <DataLayoutSwitch layout=layout /> }>
///     <DataView items=recipes … layout=layout />
/// </Panel>
/// ```
#[component]
pub fn DataLayoutSwitch(
    layout: RwSignal<DataLayout>,
    /// The layouts offered, in order.
    #[prop(default = vec![DataLayout::Grid(CardSize::Md), DataLayout::List])] choices: Vec<DataLayout>,
    #[prop(optional)] labels: DataLayoutLabels,
    /// Extra classes on the group. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let buttons = choices.into_iter().map(|choice| view! {
        <IconButton
            label=labels.for_layout(choice).to_string()
            pressed=Signal::derive(move || layout.get() == choice)
            on_click=move |_| layout.set(choice)
        >
            <Icon icon=layout_icon(choice) />
        </IconButton>
    }).collect_view();

    view! {
        <div
            role="group"
            aria-label=labels.group.clone()
            class=move || match class.get() {
                Some(extra) if !extra.is_empty() => format!("data-layout-switch {extra}"),
                _ => "data-layout-switch".to_string(),
            }
        >
            {buttons}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_layout_has_its_word_and_icon() {
        let labels = DataLayoutLabels::default();
        assert_eq!(labels.for_layout(DataLayout::Grid(CardSize::Sm)), "Small grid");
        assert_eq!(labels.for_layout(DataLayout::default()), "Grid");
        assert_eq!(labels.for_layout(DataLayout::Row), "Strip");
        assert_eq!(layout_icon(DataLayout::Grid(CardSize::Lg)), LAYOUT_GRID);
        assert_eq!(layout_icon(DataLayout::Wrap), TAGS);
    }
}
