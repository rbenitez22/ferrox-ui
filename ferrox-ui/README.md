# ferrox-ui

UI primitives for Leptos CSR apps. Components come in feature groups (`core`
always, `overlay` and more by feature), and they're styled by class-only CSS
on [ferrox-theme](../ferrox-theme)'s tokens, so every theme reaches every
component. Nothing is styled unless it carries a
class: no element selectors, no global pseudo-classes.

Part of the [ferrox-ui workspace](../README.md). Apps using webapp-lib reach
it as `webapp_lib::ui` and don't list it themselves.

## Setup

```toml
[dependencies]
ferrox-ui = { git = "https://github.com/rbenitez22/ferrox-ui", tag = "v0.1.0", features = ["overlay"] }
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
| `core` | always on | `Button`, `IconButton`, `Toolbar`, `ToolbarGroup`, `Spinner`, `LoadingOverlay`, `Alert`; help: `HelpTopic`, `HelpButton`, `HelpScope`, `HelpAssistant`, `HelpRequest` |
| `overlay` | `overlay` | `Dialog`, `DialogAction`, `ConfirmDialog`, `Sheet` |
| `forms` | `forms` | `Field`, `Input`, `Textarea`, `PasswordInput`, `Checkbox`, `Switch`, `Select` (documented with the group's last piece; see the rustdoc meanwhile) |
| `menus`, `navigation`, `layout`, `data` | one each, planned | see `plans/ferrox-ui.md` |

### Conventions (every component)

- `class`: extra classes, appended to the component's own. A string, or a
  `Signal<String>` to make it reactive. A bare closure doesn't convert to
  `MaybeProp<String>`, so wrap it in `Signal::derive`.
- No `style` prop. Styling goes through classes and tokens.
- Enums (`ButtonFamily`, `SpinnerKind`, `AlertSeverity`, …) are
  `#[non_exhaustive]`: match them with a `_` arm.
- Each component has a `*_classes` function that returns the class list it
  renders, for markup that needs the same look without the component.
- User-facing text (button labels, "Close", "Cancel", "Dismiss") is a prop
  with an English default.
- A callback with no input is a zero-argument closure:
  `on_close=move || open.set(false)`.
