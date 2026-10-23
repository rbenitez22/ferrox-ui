# ferrox-ui

UI primitives for Leptos CSR apps. Components come in feature groups, and
they're styled by class-only CSS on [ferrox-theme](../ferrox-theme)'s tokens,
so every theme reaches every component. Nothing is styled unless it carries a
class: no element selectors, no global pseudo-classes.

Part of the [ferrox-ui workspace](../README.md). Apps using webapp-lib reach
it as `webapp_lib::ui` and don't list it themselves.

## Setup

```toml
[dependencies]
ferrox-ui = { git = "https://github.com/rbenitez22/ferrox-ui", tag = "v0.1.0" }
```

```rust
fn main() {
    ferrox_theme::init();      // the theme (its CSS is linked in index.html)
    ferrox_ui::init();         // the components' CSS, for every compiled group
    leptos::mount::mount_to_body(App);
}
```

`init()` adds one `<style id="ferrox-ui-…">` per sheet to `<head>`. A second
call adds nothing.

### Groups

| Group | Feature | Contains |
|---|---|---|
| `core` | always on | `Button`, `IconButton`, `Toolbar`, `ToolbarGroup`, `Spinner`, `LoadingOverlay`, `Alert` |
| `forms`, `menus`, `overlay`, `navigation`, `layout`, `data` | one each, planned | see `plans/ferrox-ui.md` |

### Conventions (every component)

- `class`: extra classes, appended to the component's own. A string, or a
  `Signal<String>` to make it reactive. A bare closure doesn't convert to
  `MaybeProp<String>`, so wrap it in `Signal::derive`.
- No `style` prop. Styling goes through classes and tokens.
- Enums (`ButtonFamily`, `SpinnerKind`, `AlertSeverity`, …) are
  `#[non_exhaustive]`: match them with a `_` arm.
- Each component has a `*_classes` function that returns the class list it
  renders, for markup that needs the same look without the component.

## core

### `Button`

A padded button (`.btn`). The **family** is what the button means. **Quiet**
is a separate axis: how heavy it looks.

```rust
use ferrox_ui::{Button, ButtonFamily, ButtonSize};

view! {
    <Button on_click=move |_| save()>"Save"</Button>
    <Button family=ButtonFamily::Cancel on_click=move |_| close()>"Cancel"</Button>
    <Button family=ButtonFamily::Secondary quiet=true size=ButtonSize::Sm>"Export"</Button>
    <Button saving=saving button_type="submit">"Create"</Button>
}
```

| Prop | Default | |
|---|---|---|
| `family` | `Primary` | `Primary` (affirmative), `Secondary` (committing, not the main action), `Cancel` (dismissive, always an outline), `Danger` (destructive) |
| `quiet` | `false` | outline instead of fill |
| `size` | `Md` | `Sm`, `Md`, `Lg` |
| `saving` | `false` | a request is running: `aria-disabled` and a sweeping border. The button stays focusable and refuses clicks. It never sets `disabled`. |
| `disabled` | `false` | |
| `button_type` | `"button"` | not HTML's `"submit"`; pass `"submit"` for a form's submit button |
| `on_click`, `title`, `class` | | |

`button_classes(family, quiet, size, saving, extra)` returns the class list.

### `IconButton`

A compact icon-only button (`.icon-btn`): 1.75rem square, no border. It
answers hover and press in colour, not size.

```rust
use ferrox_icons::{Icon, CAMERA, DELETE, SEARCH};
use ferrox_ui::IconButton;

view! {
    <IconButton title="Search" on_click=move |_| open_search()><Icon icon=SEARCH/></IconButton>
    <IconButton title="Camera" pressed=camera on_click=move |_| camera.update(|c| *c = !*c)><Icon icon=CAMERA/></IconButton>
    <IconButton title="Remove line" danger=true on_click=move |_| remove()><Icon icon=DELETE/></IconButton>
}
```

| Prop | |
|---|---|
| `title` | required: the button's only label (also `aria-label`) |
| `pressed` | makes it a switch (`aria-pressed`); on reads as tinted |
| `danger` | for a destructive action: drawn in the danger colour, tinted on hover |
| `disabled`, `on_click`, `class` | |
| children | the icon: ferrox-icons' `Icon`, or any `<svg class="icon">` |

`icon_button_classes(danger, extra)` returns the class list.

### `Toolbar`, `ToolbarGroup`

A row of buttons. Buttons sit at the right, and an optional `title` takes the
free space on the left. `compact` tightens the row for icon buttons and small
buttons. Groups are split by a hairline.

```rust
use ferrox_ui::{Toolbar, ToolbarGroup};

view! {
    <Toolbar title="Accounts">
        <Button on_click=move |_| add()>"New account"</Button>
    </Toolbar>

    <Toolbar compact=true>
        <ToolbarGroup>
            <IconButton title="Search"><Icon icon=SEARCH/></IconButton>
            <IconButton title="Camera" pressed=camera><Icon icon=CAMERA/></IconButton>
        </ToolbarGroup>
        <ToolbarGroup>
            <IconButton title="Help"><Icon icon=HELP/></IconButton>
        </ToolbarGroup>
    </Toolbar>
}
```

`Toolbar` takes `title`, `compact` and `class`; `ToolbarGroup` takes `class`.
`toolbar_classes(compact, extra)` returns the class list.

### `Spinner`, `LoadingOverlay`

Choose a spinner by **what stays usable while the work runs**, not by how
long it takes:

