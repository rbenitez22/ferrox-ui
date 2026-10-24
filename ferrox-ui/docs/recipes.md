# Recipes

Whole screens built only from ferrox-ui, to copy: read the one you need and
change the nouns. Together they're one small app, "Cookbook", in the demo
at `/recipes` (run `trunk serve` in `demo/`), and its e2e tests
(`demo/e2e/tests/recipes.spec.ts`) cover each one, so they work.

| Recipe | File | What it shows |
|---|---|---|
| App shell | [`demo/src/recipes/shell.rs`](../../demo/src/recipes/shell.rs) | the store provided for every page; `AppBar` (menu button, the name linking home, a user `Menu` with a header); `SideMenu` with the app's pages; the router's `Outlet` |
| List page | [`demo/src/recipes/list_page.rs`](../../demo/src/recipes/list_page.rs) | `PageGrid` and a page `Panel` with a `DataLayoutSwitch` and an add button; `DataView` of `Card`s, each a link to its record with a row `Menu` (edit, delete); loading, error with Retry, and empty states; `ConfirmDialog` before deleting |
| Form in a dialog | [`demo/src/recipes/recipe_dialog.rs`](../../demo/src/recipes/recipe_dialog.rs) | one `Dialog` to add or edit, filled each time it opens; `Field`s in a `Stack`; errors only after a save is tried; Save's saving state until the store answers |
| Detail page | [`demo/src/recipes/detail_page.rs`](../../demo/src/recipes/detail_page.rs) | the record's name as the page's title; `SplitButton` (Edit, with Duplicate and Delete behind it); `Facts` in a side column; loading and not-found; delete, then back to the list |
| Settings page | [`demo/src/recipes/settings_page.rs`](../../demo/src/recipes/settings_page.rs) | a narrow page of `Panel`s: `Switch`es and a `Select` applied at once; a collapsible "Advanced"; a danger zone that asks |
| The data | [`demo/src/recipes/store.rs`](../../demo/src/recipes/store.rs) | a store standing in for the app's API: the screens read its signals and call `load`, `save`, `remove`; replace the bodies with requests |

The starter (`starter/`) is the smallest of these: one list page, no shell.