- Components with a label or a title take `help` (a `HelpTopic`): see
  [Help](#help-in-core).

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

`Toolbar` takes `title`, `compact`, `help` (a `?` right after the title) and
`class`; `ToolbarGroup` takes `class`.
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

## overlay (feature `overlay`)

Modals built on the browser's own `<dialog>`, opened with `showModal()`. The
browser makes the page behind inert, keeps focus inside, and puts the modal in
the top layer, above everything including `LoadingOverlay`. All three share
the same behaviour:

- **`open` drives it.** Every way out (Esc, the ×, the dismiss button, a
  sheet's backdrop) calls `on_close` / `on_cancel`, and the app sets `open`
  to false. If the browser closes the element itself, the callback runs too,
  so the app's state and the element never disagree.
- **Focus** goes to an element marked `autofocus`, otherwise the first
  control (the ×). On close it returns to whatever opened the modal.
- **The page doesn't scroll** while a modal is open (`<body>` gets
  `.dialog-open`). This is counted, so a confirm opened from a dialog keeps
  the page locked until both close.
- **The body is mounted only while open**, so each opening starts fresh.

| Use it when | Use |
|---|---|
| a task or a form with a clear finish | `Dialog` |
| a destructive action needs a decision (every one does: no "undo toast" instead) | `ConfirmDialog` |
| navigation, details or filters that slide in from an edge | `Sheet` |

**Watch your descendant selectors.** A `<dialog>` stays where it's declared
in the DOM, so an app rule like `.section h2 { margin: … }` also reaches the
dialog's title. Scope such rules to direct children (`.section > h2`) where
dialogs are declared.

### `Dialog`, `DialogAction`

A title bar with the ×, the body, and a footer band. **The footer isn't a
slot.** Pass the actions and the component fixes the order: dismiss,
alternate, affirm, with the heaviest button last.

```rust
use ferrox_ui::{Dialog, DialogAction, DialogSize};

let editing = RwSignal::new(false);
let saving = RwSignal::new(false);

view! {
    <Button on_click=move |_| editing.set(true)>"Edit account…"</Button>
    <Dialog
        open=editing
        title="Edit account"
        on_close=move || editing.set(false)
        affirm=DialogAction::new("Save", move || save()).saving(saving)
        alternate=DialogAction::new("Save as draft", move || save_draft())
    >
        <label>"Name" <input autofocus=true /></label>
    </Dialog>
}
```

| Prop | Default | |
|---|---|---|
| `open`, `title`, `on_close` | required | `on_close` is called by Esc, the × and the dismiss button: the same act |
| `affirm` | none | the last button; `DialogAction::new(label, fn)`, plus `.danger()` for a destructive commit, `.saving(signal)`, `.disabled(signal)` |
| `alternate` | none | a lighter way to finish, before `affirm`, always secondary; Enter never reaches it |
| `dismiss_label` | `"Cancel"` | |
| `no_dismiss` | `false` | no dismiss button (the × and Esc still close) |
| `submit_on_enter` | `true` | Enter fires `affirm`, but not from a textarea, a button or a link, and not while it's saving or disabled |
| `size` | `Md` | `Md` (32rem) or `Wide` (52rem, for a body that's a table) |
| `alert` | `false` | `role="alertdialog"`, for a dialog that interrupts to ask for a decision |
| `close_label` | `"Close"` | the ×'s label |
| `help` | none | a `?` in the title bar, before the × |
| `class` | | |

A backdrop click doesn't close a Dialog, so a stray click can't throw away
typed input. On a phone (≤ 768px) a dialog is full width inside an 8px
margin, and the × is 44px. `dialog_classes(size, extra)` returns the class
list.

### `ConfirmDialog`

The standard destructive confirmation (`role="alertdialog"`). The confirm
button is always in the danger family and comes last. Its label is the verb
("Delete", "Void", "Archive"), not "OK". The body names exactly what is about
to be lost. Focus opens on the ×, never on the confirm button.

```rust
use ferrox_ui::ConfirmDialog;

view! {
    <ConfirmDialog
        open=confirming
        title="Delete account?"
        confirm_label="Delete"
        saving=deleting
        on_confirm=move || delete_account()     // set `deleting`, close when done
        on_cancel=move || confirming.set(false)
    >
        <p>"This deletes “Groceries” and its 214 transactions."</p>
    </ConfirmDialog>
}
```

Props: `open`, `title`, `confirm_label`, `on_confirm`, `on_cancel` (Cancel,
the × and Esc), `saving` (the confirm button sweeps and refuses clicks),
`cancel_label` (`"Cancel"`), `close_label` (`"Close"`), `help` (a `?` before
the ×).

### `Sheet`

A modal panel pinned to an edge, with a title bar, the × and a scrolling
body. It slides in over `--react-time`; under reduced motion it just appears,
and closing never waits on an animation. A click on the backdrop closes it,
but only if the press also started there, so a text selection dragged out of
the sheet doesn't close it.

```rust
use ferrox_ui::{Sheet, SheetSide};

view! {
    <IconButton title="Menu" on_click=move |_| menu.set(true)><Icon icon=MENU/></IconButton>
    <Sheet open=menu title="Menu" on_close=move || menu.set(false)>
        <nav>…</nav>
    </Sheet>
}
```

| Prop | Default | |
|---|---|---|
| `open`, `title`, `on_close` | required | `title` is shown and is the sheet's accessible name |
| `side` | `Left` | `Left` / `Right`: full height, `min(20rem, 85vw)` wide. `Bottom`: full width, at most 85vh tall |
| `close_label` | `"Close"` | |
| `help` | none | a `?` in the title bar, before the × |
| `class` | | |

`sheet_classes(side, extra)` returns the class list.

## Help (in `core`)

Any component with a label or a title can carry help. Give it a topic and a
discreet `?` appears beside the label. It opens the topic in a popover: a
short, structured explanation that works offline. Inside a `HelpAssistant`,
the popover also offers **Ask about this**, which hands the app a
`HelpRequest`: the topic, which component, its label, and the context the app
declared around it. The app sends that to its chat, a model, or a memory
store. ferrox-ui never calls a model.

**When to give help:** one topic per thing that can carry a `?`, and only
where a user would ask. A screen of `?`s is noise. A topic too small to stand
on its own means the `?` belongs on the thing around it.

### `HelpTopic`

The explanation, as structured data (not Markdown or HTML, so nothing parses
a string or trusts tags in one). The app owns the wording and its
translation. It's cheap to clone.

```rust
use std::sync::LazyLock;
use ferrox_ui::HelpTopic;

static SPLIT_AMOUNT: LazyLock<HelpTopic> = LazyLock::new(|| {
    HelpTopic::new("split-amount", "Amounts on a split")          // id, title
        .paragraph("Each line moves money into or out of one account.")
        .list(["Positive: money comes in", "Negative: money goes out"])
        .table(["Sign", "Means"], [["+", "in"], ["-", "out"]])
        .guide("/help#split-amount")                             // "Full guide"
});
```

The `id` is stable: an assistant and a guide find the topic by it. The
`title` names the `?`.

### The `help` prop

| Component | Where the `?` sits | Also |
|---|---|---|
| `Field` | right after the label, outside the `<label>` | `help_skip_tab`, `help_shares_value` |
| `Checkbox` | after its words, outside the `<label>` (clicking it never ticks the box) | `help_skip_tab`, `help_shares_value` ("true" / "false") |
| `Switch` | beside the switch, not inside it | `help_skip_tab` |
| `Toolbar` | right after the title | |
| `Dialog`, `ConfirmDialog`, `Sheet` | in the title bar, just before the × | |

```rust
<Field label="Amount" help=SPLIT_AMOUNT.clone()>
    <Input value=amount />
</Field>
```

`help` takes a `MaybeHelp`, which converts from a `HelpTopic` or an
`Option<HelpTopic>`. `help_skip_tab` keeps the `?` out of the Tab order for a
field in a row (it stays clickable and reachable by a screen reader).

### `HelpButton`

The `?` on its own, for anywhere else: beside a heading, in a table header.
The components above place one for you.

```rust
<h2>"Statement balance" <HelpButton topic=STATEMENT_BALANCE.clone() /></h2>
```

- **It names what it explains:** "Help: {title}", as its label and tooltip,
  so a screen reader knows which `?` it landed on.
- The popover is the browser's own: Esc or a click elsewhere closes it, one
  is open at a time, and Esc gives focus back to the `?`. It sits below the
  `?` (above when there's no room), stays inside the viewport, and follows
  the page as it scrolls.
- Props: `topic`, `skip_tab`, `kind` / `about` / `value` (what a request says
  carries the `?`; components set these), `help_label` (`"Help"`),
  `guide_label` (`"Full guide"`), `ask_label` (`"Ask about this"`), `class`.
- Put it where block content is allowed: not inside a `<label>` or a
  `<button>`.

### `HelpScope`: context for an assistant

Declare what an assistant should know where you know it. Scopes nest; a
request gathers them outermost first. A scope reaches only its own children.

```rust
<HelpScope context=[("mcp_server", "conrad"), ("domain", "bookkeeping")]>
    <HelpScope context=[("screen", "reconcile")]>
        <HelpScope context=move || vec![
            ("entity".into(), "account".into()),
            ("account_id".into(), account.get()),
        ]>
            <Field label="Statement balance" help=STATEMENT_BALANCE.clone()> … </Field>
        </HelpScope>
    </HelpScope>
</HelpScope>
```

`context` is an array or a `Vec` of pairs, or a closure returning them for
context that follows the page. It's read when the user asks, so it's always
current.

**Recommended keys.** The keys are free-form; these names let an assistant
(and a memory store such as Patina) rely on them:

| Key | Meaning | Example |
|---|---|---|
| `domain` | the app or subject area | `bookkeeping` |
| `screen` | the screen the user is on | `reconcile` |
| `section` | a part of that screen | `splits` |
| `entity` | the kind of thing in view | `transaction` |
| `<entity>_id` | its id, so a tool can fetch it | `transaction_id` |
| `mcp_server` | the MCP server that knows about this area | `conrad` |
| `mcp_tool` | the tool best placed to answer here | `conrad_get_transaction` |

Put `mcp_server` in a scope at the app's root, and `mcp_tool` and ids at the
screen or section that knows them. Stable mappings can live in the app's
`on_ask` handler instead. A rule of thumb: the page declares what only the
page knows, such as the current entity's id. MCP keys are **hints**: which
tools a model may call is decided by the client, never by the page, and the
model may pick a better tool. `domain`, `screen` and `entity` map onto
memory tags (Patina's `recall_tagged`).

### `HelpAssistant` and `HelpRequest`

Wrap the app (or a part of it). Every help popover inside offers "Ask about
this".

```rust
use ferrox_ui::{HelpAssistant, HelpRequest};

view! {
    <HelpAssistant on_ask=move |request: HelpRequest| chat.ask(request)>
        <Router>…</Router>
    </HelpAssistant>
}
```

| `HelpRequest` field | |
|---|---|
| `topic_id`, `topic_title` | the topic |
| `component` | what carries the `?`: `field`, `checkbox`, `switch`, `toolbar`, `dialog`, `alertdialog`, `sheet`, or `help` for a bare `HelpButton` |
| `label` | that component's label or title |
| `context` | every enclosing scope's pairs, outermost first |
| `value` | the control's value, only when the component opted in |

`to_prompt_context()` renders it as plain lines for a prompt:

```text
topic: split-amount (Amounts on a split)
component: field
label: Amount
screen: reconcile
account_id: checking
```

`HelpRequest::new(&topic, component, label)` with `.with_context(…)` and
`.with_value(…)` builds one, for testing a handler. Without a
`HelpAssistant`, help is just the topics; nothing else changes.

### Privacy

A request carries only what the developer declared: the topic, the
component, its label, and scope context. A control's **value** is sent only
when that component says `help_shares_value=true`. `PasswordInput` never
offers its value, so a password can't be sent even then. Keep ids and
private details out of `HelpScope` unless the assistant should see them.

## CSS classes (core)

All of these are injected by `init()`, built only from theme tokens, and
selected by class only.

| Sheet | Classes |
|---|---|
| `a11y.css` | `.sr-only`: read out by screen readers, takes no space |
| `focus.css` | the focus ring for `.btn` and `.icon-btn` |
| `buttons.css` | `.btn`, `.btn--{primary,secondary,cancel,danger}`, `.btn--quiet`, `.btn--{sm,lg}`, `.is-saving`; `.icon-btn`, `.icon-btn--danger`, `[aria-pressed='true']` |
| `icon.css` | `.icon`: an SVG icon as a block with no pointer events |
| `toolbar.css` | `.toolbar`, `.toolbar--compact`, `.toolbar__title`, `.toolbar__group`, `.toolbar__heading` (a title with its `?`) |
| `spinner.css` | `.spinner`, `.spinner--{region,inline,field}`, `.spinner-status`, `.spinner-status--region`, `.spinner-overlay`, `.spinner-overlay__text`; keyframes `ferrox-spin`, `ferrox-spinner-appear` |
| `alert.css` | `.alert`, `.alert--{info,success,warning,error}`, `.alert__icon`, `.alert__body`, `.alert__dismiss` |
| `help.css` | `.help-btn` (the `?`, with `.icon-btn`), `.help-popover`, `__title`, `__body`, `__paragraph`, `__list`, `__table` / `__heading` / `__cell`, `__actions`, `__link`, `__ask`; `[data-side]` below or above |

## CSS classes (overlay)

| Sheet | Classes |
|---|---|
| `modal.css` | `.dialog-open` on `<body>`: no page scroll while a modal is open |
| `dialog.css` | `.dialog`, `.dialog--wide`, `.dialog::backdrop`, `.dialog__title-bar`, `.dialog__title`, `.dialog__help`, `.dialog__close`, `.dialog__body`, `.dialog__footer` |
| `sheet.css` | `.sheet`, `.sheet--{left,right,bottom}`, `.sheet::backdrop`, `.sheet__title-bar`, `.sheet__title`, `.sheet__help`, `.sheet__close`, `.sheet__body` |

These are injected only when the `overlay` feature is on.

A unit test (`css.rs`) fails if any shipped selector doesn't start with a
class.

## Requirements

- `leptos` 0.8 (CSR) and a browser (`web-sys`).
- ferrox-theme's stylesheet linked in the app's `index.html`
  (ferrox-theme-build). Every component reads its tokens.
- ferrox-icons, used for Alert's icons and the close buttons (a dependency,
  not something the app sets up).
- `overlay`: a browser with `<dialog>`/`showModal()` and `@starting-style`
  (Chrome 117+, Firefox 129+, Safari 17.5+). Without `@starting-style` a
  sheet just appears instead of sliding.
- Help: a browser with the Popover API (`popover`, `popovertarget`; Chrome
  114+, Firefox 125+, Safari 17+).

## Testing

- `cargo test -p ferrox-ui`: class lists, roles, and the class-only rule on
  every sheet.
- The demo (`demo/`) shows every component: `/buttons`, `/feedback`,
  `/overlay`, `/forms` and `/help` (with a fake assistant that shows the
  request it receives). Its Playwright tests (`demo/e2e/tests/*.spec.ts`)
  run in Chromium and Firefox through `scripts/release-check.sh`.

## Planned

From `plans/ferrox-ui.md`, one group per piece, each with a demo page and e2e
tests: `forms` (Combobox, then its docs), `navigation` (Side
menu, on `Sheet`), `layout` (Panel), `data` (Data grid). `core` later gains
Skeleton, Badge, Card and a theme toggle. `overlay` later gains
`TypeToConfirmDialog` (after `forms`), Toast, Popover and Tooltip, and the
phone variants of Dialog (a bottom sheet, full screen) when an app needs them.
Help later gains a topic registry (checked `related` links, a `/help` guide
rendered from the same topics) and `help` on new components as they land.

## Licence

`MIT OR Apache-2.0`.
