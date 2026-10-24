//! Recipe: a list page. A page panel (its title, a layout switch and an
//! add button), the records as cards in a `DataView`, each card a link to
//! its record and its actions in a row menu, the add / edit dialog, and a
//! confirmation before deleting. Loading, error (with Retry) and empty
//! states come from the `DataView`.

use ferrox_icons::{Icon, DELETE, EDIT, MORE_VERTICAL, PLUS};
use ferrox_ui::{
    Align, Button, Card, ConfirmDialog, DataLayout, DataLayoutSwitch, DataView, Gap, HeadingLevel, IconButton, Menu, MenuDivider,
    MenuItem, MenuTrigger, PageGrid, PageWidth, Panel, Stack,
};
use leptos::prelude::*;

use super::recipe_dialog::RecipeDialog;
use super::store::{use_store, Recipe};

#[component]
pub fn ListPage() -> impl IntoView {
    let store = use_store();
    let layout = RwSignal::new(DataLayout::default());

    // Adding and editing share one dialog: `editing` says which.
    let dialog_open = RwSignal::new(false);
    let editing = RwSignal::new(None::<Recipe>);
    let add = move || {
        editing.set(None);
        dialog_open.set(true);
    };
    let edit = move |recipe: Recipe| {
        editing.set(Some(recipe));
        dialog_open.set(true);
    };

    // Deleting asks first (unless the user turned that off in settings), and
    // the confirmation shows its saving state until the store answers.
    let confirming = RwSignal::new(false);
    let deleting = RwSignal::new(false);
    let to_delete = RwSignal::new(None::<Recipe>);
    let delete = move || {
        let Some(recipe) = to_delete.get_untracked() else { return };
        deleting.set(true);
        store.remove(recipe.id, move || {
            deleting.set(false);
            confirming.set(false);
        });
    };
    let ask_delete = move |recipe: Recipe| {
        to_delete.set(Some(recipe));
        if store.settings.confirm_delete.get_untracked() {
            confirming.set(true);
        } else {
            delete();
        }
    };

    // One card per recipe: the whole card links to it, its actions in a menu.
    let card = move |recipe: Recipe| {
        let minutes = recipe.minutes;
        let label = format!("{} actions", recipe.name);
        let edited = recipe.clone();
        view! {
            <Card
                title=recipe.name.clone()
                href=format!("/recipes/{}", recipe.id)
                actions=move || view! {
                    <Menu label=label trigger=MenuTrigger::icon(MORE_VERTICAL, "More actions")>
                        <MenuItem icon=EDIT on_select=move || edit(edited.clone())>"Edit"</MenuItem>
                        <MenuDivider />
                        <MenuItem icon=DELETE danger=true on_select=move || ask_delete(recipe.clone())>"Delete"</MenuItem>
                    </Menu>
                }
            >
                {move || store.settings.show_minutes.get().then(|| view! { <p>{format!("{minutes} min")}</p> })}
            </Card>
        }
    };

    view! {
        <PageGrid width=PageWidth::Wide>
            <Panel
                title="Recipes"
                heading_level=HeadingLevel::H1
                actions=move || view! {
                    <DataLayoutSwitch layout=layout />
                    <IconButton label="Add recipe" on_click=move || add()><Icon icon=PLUS /></IconButton>
                }
            >
                <DataView
                    items=store.recipes
                    key_of=|recipe: &Recipe| recipe.id.to_string()
                    item=card
                    label="Recipes"
                    layout=layout
                    loading=store.loading
                    loading_label="Loading recipes…"
                    error=store.error
                    on_retry=move || store.load()
                    empty_view=move || view! {
                        <Stack gap=Gap::Sm align=Align::Start>
                            <p>"No recipes yet."</p>
                            <Button on_click=move || add()>"Add a recipe"</Button>
                        </Stack>
                    }
                />
            </Panel>
        </PageGrid>
        <RecipeDialog open=dialog_open editing=editing />
        <ConfirmDialog open=confirming title="Delete recipe?" confirm_label="Delete" saving=deleting on_confirm=delete>
            <p>{move || format!("This deletes “{}”.", to_delete.get().map(|r| r.name).unwrap_or_default())}</p>
        </ConfirmDialog>
    }
}
