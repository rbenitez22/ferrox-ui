//! Recipes: whole screens built only from ferrox-ui, to copy. Together they
//! are one small app, "Cookbook": a shell (the header and the side menu)
//! around a list page, a detail page and a settings page, with a form in a
//! dialog for adding and editing. The store stands in for the app's API.
//!
//! Each file is one recipe; `ferrox-ui/docs/recipes.md` lists them.

mod detail_page;
mod list_page;
mod recipe_dialog;
mod settings_page;
mod shell;
mod store;

pub use detail_page::DetailPage;
pub use list_page::ListPage;
pub use settings_page::SettingsPage;
pub use shell::RecipeShell;