| Use it when | Use |
|---|---|
| nothing on screen is valid any more: boot, resolving auth, the post-login redirect. Never a route change or a data fetch. | `LoadingOverlay` |
| any committing action | `Button saving=…` |
| a pane fetching data whose shape it can't promise | `Spinner` (region, the default) |
| an icon-only row action; the spinner takes the icon's place | `Spinner kind=SpinnerKind::Inline` |
| async work that belongs to one field (lookup, typeahead) | `Spinner kind=SpinnerKind::Field` |

Use one indicator per action: a saving button never also raises an overlay.

Every spinner is transparent until `--loading-delay` (400ms), so work that
finishes faster never shows one. Under `prefers-reduced-motion` the spin slows
to 2.4s but doesn't stop; a still spinner reads as "hung".

```rust
use ferrox_ui::{LoadingOverlay, Spinner, SpinnerKind};

view! {
    <Spinner label="Loading accounts"/>
    <Spinner kind=SpinnerKind::Inline label="Deleting the row"/>
    {move || signing_in.get().then(|| view! { <LoadingOverlay text="Signing in…"/> })}
}
```

- `Spinner`: `label` (required; read out by screen readers, not shown),
  `kind` (`Region` by default, `Inline` or `Field`), `class`. It renders
  `role="status"`.
- `LoadingOverlay`: `text` (optional; shown under the spinner and read out;
  without it screen readers hear "Loading…"). A fixed scrim over the whole
  page, `role="status"` with `aria-live="polite"`.

`spinner_status_classes(kind, extra)` returns the wrapper's class list.

### `Alert`

An in-page or in-form message that **never times out**: the reason a submit
failed, a server refusal no single field owns. Messages that go away by
themselves are toasts (planned, `overlay`). Anything the user must act on is
an alert.

```rust
use ferrox_ui::{Alert, AlertSeverity};

view! {
    <Alert severity=AlertSeverity::Error>"The server refused the change: the period is closed."</Alert>
    <Alert severity=AlertSeverity::Info icon=false>"Changes apply to new entries only."</Alert>
    {move || error.get().map(|e| view! {
        <Alert severity=AlertSeverity::Error on_dismiss=move |_| error.set(None)>{e}</Alert>
    })}
}
```

| Prop | Default | |
|---|---|---|
| `severity` | required | `Info`, `Success`, `Warning` (non-blocking: if it should stop a submit, it's an `Error`), `Error` |
| `icon` | `true` | draws the severity's icon (ferrox-icons `INFO`, `SUCCESS`, `WARNING`, `ERROR`), so severity doesn't rest on colour alone |
| `on_dismiss` | none | shows a close button that calls it; the caller removes the alert |
| `dismiss_label` | `"Dismiss"` | the close button's label |
| `class` | | |

Only `Error` interrupts a screen reader (`role="alert"`); the others are
`role="status"`. There's no primary alert, because an alert reports a state
and doesn't act. `alert_classes(severity, extra)` returns the class list, and
`AlertSeverity` has `class()`, `icon()` and `role()`.

## CSS classes (core)

All of these are injected by `init()`, built only from theme tokens, and
selected by class only.

| Sheet | Classes |
|---|---|
| `a11y.css` | `.sr-only`: read out by screen readers, takes no space |
| `focus.css` | the focus ring for `.btn` and `.icon-btn` |
| `buttons.css` | `.btn`, `.btn--{primary,secondary,cancel,danger}`, `.btn--quiet`, `.btn--{sm,lg}`, `.is-saving`; `.icon-btn`, `.icon-btn--danger`, `[aria-pressed='true']` |
| `icon.css` | `.icon`: an SVG icon as a block with no pointer events |
| `toolbar.css` | `.toolbar`, `.toolbar--compact`, `.toolbar__title`, `.toolbar__group` |
| `spinner.css` | `.spinner`, `.spinner--{region,inline,field}`, `.spinner-status`, `.spinner-status--region`, `.spinner-overlay`, `.spinner-overlay__text`; keyframes `ferrox-spin`, `ferrox-spinner-appear` |
| `alert.css` | `.alert`, `.alert--{info,success,warning,error}`, `.alert__icon`, `.alert__body`, `.alert__dismiss` |

A unit test (`css.rs`) fails if any shipped selector doesn't start with a
class.

## Requirements

- `leptos` 0.8 (CSR) and a browser (`web-sys`).
- ferrox-theme's stylesheet linked in the app's `index.html`
  (ferrox-theme-build). Every component reads its tokens.
- ferrox-icons, used for Alert's icons and close button (a dependency, not
  something the app sets up).

## Testing

- `cargo test -p ferrox-ui`: class lists, roles, and the class-only rule on
  every sheet.
- The demo (`demo/`) shows every component: `/buttons` and `/feedback`. Its
  Playwright tests (`demo/e2e/tests/buttons.spec.ts`, `feedback.spec.ts`) run
  in Chromium and Firefox through `scripts/release-check.sh`.

## Planned

From `plans/ferrox-ui.md`, one group per piece, each with a demo page and e2e
tests: `overlay` (Dialog, Alert dialog, Sheet), `forms` (Field, Input,
Checkbox, Select, Combobox), `navigation` (Side menu), `layout` (Panel),
`data` (Data grid). `core` later gains Skeleton, Badge, Card and a theme
toggle.

## Licence

`MIT OR Apache-2.0`.
