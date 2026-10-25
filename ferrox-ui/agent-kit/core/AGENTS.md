# ferrox-ui, for agents

ferrox-ui is the UI for this app: components for Leptos CSR apps (buttons,
forms, dialogs, menus, lists, page layout) styled by a theme the app
doesn't write. Use it for every screen. An app built only from its
components, with no CSS of its own, looks and behaves like every other
ferrox-ui app; that consistency is the point, so don't hand-make what a
component already does.

## How to work

1. **Find the docs.** No `Cargo.toml` with ferrox-ui in it yet? Do the
   `setup` task first; it needs no docs. Then run
   `{{KIT}}/check/ferrox-docs.sh` (or `ferrox-docs.ps1`) in the app's
   folder. It prints the docs folder of the ferrox-ui this app builds
   against; below, that folder is **DOCS**. Read nothing else for
   ferrox-ui's API: those docs match the code you compile.
2. **Pick the job's task** and follow it:
   <!-- rendered: tasks -->
   - `setup`: Add ferrox-ui to a Leptos CSR app, or start a new app from the starter (`{{KIT}}/core/tasks/setup.md`)
   - `screen`: Build a screen (a list, a form, a detail page, settings, the app shell) from a recipe (`{{KIT}}/core/tasks/screen.md`)
   - `component`: Find the right component for a need and use it correctly (`{{KIT}}/core/tasks/component.md`)
   - `check`: Run the drift check on the app and fix what it finds (`{{KIT}}/core/tasks/check.md`)
   <!-- /rendered -->
3. **Read before you write.** For each component you use, read its page
   (`DOCS/<component>.md`) first. `DOCS/index.md` lists every public item
   with its exact props and types; `DOCS/conventions.md` (read it once) has
   the rules every component follows: how callbacks, words, overlays and
   `class` work. Never guess a prop.
4. **Start from a recipe** for any whole screen: `DOCS/recipes.md` lists
   working screens (a list page, a form in a dialog, a detail page, a
   settings page, an app shell) to copy and change the nouns of.
5. **Check before you say it's done.** Run `{{KIT}}/check/ferrox-check.sh`
   (or `.ps1`) on the app's folder and fix every finding (the `check`
   task). Then build the app.

## The rules

The drift check enforces these; each line says what to do instead.

<!-- rendered: rules -->
- `raw-button`: a hand-made <button>. Instead: use Button, or IconButton for an icon.
- `raw-input`: a hand-made <input>. Instead: use Input (in a Field), Checkbox or Switch.
- `raw-select`: a hand-made <select>. Instead: use Select (in a Field), or Combobox for a long list.
- `raw-textarea`: a hand-made <textarea>. Instead: use Textarea (in a Field).
- `raw-dialog`: a hand-made <dialog>. Instead: use Dialog, ConfirmDialog or Sheet.
- `raw-menu`: a hand-made menu. Instead: use Menu, SplitButton or ContextMenu.
- `inline-style`: an inline style. Instead: use a component prop, a layout component (Stack, Cluster, PageGrid) or a class on the tokens.
- `css-hex-colour`: a colour written out in CSS (outside a --token definition). Instead: use a theme token such as var(--text) or var(--primary-fill).
- `css-colour-function`: a colour written out in CSS (outside a --token definition). Instead: use a theme token such as var(--text) or var(--primary-fill).
- `css-pixels`: a size in pixels in CSS (outside @media and --token definitions). Instead: use a spacing token such as var(--space-sm), or a layout component.
- `missing-ui-init`: ferrox_ui::init() is never called. Instead: call it in main(), after ferrox_theme::init() and before mounting.
- `missing-theme-init`: ferrox_theme::init() is never called. Instead: call it in main(), before ferrox_ui::init() and mounting.
- `missing-app-ground`: <body> lacks class="ferrox-app". Instead: write <body class="ferrox-app"> in index.html.
- `missing-prepaint`: index.html does not inline the pre-paint script. Instead: add <link data-trunk rel="inline" href="style/ferrox/prepaint.js" /> first in <head>.
<!-- /rendered -->

