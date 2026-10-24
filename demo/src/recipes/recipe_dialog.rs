//! Recipe: a form in a dialog. Adds a recipe, or edits one: the same
//! fields, filled from the recipe each time it opens. Errors show under
//! their field, and only once the user has tried to save. Save shows its
//! saving state until the store answers, then closes the dialog.

use ferrox_ui::{Dialog, DialogAction, Field, Gap, Input, InputKind, Stack, Textarea};
use leptos::prelude::*;

use super::store::{use_store, Recipe};

#[component]
pub fn RecipeDialog(
    open: RwSignal<bool>,
    /// The recipe to edit; `None`, or one with id 0, to add one.
    editing: RwSignal<Option<Recipe>>,
    /// Told with the saved recipe.
    #[prop(optional, into)] on_saved: Option<UnsyncCallback<Recipe>>,
) -> impl IntoView {
    let store = use_store();
    let name = RwSignal::new(String::new());
    let minutes = RwSignal::new(String::new());
    let notes = RwSignal::new(String::new());
    let saving = RwSignal::new(false);
    // Errors wait for the first save, so an empty form doesn't open in red.
    let tried = RwSignal::new(false);

    // Each opening starts from the recipe (or blank).
    Effect::new(move |_| {
        if open.get() {
            let recipe = editing.get_untracked();
            name.set(recipe.as_ref().map(|r| r.name.clone()).unwrap_or_default());
            minutes.set(recipe.as_ref().map(|r| r.minutes.to_string()).unwrap_or_default());
            notes.set(recipe.map(|r| r.notes).unwrap_or_default());
            tried.set(false);
        }
    });

    let name_error = Signal::derive(move || {
        (tried.get() && name.get().trim().is_empty()).then(|| "Give the recipe a name.".to_string())
    });
    let minutes_error = Signal::derive(move || {
        let valid = minutes.get().trim().parse::<u32>().is_ok_and(|m| m > 0);
        (tried.get() && !valid).then(|| "Minutes: a whole number above 0.".to_string())
    });

    let save = move || {
        tried.set(true);
        if name_error.get_untracked().is_some() || minutes_error.get_untracked().is_some() {
            return;
        }
        let recipe = Recipe {
            id: editing.get_untracked().map(|r| r.id).unwrap_or(0),
            name: name.get_untracked().trim().to_string(),
            minutes: minutes.get_untracked().trim().parse().unwrap_or(0),
            notes: notes.get_untracked(),
        };
        saving.set(true);
        store.save(recipe, move |saved| {
            saving.set(false);
            open.set(false);
            if let Some(on_saved) = on_saved {
                on_saved.run(saved);
            }
        });
    };

    // A recipe with id 0 is new (a duplicate, say), even though it has fields.
    let is_new = move || editing.with(|r| r.as_ref().is_none_or(|r| r.id == 0));
    let title = Signal::derive(move || if is_new() { "New recipe" } else { "Edit recipe" }.to_string());

    view! {
        <Dialog open=open title=title affirm=DialogAction::new("Save", save).saving(saving)>
            <Stack gap=Gap::Md>
                <Field label="Name" error=name_error>
                    <Input value=name autofocus=true required=true />
                </Field>
                <Field label="Minutes" hint="Start to finish" error=minutes_error>
                    <Input value=minutes kind=InputKind::Number />
                </Field>
                <Field label="Notes">
                    <Textarea value=notes rows=4 />
                </Field>
            </Stack>
        </Dialog>
    }
}
