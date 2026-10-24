//! Data: a DataView of recipes driven by buttons and switches (update,
//! append, re-order, layouts, loading, error), a tag cloud, and a view whose
//! empty state is a view.

use ferrox_ui::{Button, ButtonFamily, Card, CardSize, DataLayout, DataView, Switch};
use leptos::prelude::*;

use crate::section::Section;

/// One record: an id (the key) and a name.
#[derive(Clone, Debug, PartialEq)]
struct Recipe {
    id: u32,
    name: String,
}

const RECIPE_NAMES: [&str; 8] = [
    "Shakshuka", "Ribollita", "Pad see ew", "Dal makhani",
    "Pierogi", "Feijoada", "Bibimbap", "Tarte tatin",
];

fn recipes() -> Vec<Recipe> {
    RECIPE_NAMES.iter().enumerate().map(|(i, name)| Recipe { id: i as u32 + 1, name: name.to_string() }).collect()
}

/// The layouts the demo offers, with their button labels.
const LAYOUTS: [(&str, DataLayout); 6] = [
    ("Grid S", DataLayout::Grid(CardSize::Sm)),
    ("Grid M", DataLayout::Grid(CardSize::Md)),
    ("Grid L", DataLayout::Grid(CardSize::Lg)),
    ("List", DataLayout::List),
    ("Row", DataLayout::Row),
    ("Wrap", DataLayout::Wrap),
];

#[component]
pub fn DataPage() -> impl IntoView {
    view! {
        <Recipes />
        <Tags />
        <Lists />
    }
}

/// The recipe cards: each has a note field, so a kept node is easy to see.
fn recipe_card(recipe: Recipe) -> impl IntoView {
    let note = format!("Note on {}", recipe.name);
    view! {
        <Card title=recipe.name.clone() compact=true>
            <input class="form-input" aria-label=note.clone() placeholder=note />
        </Card>
    }
}

#[component]
fn Recipes() -> impl IntoView {
    let items = RwSignal::new(recipes());
    let layout = RwSignal::new(DataLayout::default());
    let loading = RwSignal::new(false);
    let failing = RwSignal::new(false);
    let retries = RwSignal::new(0_u32);
    let next_id = RwSignal::new(RECIPE_NAMES.len() as u32 + 1);
    let error = Signal::derive(move || failing.get().then(|| "Couldn't load the recipes.".to_string()));

    let update = move |_| items.update(|list| *list = list.clone());
    let append = move |_| {
        let id = next_id.get_untracked();
        next_id.set(id + 1);
        items.update(|list| list.push(Recipe { id, name: format!("New recipe {id}") }));
    };
    let reverse = move |_| items.update(|list| list.reverse());
    let empty_out = move |_| items.set(Vec::new());
    let restore = move |_| items.set(recipes());

    view! {
        <Section title="Data view">
            <div class="demo-row" role="group" aria-label="Layout">
                {LAYOUTS.into_iter().map(|(name, choice)| view! {
                    <Button family=ButtonFamily::Secondary on_click=move |_| layout.set(choice)>{name}</Button>
                }).collect_view()}
            </div>
            <div class="demo-row">
                <Button family=ButtonFamily::Secondary on_click=update>"Update"</Button>
                <Button family=ButtonFamily::Secondary on_click=append>"Append"</Button>
                <Button family=ButtonFamily::Secondary on_click=reverse>"Reverse"</Button>
                <Button family=ButtonFamily::Secondary on_click=empty_out>"Empty"</Button>
                <Button family=ButtonFamily::Secondary on_click=restore>"Restore"</Button>
            </div>
            <div class="demo-row">
                <Switch checked=loading label="Loading" />
                <Switch checked=failing label="Failing" />
            </div>
            <DataView
                items=items
                key_of=|r: &Recipe| r.id.to_string()
                item=recipe_card
                label="Recipes"
                layout=layout
                loading=loading
                loading_label="Loading recipes…"
                error=error
                on_retry=move || retries.update(|n| *n += 1)
                empty="No recipes yet."
            />
            <dl class="demo-echo">
                <dt>"layout"</dt><dd data-echo="layout">{move || format!("{:?}", layout.get())}</dd>
                <dt>"retries"</dt><dd data-echo="retries">{move || retries.get().to_string()}</dd>
            </dl>
        </Section>
    }
}

/// A tag cloud: `Wrap`, items sized to their content.
#[component]
fn Tags() -> impl IntoView {
    let tags = Signal::stored(
        ["vegetarian", "quick", "one pot", "spicy", "baking", "make ahead", "freezer friendly", "street food", "dessert", "brunch"]
            .map(String::from)
            .to_vec(),
    );
    view! {
        <Section title="Tag cloud">
            <DataView
                items=tags
                key_of=|t: &String| t.clone()
                item=|t: String| view! { <span class="demo-tag">{t}</span> }
                label="Tags"
                layout=DataLayout::Wrap
            />
        </Section>
    }
}

/// An empty state that is a view: a line and a button that adds the first.
#[component]
fn Lists() -> impl IntoView {
    let lists = RwSignal::new(Vec::<String>::new());
    let add = move |_| lists.update(|l| l.push(format!("List {}", l.len() + 1)));
    view! {
        <Section title="Empty view">
            <DataView
                items=lists
                key_of=|l: &String| l.clone()
                item=|l: String| view! { <span>{l}</span> }
                label="Shopping lists"
                layout=DataLayout::List
                empty_view=move || view! {
                    <p>"No shopping lists yet."</p>
                    <Button on_click=add>"Add a list"</Button>
                }
            />
        </Section>
    }
}