Beyond what the check can see: a page is a `PageGrid`, the space between
things comes from `Stack` and `Cluster`, a page's title and its buttons are
a `Panel` (`heading_level=HeadingLevel::H1`), and a list of records is a
`DataView`.

## Which component for which need

<!-- rendered: choosing -->
### core

| Need | Use |
|---|---|
| an action with words (Save, Delete) | `Button`: `family` says what it does (`Primary` for the screen's main action, `Secondary`, `Cancel`, `Danger`); `saving` while it works |
| an action shown only as an icon (a row's, a toolbar's) | `IconButton`, named by its `label` |
| a bar of buttons, grouped, with an optional title | `Toolbar` with `ToolbarGroup`s |
| a region whose data is on its way | `Spinner` (`SpinnerKind::Region`); a button's own work: its `saving` |
| nothing on screen is valid yet (boot, signing in) | `LoadingOverlay` |
| a message that stays until acted on (why a save failed) | `Alert` |
| help on a component | its `help` prop (a `HelpTopic`), or a `HelpButton` |

### overlay

| Use it when | Use |
|---|---|
| a task or a form with a clear finish | `Dialog` |
| a destructive action needs a decision (every one does: no "undo toast" instead) | `ConfirmDialog` |
| navigation, details or filters that slide in from an edge | `Sheet` |

### forms

| Need | Use |
|---|---|
| a label, a control, and its hint or error | `Field` around the control |
| a line of text, an email, a number, a date | `Input` (its `kind`) |
| several lines of text | `Textarea` |
| a password (a strength meter for a new one) | `PasswordInput` |
| yes or no, applied when the form is submitted | `Checkbox` |
| on or off, applied at once | `Switch` |
| one choice from a short list | `Select` |
| one choice from a long list, typed to filter (or to create a new one) | `Combobox` |
| running an action, not choosing a value | not a form control: `Button`, or a `Menu` |

### navigation

| Use it for | Use |
|---|---|
| a list of places: a sidebar, a settings page's sections | `NavList` of `NavLink`s |
| places that belong together (one service's pages, the account's) | `NavGroup` inside the list, collapsible if it's long or rarely used |
| the app's navigation on a phone, or any screen without room for a sidebar | `SideMenu`, opened by a `SideMenuButton` |

### layout

| Use it for | Use |
|---|---|
| the app's top band: its menu button, its name, its actions | `AppBar` |
| the page: its width, padding and `<main>`, and a side column (filters, a summary) that moves under it on a phone | `PageGrid` |
| things one under another with even space: fields, panels, a card's lines | `Stack` |
| things side by side, wrapping: a row of buttons, tags | `Cluster` |
| a record's facts, label beside value (on a detail page, in a side column) | `Facts` with a `Fact` each |
| a page's own title and content, or a titled section of a page, with its buttons (add, search) | `Panel` |
| a section most users skip, or one that loads data only some need | `Panel collapsible=true` |
| one thing in a list (a record, an item, a choice), as a whole a link or a button if it opens something | `Card` |

### data

| Component | For |
|---|---|
| `DataView` | the list itself: one template per record (usually a `Card` in a grid, a row in a list), four layouts, its states beside it, optional infinite scroll |
| `DataLayoutSwitch` | lets the user pick the layout; sits in a `Panel`'s actions |
| `Pagination` | pages of anything; the app slices or fetches the page |

### menus

| Need | Use |
|---|---|
| a row's or card's actions, an overflow "More" | `Menu` with `MenuTrigger::icon(MORE_VERTICAL, "More actions")` |
| a button that offers a few related actions ("Export ▾") | `Menu` with `MenuTrigger::button("Export")` |
| a user menu, a book or workspace switcher | `Menu` with `MenuTrigger::custom(view, label)`, a `header`, `current` items |
| one main action and its variants ("Save ▾") | `SplitButton` |
| the same actions on right-click, Shift+F10 or a long press | `ContextMenu` around the area, alongside a visible `Menu` |
| choosing a value, not running an action | not a menu: `Select` or `Combobox` (`forms`) |
<!-- /rendered -->
