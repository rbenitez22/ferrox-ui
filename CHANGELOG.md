# Changelog

All crates in this repo share one version. Nothing is released yet; until
`v0.1.0`, breaking changes land here under "Unreleased".

## Unreleased

### Changed (breaking): one name, one type per idea

From the names audit in `plans/agent-first.md` (piece 3).

- Every human-readable word is a `Signal<String>` (or `MaybeProp<String>`
  when optional). Literals still work. Changed from `String`: `Alert`'s,
  `Dialog`'s, `ConfirmDialog`'s, `Sheet`'s and `SideMenu`'s `close_label`;
  `Combobox` `clear_label`, `toggle_label`, `refresh_label`; `Select`
  `refresh_label`, `create_label`; `PasswordInput` `show_label`;
  `SideMenuButton` `label`; `SplitButton` `text`, `menu_label`;
  `DataView` `retry_label`; `MenuTrigger`'s texts and labels. `Button`
  `title` is a `MaybeProp<String>`.
- `IconButton` `title` → `label` (it's the button's name; `title` is a
  heading everywhere else).
- `LoadingOverlay` `text` → `label`.
- `Alert` `on_dismiss` → `on_close`, `dismiss_label` → `close_label`
  (default `"Close"`), class `.alert__dismiss` → `.alert__close`.
- `Dialog` `dismiss_label` → `cancel_label`, `no_dismiss` → `no_cancel`.
- `DataView` `empty` → `empty_label`.
- `ContextMenu` `items` → `menu`.
- No callback passes a DOM event: `Button`, `IconButton` and `SplitButton`
  `on_click` are `UnsyncCallback<()>` (as `Card`'s already was). Write
  every callback as `move || …`.

### Added

- `class` on `MenuItem`, `MenuLink` and `LoadingOverlay`;
  `menu_item_classes` takes the extra classes.
- `.ferrox-app` (in the theme stylesheet): the page's ground on `<body>`.
- `ferrox_theme_build::PREPAINT_SCRIPT` and `PREPAINT_FILE`: `write_css`
  writes `prepaint.js`, which `index.html` inlines.
- `starter/`: an app to copy.

### Removed

- `ferrox_theme::PREPAINT_SCRIPT`: the script is written by
  `ferrox-theme-build` and inlined, not pasted.
