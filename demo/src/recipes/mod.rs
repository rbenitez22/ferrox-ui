//! Recipes: whole screens built only from ferrox-ui, to copy. Together they
//! are one small app, "Cookbook": a shell (the header and the side menu)
//! around a list page, a detail page and a settings page, with a form in a
//! dialog for adding and editing. The store stands in for the app's API.
//!
//! Each file is one recipe; `ferrox-ui/docs/recipes.md` lists them. The
//! files live in the crate's docs (they ship with it, for apps to copy);
//! the demo compiles them from there, so its e2e tests prove them.

#[path = "../../../ferrox-ui/docs/recipes/detail_page.rs"]
mod detail_page;
#[path = "../../../ferrox-ui/docs/recipes/list_page.rs"]
mod list_page;
#[path = "../../../ferrox-ui/docs/recipes/recipe_dialog.rs"]
mod recipe_dialog;
#[path = "../../../ferrox-ui/docs/recipes/settings_page.rs"]
mod settings_page;
#[path = "../../../ferrox-ui/docs/recipes/shell.rs"]
mod shell;
#[path = "../../../ferrox-ui/docs/recipes/store.rs"]
mod store;

pub use detail_page::DetailPage;
pub use list_page::ListPage;
pub use settings_page::SettingsPage;
pub use shell::RecipeShell;
