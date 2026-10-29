# Which component for which need

One table per group. Each component's page has its props and example;
`index.md` lists every public item.

## core

| Need | Use |
|---|---|
| an action with words (Save, Delete) | `Button`: `family` says what it does (`Primary` for the screen's main action, `Secondary`, `Cancel`, `Danger`); `saving` while it works |
| an action shown only as an icon (a row's, a toolbar's) | `IconButton`, named by its `label` |
| a bar of buttons, grouped, with an optional title | `Toolbar` with `ToolbarGroup`s |
| a region whose data is on its way | `Spinner` (`SpinnerKind::Region`); a button's own work: its `saving` |
| nothing on screen is valid yet (boot, signing in) | `LoadingOverlay` |
| a message that stays until acted on (why a save failed) | `Alert` |
| help on a component | its `help` prop (a `HelpTopic`), or a `HelpButton` |
| many `?`s down a form | `HelpReveal::WhileActive` on their `HelpScope` |
| help from the keyboard (F1) | `HelpKeys`, once around the app |
| noticing a user may need help | `HelpSignals` around the app; the app's own through `help_signals()`; offering it: `HelpSuggestions` |

## overlay

| Use it when | Use |
|---|---|
| a task or a form with a clear finish | `Dialog` |
| a destructive action needs a decision (every one does: no "undo toast" instead) | `ConfirmDialog` |
| navigation, details or filters that slide in from an edge | `Sheet` |

## forms

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

## navigation

| Use it for | Use |
|---|---|
| the app's frame: its bar, its side menu of pages, one `<main>` around the routes | `AppShell` (every app) |
| a list of places: a sidebar, a settings page's sections | `NavList` of `NavLink`s |
| places that belong together (one service's pages, the account's) | `NavGroup` inside the list, collapsible if it's long or rarely used |
| a side menu outside `AppShell` (rare: the shell has one) | `SideMenu`, opened by a `SideMenuButton` |

## layout

| Use it for | Use |
|---|---|
| a page: its `<h1>` title and actions, its width, a side column, the tab's title | `Page` (every route) |
| a page's grid with no title yet (a spinner while its record loads) | `PageGrid` |
| the app's top band outside `AppShell` (rare: the shell has one) | `AppBar` |
| things one under another with even space: fields, panels, a card's lines | `Stack` |
| things side by side, wrapping: a row of buttons, tags | `Cluster` |
| a record's facts, label beside value (on a detail page, in a side column) | `Facts` with a `Fact` each |
| a page's own title and content, or a titled section of a page, with its buttons (add, search) | `Panel` |
| a section most users skip, or one that loads data only some need | `Panel collapsible=true` |
| one thing in a list (a record, an item, a choice), as a whole a link or a button if it opens something | `Card` |

## data

| Component | For |
|---|---|
| `DataView` | the list itself: one template per record (usually a `Card` in a grid, a row in a list), four layouts, its states beside it, optional infinite scroll |
| `DataLayoutSwitch` | lets the user pick the layout; sits in a `Panel`'s actions |
| `Pagination` | pages of anything; the app slices or fetches the page |

## media

| Component | For |
|---|---|
| `Carousel` | items in pages swiped sideways: a launcher's tiles, a gallery; on a phone where a grid won't fit |

## menus

| Need | Use |
|---|---|
| a row's or card's actions, an overflow "More" | `Menu` with `MenuTrigger::icon(MORE_VERTICAL, "More actions")` |
| a button that offers a few related actions ("Export ▾") | `Menu` with `MenuTrigger::button("Export")` |
| a user menu, a book or workspace switcher | `Menu` with `MenuTrigger::custom(view, label)`, a `header`, `current` items |
| one main action and its variants ("Save ▾") | `SplitButton` |
| the same actions on right-click, Shift+F10 or a long press | `ContextMenu` around the area, alongside a visible `Menu` |
| choosing a value, not running an action | not a menu: `Select` or `Combobox` (`forms`) |
