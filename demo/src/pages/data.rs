//! Data: a DataView of recipes driven by buttons and switches (update,
//! append, re-order, layouts, loading, error), a tag cloud, a view whose
//! empty state is a view, a paged list, and two infinite feeds (in a scroll
//! box, and at the page's foot, scrolled by the window).

use std::time::Duration;

use ferrox_ui::{Button, ButtonFamily, Card, CardSize, DataLayout, DataLayoutSwitch, DataView, Pagination, Switch};
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

/// Every layout, for the recipes' switch.
const LAYOUTS: [DataLayout; 6] = [
    DataLayout::Grid(CardSize::Sm),
    DataLayout::Grid(CardSize::Md),
    DataLayout::Grid(CardSize::Lg),
    DataLayout::List,
    DataLayout::Row,
    DataLayout::Wrap,
];

#[component]
pub fn DataPage() -> impl IntoView {
    view! {
        <Recipes />
        <Tags />
        <Lists />
        <Paged />
        <Feed title="Feed in a box" name="box" page=2 total=20 layout=DataLayout::List boxed=true />
        <Feed title="Feed" name="page" page=6 total=24 layout=DataLayout::List boxed=false />
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
            <div class="demo-row">
                <DataLayoutSwitch layout=layout choices=LAYOUTS.to_vec() />
                <Button family=ButtonFamily::Secondary on_click=move |_| layout.set(DataLayout::List)>"Choose List from code"</Button>
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

/// Items per page in the paged list.
const PER_PAGE: usize = 10;

/// A paged list: `Pagination` under a `DataView` showing that page's slice,
/// with buttons that change the page count and set the page from code.
#[component]
fn Paged() -> impl IntoView {
    let page = RwSignal::new(1_usize);
    let page_count = RwSignal::new(20_usize);
    let slice = Signal::derive(move || {
        let first = (page.get() - 1) * PER_PAGE + 1;
        (first..first + PER_PAGE).filter(|n| *n <= page_count.get() * PER_PAGE).collect::<Vec<usize>>()
    });
    let set_count = move |count: usize| {
        page_count.set(count);
        page.set(1);
    };
    view! {
        <Section title="Pagination">
            <div class="demo-row">
                <Button family=ButtonFamily::Secondary on_click=move |_| set_count(20)>"20 pages"</Button>
                <Button family=ButtonFamily::Secondary on_click=move |_| set_count(5)>"5 pages"</Button>
                <Button family=ButtonFamily::Secondary on_click=move |_| set_count(1)>"1 page"</Button>
                <Button family=ButtonFamily::Secondary on_click=move |_| page.set(20)>"Go to page 20 from code"</Button>
            </div>
            <DataView
                items=slice
                key_of=|n: &usize| n.to_string()
                item=|n: usize| view! { <span>{format!("Item {n}")}</span> }
                label="Paged items"
                layout=DataLayout::Wrap
            />
            <Pagination page=page page_count=page_count label="Item pages" />
            <dl class="demo-echo">
                <dt>"page"</dt><dd data-echo="page">{move || page.get().to_string()}</dd>
            </dl>
        </Section>
    }
}

/// An infinite feed of numbered entries: `page` at a time, `total` in all,
/// each page arriving 300ms after it is asked for. `boxed` puts it in a
/// scrolling box. Echoes how often it was asked for more. Its layout switch
/// offers the default choices.
#[component]
fn Feed(title: &'static str, name: &'static str, page: usize, total: usize, layout: DataLayout, boxed: bool) -> impl IntoView {
    let layout = RwSignal::new(layout);
    let entries = RwSignal::new((1..=page).collect::<Vec<usize>>());
    let loading_more = RwSignal::new(false);
    let calls = RwSignal::new(0_u32);
    let has_more = Signal::derive(move || entries.with(Vec::len) < total);
    let more = move || {
        calls.update(|n| *n += 1);
        loading_more.set(true);
        set_timeout(move || {
            entries.update(|e| {
                let from = e.len() + 1;
                e.extend(from..(from + page).min(total + 1));
            });
            loading_more.set(false);
        }, Duration::from_millis(300));
    };
    view! {
        <Section title=title>
            <DataLayoutSwitch layout=layout />
            <div class=if boxed { "demo-feed-box" } else { "" } data-feed=name>
                <DataView
                    items=entries
                    key_of=|n: &usize| n.to_string()
                    item=|n: usize| view! { <Card title=format!("Entry {n}") compact=true><p>"One more."</p></Card> }
                    label=title
                    layout=layout
                    on_more=more
                    has_more=has_more
                    loading_more=loading_more
                />
            </div>
            <dl class="demo-echo">
                <dt>"calls"</dt><dd data-echo=format!("{name}-calls")>{move || calls.get().to_string()}</dd>
            </dl>
        </Section>
    }
}
