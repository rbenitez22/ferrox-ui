//! Recipe: a detail page. The record's own panel (its name as the page's
//! title; Edit, with Duplicate and Delete behind its chevron), its facts in
//! a side column, and what happens when the record isn't there (still
//! loading, or deleted). Deleting asks, then goes back to the list.

use ferrox_ui::{
    ButtonFamily, ConfirmDialog, Fact, Facts, MenuDivider, MenuItem, Page, PageGrid, PageWidth, Panel, Spinner, SplitButton,
};
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};

use super::recipe_dialog::RecipeDialog;
use super::store::{use_store, Recipe};

#[component]
pub fn DetailPage() -> impl IntoView {
    let store = use_store();
    let navigate = use_navigate();
    let params = use_params_map();
    let id = Memo::new(move |_| params.with(|p| p.get("id").and_then(|id| id.parse::<u32>().ok())).unwrap_or(0));
    // Tracked: edits made anywhere show here.
    let recipe = Memo::new(move |_| store.find(id.get()));

    let dialog_open = RwSignal::new(false);
    let editing = RwSignal::new(None::<Recipe>);
    let edit = move || {
        editing.set(recipe.get_untracked());
        dialog_open.set(true);
    };
    // A duplicate is a new recipe (id 0) with the same fields, opened for editing.
    let duplicate = move || {
        editing.set(recipe.get_untracked().map(|r| Recipe { id: 0, name: format!("{} (copy)", r.name), ..r }));
        dialog_open.set(true);
    };
    let go_to = {
        let navigate = navigate.clone();
        move |saved: Recipe| navigate(&format!("/recipes/{}", saved.id), Default::default())
    };

    let confirming = RwSignal::new(false);
    let deleting = RwSignal::new(false);
    let delete = move || {
        deleting.set(true);
        let navigate = navigate.clone();
        store.remove(id.get_untracked(), move || {
            deleting.set(false);
            confirming.set(false);
            navigate("/recipes", Default::default());
        });
    };

    let name = Signal::derive(move || recipe.get().map(|r| r.name).unwrap_or_default());
    let facts = move || {
        recipe.get().map(|r| {
            let units = if store.settings.units.get() == "us" { "US cups and ounces" } else { "metric" };
            view! {
                <Facts>
                    <Fact label="Time">{format!("{} minutes", r.minutes)}</Fact>
                    <Fact label="Units">{units}</Fact>
                </Facts>
            }
        })
    };

    view! {
        <Show
            when=move || recipe.get().is_some()
            fallback=move || view! {
                {move || if store.loading.get() {
                    view! { <PageGrid width=PageWidth::Narrow><Spinner label="Loading the recipe" /></PageGrid> }.into_any()
                } else {
                    view! {
                        <Page title="Not found" width=PageWidth::Narrow>
                            <p>"There's no such recipe; it may have been deleted."</p>
                            <a href="/recipes">"All recipes"</a>
                        </Page>
                    }.into_any()
                }}
            }
        >
            <Page
                title=name
                side=move || view! { <Panel title="At a glance" compact=true>{facts}</Panel> }
                actions=move || view! {
                    <SplitButton text="Edit" family=ButtonFamily::Secondary on_click=move || edit() menu_label="More recipe actions">
                        <MenuItem on_select=move || duplicate()>"Duplicate"</MenuItem>
                        <MenuDivider />
                        <MenuItem danger=true on_select=move || confirming.set(true)>"Delete"</MenuItem>
                    </SplitButton>
                }
            >
                <p>{move || recipe.get().map(|r| r.notes).unwrap_or_default()}</p>
                <a href="/recipes">"All recipes"</a>
            </Page>
        </Show>
        <RecipeDialog open=dialog_open editing=editing on_saved=go_to />
        <ConfirmDialog open=confirming title="Delete recipe?" confirm_label="Delete" saving=deleting on_confirm=delete>
            <p>{move || format!("This deletes “{}”.", name.get())}</p>
        </ConfirmDialog>
    }
}
