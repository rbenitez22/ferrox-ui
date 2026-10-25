# Recipes

Whole screens built only from ferrox-ui, to copy: read the one you need and
change the nouns. The files are here, in `recipes/`, beside these docs.
Together they're one small app, "Cookbook": the demo compiles them and
shows it at `/recipes`, and its e2e tests cover each one, so they work.

| Recipe | File | What it shows |
|---|---|---|
| App shell | [`recipes/shell.rs`](recipes/shell.rs) | the store provided for every page; an `AppShell`: the name linking home, a user `Menu` with a header at the bar's end, the app's pages in the side menu, its footer; the router's `Outlet` |
| List page | [`recipes/list_page.rs`](recipes/list_page.rs) | `PageGrid` and a page `Panel` with a `DataLayoutSwitch` and an add button; `DataView` of `Card`s, each a link to its record with a row `Menu` (edit, delete); loading, error with Retry, and empty states; `ConfirmDialog` before deleting |
| Form in a dialog | [`recipes/recipe_dialog.rs`](recipes/recipe_dialog.rs) | one `Dialog` to add or edit, filled each time it opens; `Field`s in a `Stack`; errors only after a save is tried; Save's saving state until the store answers |
| Detail page | [`recipes/detail_page.rs`](recipes/detail_page.rs) | the record's name as the page's title; `SplitButton` (Edit, with Duplicate and Delete behind it); `Facts` in a side column; loading and not-found; delete, then back to the list |
| Settings page | [`recipes/settings_page.rs`](recipes/settings_page.rs) | a narrow page of `Panel`s: `Switch`es and a `Select` applied at once; a collapsible "Advanced"; a danger zone that asks |
| The data | [`recipes/store.rs`](recipes/store.rs) | a store standing in for the app's API: the screens read its signals and call `load`, `save`, `remove`; replace the bodies with requests |

The starter (`starter/`) is the smallest of these: one list page, no shell.

## Wiring copied recipes in

Put the files in one module, `src/recipes/`, with a `mod.rs` that declares
them (`mod store; mod recipe_dialog; mod list_page;`) and re-exports the
screens and the store (`pub use list_page::ListPage;`,
`pub use store::Store;`), and `mod recipes;` in
`main.rs`. Call `Store::provide()` once, in the component that holds the
screens (the Cookbook shell does; in the starter, `App`), then render a
screen. `store.rs` reads the page's address (`?offline`): it needs
web-sys's `Location` feature, which the starter enables.

**Into the starter.** Its `src/app.rs` is the frame (`App`: the router,
the `AppShell` and its links) and one sample screen (`ItemsPage`, with its
`Item` and in-memory list). Keep `App`; delete the sample; call
`Store::provide()` at the top of `App`; make each route render a recipe's
page (`<Route path=path!("/") view=ListPage />`) and give it a `NavLink`
in the shell's `nav`.

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
| The data | nothing; `?offline` in the address fails the first load, to show the error state | delete what no copied page uses (`Settings` and its fields, `find`, `remove_all`), or the build warns of dead code; its latency is `LATENCY` |
