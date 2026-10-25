# ferrox-ui

UI primitives for Leptos CSR apps. Components come in feature groups (`core`
always, the others by Cargo feature), and they're styled by class-only CSS on
[ferrox-theme](../ferrox-theme)'s tokens, so every theme reaches every
component. Nothing is styled unless it carries a class: no element
selectors, no global pseudo-classes.

Part of the [ferrox-ui workspace](../README.md).

## Setup

The workspace's [Start here](../README.md#start-here) and its
[`starter/`](../starter/) app hold the setup: the dependencies, the build
script, `index.html`, and the two calls before mounting (`ferrox_theme::init()`,
then `ferrox_ui::init()`). `ferrox_ui::init()` adds one `<style
id="ferrox-ui-…">` per stylesheet of the compiled groups to `<head>`; a second
call adds nothing.

## Finding your way

| To | Read |
|---|---|
| pick the component for a need | [`docs/choosing.md`](docs/choosing.md) |
| build a whole screen (list, form, detail, settings, app shell) | [`docs/recipes.md`](docs/recipes.md): working screens to copy |
| see every public item and its props, one line each | [`docs/index.md`](docs/index.md) (generated from the source) |
| use one component: its props, example and behaviour | `docs/<component>.md`, linked from its group's page below |
| learn the rules every component follows (names, callbacks, words, `class`) | [`docs/conventions.md`](docs/conventions.md) |
| style your own markup like a component | [`docs/css-classes.md`](docs/css-classes.md) |
| work through an agent | [`agent-kit/`](agent-kit/README.md): install it into the app; the agent then needs only "use ferrox-ui" |

## Groups

| Group | Feature | Contains |
|---|---|---|
| [`core`](docs/core.md) | always on | `Button`, `IconButton`, `Toolbar`, `ToolbarGroup`, `Spinner`, `LoadingOverlay`, `Alert`; help: `HelpTopic`, `HelpButton`, `HelpScope`, `HelpAssistant`, `HelpRequest` |
| [`overlay`](docs/overlay.md) | `overlay` | `Dialog`, `DialogAction`, `ConfirmDialog`, `Sheet` |
| [`forms`](docs/forms.md) | `forms` | `Field`, `Input`, `Textarea`, `PasswordInput`, `Checkbox`, `Switch`, `Select`, `Combobox` |
| [`navigation`](docs/navigation.md) | `navigation` (turns on `overlay` and `layout`; brings leptos_router) | `AppShell`, `NavList`, `NavLink`, `NavGroup`, `SideMenu`, `SideMenuButton` |
| [`layout`](docs/layout.md) | `layout` | `AppBar`, `PageGrid`, `Stack`, `Cluster` (with `PageWidth`, `SidePosition`, `Gap`, `Align`, `Justify`), `Panel` (also collapsible), `PanelTone`, `HeadingLevel`, `Card`, `Facts`, `Fact` |
| [`data`](docs/data.md) | `data` | `DataView` (with infinite scroll), `DataLayout`, `CardSize`, `DataLayoutSwitch`, `DataLayoutLabels`, `Pagination`, `PaginationLabels`, `page_window`, `PageItem` |
| [`menus`](docs/menus.md) | `menus` | `Menu`, `MenuTrigger`, `MenuAlign`, `MenuItem`, `MenuLink`, `MenuDivider`, `SplitButton`, `ContextMenu` |

## Requirements

- `leptos` 0.8 (CSR) and a browser (`web-sys`).
- ferrox-theme's stylesheet linked in the app's `index.html`
  (ferrox-theme-build). Every component reads its tokens.
- ferrox-icons, used for Alert's icons and the close buttons (a dependency,
  not something the app sets up).
- `overlay`: a browser with `<dialog>`/`showModal()` and `@starting-style`
  (Chrome 117+, Firefox 129+, Safari 17.5+). Without `@starting-style` a
  sheet just appears instead of sliding.
- `navigation`: leptos_router 0.8, with every component inside a
  `<Router>`.
- Help and `Combobox`: a browser with the Popover API (`popover`, `popovertarget`; Chrome
  114+, Firefox 125+, Safari 17+).
- `data`: `IntersectionObserver` for `DataView`'s infinite scroll (every
  current browser). Uses `js-sys`.

## Testing

- `cargo test -p ferrox-ui --all-features`: class lists, roles, the
  class-only rule on every sheet, and that `docs/` is current
  (`tests/docs_current.rs`: the index matches the source;
  `FERROX_BLESS=1` rewrites it), and the agent kit's scripts
  (`tests/agent_kit.rs`: the bash and PowerShell versions give the same,
  specified output on the same fixtures; the PowerShell half needs
  `pwsh`).
- The demo (`demo/`) shows every component: `/buttons`, `/feedback`,
  `/overlay`, `/forms`, `/navigation`, `/layout`, `/data`, `/menus` and `/help` (with a fake assistant that shows the
  request it receives). Its Playwright tests (`demo/e2e/tests/*.spec.ts`)
  run in Chromium and Firefox through `scripts/release-check.sh`.

## Planned

From `plans/ferrox-ui.md`, one group per piece, each with a demo page and e2e
tests: every group is built. `core` later gains
Skeleton, Badge and a theme toggle. `overlay` later gains
`TypeToConfirmDialog`, Toast, Popover and Tooltip, and the
phone variants of Dialog (a bottom sheet, full screen) when an app needs them.
Help later gains a topic registry (checked `related` links, a `/help` guide
rendered from the same topics) and `help` on new components as they land.
`forms` later gains Combobox groups and multi-select, and
NumberInput, DateInput, Radio and the rest of the inventory when an app needs
them. `navigation` later gains a persistent sidebar that folds to a rail of
icons, count badges on links (with `core`'s Badge), Breadcrumb, Bottom nav
and Stepper. `layout` later gains Tabs and Accordion (a stack of
collapsible panels), when an app needs them. `data` later gains selecting
records (one or many, with listbox or grid roles), Table and DataTable
(columns, sorting, filtering) and virtualised rendering, when an app needs
them. `menus` later gains checkable items (`menuitemcheckbox`,
`menuitemradio`), submenus, a Menubar and a command palette, when an app
needs them.

## Licence

`MIT OR Apache-2.0`.
