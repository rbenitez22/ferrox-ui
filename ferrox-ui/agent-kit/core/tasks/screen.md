# Task: build a screen

A whole screen: a list of records, a form, a record's page, settings, the
app's shell. DOCS is the folder `{{KIT}}/check/ferrox-docs` prints.

1. **Name the screen's kind** from what the user asked: a list page (browse,
   add, edit, delete records), a form (usually in a dialog), a detail page
   (one record, its actions), a settings page, or the app shell (the top
   bar, the menu, the pages).
2. **Read its recipe.** `DOCS/recipes.md` lists one file per kind; read that
   file in full. It is a working screen, tested end to end.
3. **Copy it and change the nouns**: the record type, its fields, the
   words. Keep its structure: the `PageGrid`, the page's `Panel` with its
   actions, the `DataView` and its states, the dialogs and confirmations,
   the store's calls. Wire the store's calls to the app's own data.
4. **For anything the recipe doesn't have**, follow the `component` task.
   Don't add CSS: if the screen seems to need some, a component or a
   layout component (`Stack`, `Cluster`, `PageGrid`) almost certainly does
   it; look in `DOCS/choosing.md` again.
5. **Check and build**: the `check` task, then the app's build.

## Done when

The screen works (try every action, the empty, loading and error states),
`ferrox-check` reports no drift, and the app builds.
