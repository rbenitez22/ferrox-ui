//! The app's data, standing in for its API: recipes kept in memory, loaded
//! and saved with a short delay so the screens show their loading and
//! saving states. Replace the bodies of `load`, `save` and `remove` with
//! requests; the screens only see the signals.

use std::time::Duration;

use leptos::prelude::*;

/// How long the stand-in API takes to answer.
const LATENCY: Duration = Duration::from_millis(400);

/// One recipe. `PartialEq`, so a `DataView` redraws one that changed.
#[derive(Clone, Debug, PartialEq)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub minutes: u32,
    pub notes: String,
}

/// The user's settings, shown on the settings page and used by the others.
#[derive(Clone, Copy)]
pub struct Settings {
    /// Show cooking times on the cards.
    pub show_minutes: RwSignal<bool>,
    /// "metric" or "us": the units a recipe's notes use.
    pub units: RwSignal<String>,
    /// Ask before deleting a recipe.
    pub confirm_delete: RwSignal<bool>,
}

/// The store: what the screens read, and the calls that change it. Copy, so
/// any closure can hold it; provided by the shell, found with `use_store`.
#[derive(Clone, Copy)]
pub struct Store {
    pub recipes: RwSignal<Vec<Recipe>>,
    /// The list is on its way (the first load, or a reload).
    pub loading: RwSignal<bool>,
    /// Why the last load failed.
    pub error: RwSignal<Option<String>>,
    pub settings: Settings,
    /// What the stand-in "server" holds: a load returns it, every change
    /// goes to it first.
    server: StoredValue<Vec<Recipe>>,
    next_id: StoredValue<u32>,
    /// Fail the next load once: the stand-in API is "offline" when the
    /// page's address has `?offline`, so the error state can be seen.
    fail_next: StoredValue<bool>,
}

impl Store {
    /// Creates the store, provides it to everything inside, and starts the
    /// first load.
    pub fn provide() -> Self {
        let store = Store {
            recipes: RwSignal::new(Vec::new()),
            loading: RwSignal::new(false),
            error: RwSignal::new(None),
            settings: Settings {
                show_minutes: RwSignal::new(true),
                units: RwSignal::new("metric".to_string()),
                confirm_delete: RwSignal::new(true),
            },
            server: StoredValue::new(Vec::new()),
            next_id: StoredValue::new(1),
            fail_next: StoredValue::new(starts_offline()),
        };
        let first = [("Shakshuka", 30, "Eggs poached in spiced tomatoes."), ("Ribollita", 90, "Tuscan bread soup."), ("Pierogi", 60, "Filled dumplings, boiled then fried.")];
        let seeded = first.iter().map(|(name, minutes, notes)| store.new_recipe(name, *minutes, notes)).collect();
        store.server.set_value(seeded);
        provide_context(store);
        store.load();
        store
    }

    /// Loads the recipes: `loading` while on the way, `error` if it failed.
    pub fn load(self) {
        self.loading.set(true);
        self.error.set(None);
        set_timeout(
            move || {
                if self.fail_next.get_value() {
                    self.fail_next.set_value(false);
                    self.error.set(Some("Couldn't load the recipes.".to_string()));
                } else {
                    self.recipes.set(self.server.get_value());
                }
                self.loading.set(false);
            },
            LATENCY,
        );
    }

    /// Saves a recipe: a new one (id 0) is added, an existing one replaced.
    /// `done` runs when the save has landed, with the saved recipe.
    pub fn save(self, recipe: Recipe, done: impl FnOnce(Recipe) + 'static) {
        set_timeout(
            move || {
                let saved = if recipe.id == 0 { self.new_recipe(&recipe.name, recipe.minutes, &recipe.notes) } else { recipe };
                self.server.update_value(|all| match all.iter_mut().find(|r| r.id == saved.id) {
                    Some(old) => *old = saved.clone(),
                    None => all.push(saved.clone()),
                });
                self.recipes.set(self.server.get_value());
                done(saved);
            },
            LATENCY,
        );
    }

    /// Deletes a recipe; `done` runs when it's gone.
    pub fn remove(self, id: u32, done: impl FnOnce() + 'static) {
        set_timeout(
            move || {
                self.server.update_value(|all| all.retain(|r| r.id != id));
                self.recipes.set(self.server.get_value());
                done();
            },
            LATENCY,
        );
    }

    /// Deletes every recipe; `done` runs when they're gone.
    pub fn remove_all(self, done: impl FnOnce() + 'static) {
        set_timeout(
            move || {
                self.server.set_value(Vec::new());
                self.recipes.set(Vec::new());
                done();
            },
            LATENCY,
        );
    }

    /// The recipe with this id, tracked: a view that calls it follows edits.
    pub fn find(self, id: u32) -> Option<Recipe> {
        self.recipes.with(|all| all.iter().find(|r| r.id == id).cloned())
    }

    fn new_recipe(self, name: &str, minutes: u32, notes: &str) -> Recipe {
        let id = self.next_id.get_value();
        self.next_id.set_value(id + 1);
        Recipe { id, name: name.to_string(), minutes, notes: notes.to_string() }
    }
}

/// Whether the stand-in API fails its first answer (`?offline`).
fn starts_offline() -> bool {
    window().location().search().is_ok_and(|q| q.contains("offline"))
}

/// The store the shell provided.
pub fn use_store() -> Store {
    expect_context::<Store>()
}
