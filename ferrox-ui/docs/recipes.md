# Recipes

Whole screens built only from ferrox-ui, to copy: read the one you need and
change the nouns. The files are here, in `recipes/`, beside these docs.
Together they're one small app, "Cookbook": the demo compiles them and
shows it at `/recipes`, and its e2e tests cover each one, so they work.

| Recipe | File | What it shows |
|---|---|---|
| App shell | [`recipes/shell.rs`](recipes/shell.rs) | the store provided for every page; `AppBar` (menu button, the name linking home, a user `Menu` with a header); `SideMenu` with the app's pages; the router's `Outlet` |
| List page | [`recipes/list_page.rs`](recipes/list_page.rs) | `PageGrid` and a page `Panel` with a `DataLayoutSwitch` and an add button; `DataView` of `Card`s, each a link to its record with a row `Menu` (edit, delete); loading, error with Retry, and empty states; `ConfirmDialog` before deleting |
| Form in a dialog | [`recipes/recipe_dialog.rs`](recipes/recipe_dialog.rs) | one `Dialog` to add or edit, filled each time it opens; `Field`s in a `Stack`; errors only after a save is tried; Save's saving state until the store answers |
| Detail page | [`recipes/detail_page.rs`](recipes/detail_page.rs) | the record's name as the page's title; `SplitButton` (Edit, with Duplicate and Delete behind it); `Facts` in a side column; loading and not-found; delete, then back to the list |
| Settings page | [`recipes/settings_page.rs`](recipes/settings_page.rs) | a narrow page of `Panel`s: `Switch`es and a `Select` applied at once; a collapsible "Advanced"; a danger zone that asks |
| The data | [`recipes/store.rs`](recipes/store.rs) | a store standing in for the app's API: the screens read its signals and call `load`, `save`, `remove`; replace the bodies with requests |

The starter (`starter/`) is the smallest of these: one list page, no shell.

## Copying one recipe alone

The recipes are written as one module of the Cookbook app: each finds the
others as `super::…` (`super::store`, `super::recipe_dialog`). Copy them
into one module of your app and that still works; anywhere else, change
`super::` to where you put them. Each also leans on the rest of Cookbook,
which a single screen may not have:

| Recipe | Needs | Without it |
|---|---|---|
| App shell | a router (`Outlet`), the other pages' routes | drop the `NavLink`s and `MenuLink`s to pages you don't have |
| List page | `store.rs`, `recipe_dialog.rs`; a detail route (`/recipes/{id}`, the `Card`'s `href`); `store.settings` (ask before deleting, show minutes) | no detail page: drop the `href`; no settings: always confirm, and drop the minutes line or show it always |
| Form in a dialog | `store.rs` (`save`) | call your own save; on success, what `save`'s callback does |
| Detail page | `store.rs`, `recipe_dialog.rs`; a router (`use_params_map`, `use_navigate`), the list's route; `store.settings.units` | no settings: drop the units fact |
| Settings page | `store.rs` (`settings`; `remove_all` for the danger zone) | keep the `Settings` struct from `store.rs` |
| The data | nothing; `?offline` in the address fails the first load, to show the error state | keep `Settings` only if a page reads it; its latency is `LATENCY` |
