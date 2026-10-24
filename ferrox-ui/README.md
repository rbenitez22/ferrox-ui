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
ferrox-ui = { git = "https://github.com/rbenitez22/ferrox-ui", tag = "v0.1.0", features = ["overlay", "forms"] }
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
| `forms` | `forms` | `Field`, `Input`, `Textarea`, `PasswordInput`, `Checkbox`, `Switch`, `Select`, `Combobox` |
| `navigation` | `navigation` (turns on `overlay`; brings leptos_router) | `NavList`, `NavLink`, `NavGroup`, `SideMenu`, `SideMenuButton` |
| `layout` | `layout` | `Panel` (also collapsible), `PanelTone`, `HeadingLevel`, `Card` |
| `data` | `data` | `DataView` (with infinite scroll), `DataLayout`, `CardSize`, `DataLayoutSwitch`, `DataLayoutLabels`, `Pagination`, `PaginationLabels`, `page_window`, `PageItem` |
| `menus` | `menus` | `Menu`, `MenuTrigger`, `MenuAlign`, `MenuItem`, `MenuLink`, `MenuDivider`, `SplitButton`, `ContextMenu` |

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
    <IconButton label="Search" on_click=move |_| open_search()><Icon icon=SEARCH/></IconButton>
    <IconButton label="Camera" pressed=camera on_click=move |_| camera.update(|c| *c = !*c)><Icon icon=CAMERA/></IconButton>
    <IconButton label="Remove line" danger=true on_click=move |_| remove()><Icon icon=DELETE/></IconButton>
}
```

| Prop | |
|---|---|
| `label` | required: the button's only name (`aria-label`, and its tooltip) |
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
            <IconButton label="Search"><Icon icon=SEARCH/></IconButton>
            <IconButton label="Camera" pressed=camera><Icon icon=CAMERA/></IconButton>
        </ToolbarGroup>
        <ToolbarGroup>
            <IconButton label="Help"><Icon icon=HELP/></IconButton>
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
    {move || signing_in.get().then(|| view! { <LoadingOverlay label="Signing in…"/> })}
}
```

- `Spinner`: `label` (required; read out by screen readers, not shown),
  `kind` (`Region` by default, `Inline` or `Field`), `class`. It renders
  `role="status"`.
- `LoadingOverlay`: `label` (optional; shown under the spinner and read out;
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
        <Alert severity=AlertSeverity::Error on_close=move |_| error.set(None)>{e}</Alert>
    })}
}
```

| Prop | Default | |
|---|---|---|
| `severity` | required | `Info`, `Success`, `Warning` (non-blocking: if it should stop a submit, it's an `Error`), `Error` |
| `icon` | `true` | draws the severity's icon (ferrox-icons `INFO`, `SUCCESS`, `WARNING`, `ERROR`), so severity doesn't rest on colour alone |
| `on_close` | none | shows a close button (×) that calls it; the caller removes the alert |
| `close_label` | `"Close"` | the close button's label |
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

- **`open` drives it.** Every way out (Esc, the ×, the Cancel button, a
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
slot.** Pass the actions and the component fixes the order: cancel,
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
| `open`, `title`, `on_close` | required | `on_close` is called by Esc, the × and the Cancel button: the same act |
| `affirm` | none | the last button; `DialogAction::new(label, fn)`, plus `.danger()` for a destructive commit, `.saving(signal)`, `.disabled(signal)` |
| `alternate` | none | a lighter way to finish, before `affirm`, always secondary; Enter never reaches it |
| `cancel_label` | `"Cancel"` | |
| `no_cancel` | `false` | no Cancel button (the × and Esc still close) |
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
    <IconButton label="Menu" on_click=move |_| menu.set(true)><Icon icon=MENU/></IconButton>
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

## forms (feature `forms`)

A `Field` puts a label, one control and at most one message together. Each
control is bound two-way to an `RwSignal` (`String` for text, `bool` for a
checkbox or switch): the control writes what the user does, and setting the
signal from code updates the control.

**Where a message goes.** A problem one field can fix goes under that field
(`error`). Anything cross-field, or a server refusal no single field owns,
goes in an `Alert` at the top of the form. A problem never goes in both. A
field shows a hint *or* an error, never both: once the field is wrong, the
message under it says what's wrong.

```rust
use ferrox_ui::{Alert, AlertSeverity, Field, Input, InputKind};

let email = RwSignal::new(String::new());
let email_error = Signal::derive(move || {
    let e = email.get();
    (!e.is_empty() && !e.contains('@')).then(|| "Enter an email address, like name@example.com".to_string())
});

view! {
    {move || refused.get().map(|why| view! { <Alert severity=AlertSeverity::Error>{why}</Alert> })}
    <Field label="Email" hint="Where receipts go" error=email_error>
        <Input value=email kind=InputKind::Email autocomplete="email" />
    </Field>
}
```

**The wiring is automatic.** The control inside a `Field` takes the field's
id (so the `<label for>` names it), `aria-describedby` (the message, while
there is one) and, while there's an error, `aria-invalid` and `.is-invalid`.
Call sites pass no ids, and every field on a page gets a unique one. Put
**one control per field**. Only the field's own children see its wiring: a
control after the field, or outside any field, generates its own id and has
no description.

`Input`, `Textarea`, `PasswordInput`, `Select` and `Combobox` go inside a
`Field`. `Checkbox` and `Switch` carry their own label and don't.

| Component | Use it for |
|---|---|
| `Input` | one line of text: a name, an email, a search |
| `Textarea` | text that may run to several lines |
| `PasswordInput` | a password, to sign in or to set one (with a strength meter) |
| `Checkbox` | a yes/no that a later Save sends |
| `Switch` | an on/off that takes effect as it flips: a filter, a preference written straight back |
| `Select` | a short, fixed list of choices |
| `Combobox` | a long list, filtered by typing |

### `Field`

```rust
use ferrox_ui::Field;

view! {
    <Field label="Name" hint="As it appears on statements" error=name_error help=NAME_HELP.clone()>
        <Input value=name autofocus=true />
    </Field>
}
```

| Prop | | |
|---|---|---|
| `label` | required | the `<label>`, pointing at the control |
| `hint` | none | guidance while the field is fine; hidden while there's an error |
| `error` | none | the field's problem. Present and not empty means invalid |
| `help`, `help_skip_tab`, `help_shares_value` | | a `?` after the label (see [the `help` prop](#the-help-prop)) |
| `class` | | appended to `.field` |
| children | | the one control |

The message is tied to the control by `aria-describedby`, so a screen reader
reads it with the control. An error never times out; it clears when the
field is right. `field_classes(extra)` returns the class list.

### `Input`

A one-line text field (`.form-input`).

```rust
use ferrox_ui::{Input, InputKind};

view! {
    <Field label="Name"><Input value=name autofocus=true /></Field>
    <Input value=query kind=InputKind::Search placeholder="Search accounts" />
}
```

| Prop | Default | |
|---|---|---|
| `value` | required | `RwSignal<String>` |
| `kind` | `Text` | `Text`, `Email`, `Search`, `Tel`, `Url`, `Number` (the `type`). Passwords are `PasswordInput` |
| `placeholder`, `name`, `autocomplete` | none | `autocomplete` is the browser hint (`"email"`, `"off"`, …) |
| `required` | `false` | |
| `readonly`, `disabled` | `false` | may be signals |
| `autofocus` | `false` | where a `Dialog` puts focus when it opens |
| `class` | | |

### `Textarea`

A multi-line text field (`.form-textarea`). It's resized by hand
(vertically), and Enter is a newline: a `Dialog` never submits from it.

```rust
use ferrox_ui::Textarea;

view! { <Field label="Notes"><Textarea value=notes rows=6 /></Field> }
```

Props: `value` (required), `rows` (4), `placeholder`, `name`, `required`,
`readonly`, `disabled`, `autofocus`, `class`.

### `PasswordInput`

A password field (`.form-input` inside `.password-input`) with a show/hide
button at its right end. The button is a real `<button>` with one label
("Show password"); `aria-pressed` says whether the password is showing, and
the eye is crossed out while it is.

```rust
use ferrox_ui::PasswordInput;

view! {
    <Field label="Password"><PasswordInput value=password /></Field>

    <Field label="New password" hint="At least four words, or a long mix">
        <PasswordInput value=new_password autocomplete="new-password" strength=true min_bits=70.0 />
    </Field>
}
```

| Prop | Default | |
|---|---|---|
| `value` | required | `RwSignal<String>` |
| `autocomplete` | `"current-password"` | `"new-password"` when setting one, so a password manager offers to generate it |
| `strength` | `false` | a meter under the field while it has text: a four-step bar and a word. The field is described by its hint and the meter together |
| `min_bits` | `80.0` | the entropy, in bits, that counts as `Strong` |
| `strength_labels` | English | `StrengthLabels::new("Très faible", "Faible", "Moyen", "Fort")`, for another language |
| `show_label` | `"Show password"` | the button's label |
| `placeholder`, `name`, `required`, `disabled`, `autofocus`, `class` | | `class` goes on the input |

The meter's colours come from its level's class, and the bits are on
`data-bits`, not in the text. A help request never carries a password, even
inside a `Field` with `help_shares_value`.

The calculation is public, for checking a password on submit:

```rust
use ferrox_ui::{password_strength, StrengthLevel};

let strength = password_strength(&new_password.get(), 80.0);  // PasswordStrength { bits, level }
if strength.level != StrengthLevel::Strong { /* say so under the field */ }
```

- `password_entropy(&str) -> f64`: length × log2 of the size of the
  character sets used. It counts characters, not bytes ("é" is one).
- `strength_level(bits, min_bits) -> StrengthLevel`: `VeryWeak` under 25
  bits, `Weak` under 40, `Fair` under `min_bits`, then `Strong`.
  `StrengthLevel::class()` is its meter class.
- `password_strength(&str, min_bits) -> PasswordStrength`: both.

The new-password and confirm pair stays in webapp-lib, built from two
`PasswordInput`s.

### `Checkbox`

A checkbox and its words as one `<label>`, so the words are part of the
target. **A checkbox states an intention that a later Save carries out.**
For a setting that takes effect the moment it flips, use a `Switch`.

```rust
use ferrox_ui::Checkbox;

view! {
    <Checkbox checked=recurring label="Repeat every month" hint="On the same day each month" />
    <Checkbox checked=agreed label="I accept the terms" error=terms_error required=true />
}
```

| Prop | | |
|---|---|---|
| `checked` | required | `RwSignal<bool>` |
| `label` | required | the words beside the box |
| `hint`, `error` | none | one message under it, as a `Field`'s (never both); an error also marks the box |
| `help`, `help_skip_tab`, `help_shares_value` | | a `?` after the words, outside the `<label>`, so clicking it never ticks the box. A shared value is `"true"` / `"false"` |
| `name`, `required`, `disabled`, `class` | | |

It's the native checkbox, coloured with `accent-color` and showing the
theme's focus ring. Space ticks it. `checkbox_classes(invalid, extra)`
returns the class list.

### `Switch`

An on/off that **is** the act: a filter over what the screen shows, or a
preference written straight back. A field that a dialog's Save sends stays a
`Checkbox`. It's a `<button role="switch" aria-checked>` with a track, a knob
and the words inside, so the words name it and are part of the target.

```rust
use ferrox_ui::Switch;

view! { <Switch checked=show_archived label="Show archived accounts" /> }
```

Props: `checked` (required, `RwSignal<bool>`), `label` (required), `disabled`
(off and unusable: still loading, or not this user's to change), `help` and
`help_skip_tab` (a `?` beside the switch, not inside it), `class`. A click or
Space flips it, and the knob slides across. `switch_classes(extra)` returns
the class list.

### `Select`

A dropdown of fixed choices: the native `<select>` (`.form-select`) with the
theme's own caret. `value` is the chosen option's `value`, or empty for none.

```rust
use ferrox_ui::{Select, SelectOption};

let currencies = Signal::derive(|| vec![
    SelectOption::new("EUR", "Euro"),
    SelectOption::new("USD", "US dollar"),
]);

view! {
    <Field label="Currency">
        <Select value=currency options=currencies placeholder="Choose a currency" />
    </Field>

    <Field label="Category">
        <Select
            value=category
            options=categories
            loading=categories_loading
            on_refresh=move || reload_categories()
            refreshing=categories_refreshing
            on_create=move || new_category_dialog.set(true)
            create_label="New category…"
        />
    </Field>
}
```

| Prop | Default | |
|---|---|---|
| `value` | required | `RwSignal<String>` |
| `options` | required | `Signal<Vec<SelectOption>>`; `SelectOption::new(value, label)` |
| `placeholder` | `"Select…"` | the first entry: shown while `value` is empty, and can't be chosen |
| `loading` | `false` | the options are on their way: disabled, `aria-busy`, and the placeholder reads `loading_label` (`"Loading…"`) |
| `on_refresh` | none | adds a refresh button beside the select (`refresh_label`, `"Refresh"`) |
| `refreshing` | `false` | a spinner takes the refresh icon's place, labelled `refreshing_label` (`"Refreshing…"`); the button stays put and refuses clicks |
| `on_create` | none | adds a `+` button after the select (and after refresh), labelled `create_label` (`"New…"`); disabled with the select |
| `name`, `required`, `disabled`, `class` | | `class` goes on the `<select>` |

The `<select>` is rendered once. Only its options and which one is selected
react, so it keeps focus and its choice when the options change. Loading the
options is the caller's job; webapp-lib's `SelectMenu` is an adapter that
fetches them. So is creating one: `on_create` only says "new" was pressed.
Usually the app opens a dialog, then adds the option and sets `value`.

### `Combobox<T>`

A text field that filters a list and lets one item be chosen: the ARIA
combobox pattern (`role="combobox"`, a `listbox` of `option`s,
`aria-activedescendant` for the active one). Use it when a `Select` would be
too long to scan.

```rust
use ferrox_ui::{Combobox, Field};

#[derive(Clone)]
struct Account { id: String, name: String }

// `accounts: Signal<Vec<Account>>` is loaded by the caller.
let account_text = RwSignal::new(String::new());
let chosen = RwSignal::new(None::<Account>);

view! {
    <Field label="Account" error=account_error>
        <Combobox
            items=accounts
            key_of=|a: &Account| a.id.clone()
            label_of=|a: &Account| a.name.clone()
            text=account_text
            on_select=move |a: Option<Account>| chosen.set(a)
            placeholder="Type to search"
            on_refresh=move || reload_accounts()
            refreshing=accounts_refreshing
            on_create=move |name: String| create_account(name)
        />
    </Field>
}
```

| Prop | Default | |
|---|---|---|
| `items` | required | `Signal<Vec<T>>`, owned by whatever loaded them; `T: Clone + Send + Sync + 'static` |
| `key_of` | required | a stable id per item (`data-key` on its option) |
| `label_of` | required | the text shown, and the text matched against |
| `text` | required | `RwSignal<String>`: what's in the field. Choosing an item sets it to the item's label |
| `on_select` | required | `Some(item)` when one is chosen; `None` when the text is cleared, or the field is left with text that names no item |
| `placeholder`, `name`, `required`, `disabled`, `autofocus`, `class` | | `class` goes on the input |
| `clearable` | `true` | a × that clears the text and the choice (`clear_label`, `"Clear"`) |
| `toggleable` | `true` | a chevron that shows and hides the whole list (`toggle_label`, `"Show all"`) |
| `on_refresh` | none | adds a refresh control (`refresh_label`, `"Refresh"`) |
| `refreshing` | `false` | a spinner in the refresh control's place (`refreshing_label`, `"Refreshing…"`) |
| `on_create` | none | `UnsyncCallback<String>`: offers to create an item from the typed text when nothing matches; gets the text trimmed |
| `create_label` | `Create “{text}”` | `Callback<String, String>`: the offer's text, given the typed text |
| `empty_label` | `"No matches"` | shown in the open list when nothing matches |

How it behaves:

- **Typing** filters by `label_of`, case-insensitive "contains"
  (`matches_text(label, text)` is that test, public).
- **Keys:** ↓ / ↑ move the active option, opening the list if it's closed,
  and stop at the ends. Enter chooses the active option. Tab chooses it and
  moves on. Esc closes the list, or clears the text if it's already closed.
  Inside a `Dialog`, Enter on an option chooses it without submitting, and
  Esc closes the list without closing the dialog.
- **A click** on an option chooses it.
- **Creating** (with `on_create`): while the trimmed text is not empty and
  matches no item, a "Create “milk”" button sits under the list. A click
  presses it, and so does Enter when no option is active. It's a button,
  not an option: creating is a different act from choosing. The list closes
  and `on_create` gets `"milk"`. What happens next is the app's: add it to
  a static list, `POST` it, open a dialog for more. Typically the app then
  adds the item to `items`, sets `text` to its label and treats it as
  chosen.
- **Its controls** (clear, the chevron, refresh, create) are buttons out of the Tab
  order, so a row of comboboxes costs no extra keystrokes. Pressing one
  doesn't take focus from the field.
- **The list is in the top layer** (a popover, with the offer to create
  under it), placed under the field and
  as wide as it, and it follows the field as the page scrolls. A `Dialog`
  body or any `overflow` parent can't clip it.

It doesn't fetch. webapp-lib's `AutocompleteInputModel` is the adapter that
loads `items`. Groups and multi-select are planned.

## navigation (feature `navigation`)

Links that say where the user is, and a side menu to hold them. The links
are leptos_router's `<A>`, so every component here goes inside a
`<Router>`. The feature turns on `overlay` (the side menu is a `Sheet`) and
brings in leptos_router. It's the only group that does.

| Use it for | Use |
|---|---|
| a list of places: a sidebar, a settings page's sections | `NavList` of `NavLink`s |
| places that belong together (one service's pages, the account's) | `NavGroup` inside the list, collapsible if it's long or rarely used |
| the app's navigation on a phone, or any screen without room for a sidebar | `SideMenu`, opened by a `SideMenuButton` |

```rust
use ferrox_ui::{NavGroup, NavLink, SideMenu, SideMenuButton};

let menu = RwSignal::new(false);

view! {
    <Router>
        <header class="app-bar">
            <SideMenuButton open=menu />
            <span class="app-bar__title">"Groceries"</span>
        </header>
        <SideMenu open=menu title="Groceries">
            <NavLink href="/" exact=true>"Lists"</NavLink>
            <NavLink href="/load">"Load Data"</NavLink>
            <NavGroup label="Product database" collapsible=true>
                <NavLink href="/units">"Units"</NavLink>
                <NavLink href="/categories">"Categories"</NavLink>
                <NavLink href="/products">"Products"</NavLink>
            </NavGroup>
            <NavGroup label="Account">
                <NavLink href="/account">"Account"</NavLink>
                <NavLink href="/logout">"Logout"</NavLink>
            </NavGroup>
        </SideMenu>
        <Routes fallback=|| "Not found">…</Routes>
    </Router>
}
```

Put the `SideMenu` outside `<Routes>`, beside the header, so it stays
mounted while the user moves between pages.

### `NavList`, `NavLink`

A labelled `<nav>` landmark holding a `<ul>`, one `<li>` per link, so a
screen reader hears the landmark's name and how many links it holds.

```rust
use ferrox_icons::SEARCH;
use ferrox_ui::{NavLink, NavList};

view! {
    <NavList label="Settings">
        <NavLink href="/settings" exact=true>"General"</NavLink>
        <NavLink href="/settings/search" icon=SEARCH>"Search"</NavLink>
    </NavList>
}
```

- `NavList`: `label` (required, the landmark's name), `class`, children
  (`NavLink`s and `NavGroup`s).
- `NavLink`: `href` (required), `exact`, `icon` (an `IconData`, drawn before
  the words and hidden from screen readers), `class` (on the link), children
  (the words). It renders an `<li>`, so it goes in a `NavList` or a
  `NavGroup`.

**The current page.** The router sets `aria-current="page"` on the link to
the page the user is on, and that's what tints it in the primary fill. A
link is current on its own path and on every path below it: `/accounts` is
current on `/accounts/42`. `exact` limits it to its own path, for a link
whose sub-pages have links of their own. `/` is current only on `/`.

Each link is a full-width row. It answers hover and press in colour only,
shows the theme's focus ring, and is at least 44px tall on a phone.
`nav_list_classes(extra)` and `nav_link_classes(extra)` return the class
lists.

### `NavGroup`

Links that belong together, under a heading that names them. A screen reader
hears "Product database, list, 3 items".

```rust
use ferrox_ui::NavGroup;

let remember = RwSignal::new(false);   // store it wherever the app keeps preferences

view! {
    <NavGroup label="Account">…</NavGroup>                                   // plain
    <NavGroup label="Product database" collapsible=true>…</NavGroup>         // starts collapsed
    <NavGroup label="Reports" collapsible=true start_expanded=true>…</NavGroup>
    <NavGroup label="Admin" collapsible=true expanded=remember>…</NavGroup>   // the app owns it
}
```

| Prop | Default | |
|---|---|---|
| `label` | required | the heading, and the name of the group's list |
| `collapsible` | `false` | the heading is a button that shows and hides the links |
| `start_expanded` | `false` | where a collapsible group starts |
| `expanded` | none | `RwSignal<bool>`, two-way, owned by the app; overrides `start_expanded` |
| `class` | | |
| children | | `NavLink`s; one level only, no group inside a group |

- **Plain:** the heading is quiet text (small, muted), not focusable, and a
  hairline splits the group from what comes before it.
- **Collapsible:** the heading is a `<button aria-expanded aria-controls>`
  with a chevron that points right while collapsed and turns down once
  expanded. Enter and Space toggle it. A collapsed group's links are
  `hidden`, so Tab skips them.
- **The current page is never hidden.** When the route moves to one of a
  collapsed group's links, or the page opens on one, the group expands
  itself. That happens on arrival only, so the user can still collapse the
  group around the page they're on.
- **Inside a `SideMenu`** a group's own state starts over each time the
  menu opens (the menu's body is mounted only while open): collapsed, unless
  the current page is in it. To remember it, pass `expanded`.

`nav_group_classes(collapsible, extra)` returns the class list.

### `SideMenu`, `SideMenuButton`

A `Sheet` holding a `NavList` of the children, with an optional footer
pinned to the bottom. It shares an `open` signal with its button.

```rust
use ferrox_ui::{SideMenu, SideMenuButton};

view! {
    <SideMenuButton open=menu />
    <SideMenu
        open=menu
        title="Groceries"
        help=MENU_HELP.clone()
        footer=|| view! { <span>"Version 1.4"</span> }
    >
        …
    </SideMenu>
}
```

**It closes itself.** `open` is an `RwSignal`, not `Sheet`'s `open` plus
`on_close`, because the menu sets it back to false on its own:

- when the user follows a link inside it: a plain left click on any
  `<a href>` without a `target`, so a plain `<A>` closes it too. A ctrl- or
  cmd-click opens a new tab and leaves the menu open;
- when the current page's link is clicked, though the route doesn't change;
- on any route change while it's open, such as the back button;
- on Esc, the × or a backdrop click, as any `Sheet`.

Focus then returns to the `SideMenuButton`. After a navigation, moving focus
into the new page is the app's routing concern, not the menu's.

| `SideMenu` prop | Default | |
|---|---|---|
| `open` | required | `RwSignal<bool>`, shared with `SideMenuButton` |
| `title` | required | the title bar's text, and the sheet's accessible name |
| `nav_label` | `"Sections"` | the name of the `<nav>` inside |
| `side` | `Left` | a `SheetSide` |
| `footer` | none | a closure returning a view, pinned below the links behind a hairline: a version, a legal line |
| `help` | none | a `?` in the title bar, before the × |
| `close_label` | `"Close"` | the ×'s label |
| `class` | | on the sheet, after `side-menu` |
| children | | `NavLink`s and `NavGroup`s |

`SideMenuButton` is an `IconButton` drawing `MENU`, with
`aria-haspopup="dialog"` and an `aria-expanded` that follows `open`. Props:
`open` (required), `label` (`"Menu"`, its only label and its tooltip),
`class`. `side_menu_classes(extra)` returns the sheet's class list.

Who's signed in, the app's title bar and a page frame aren't here: the bar
is the app's (or webapp-lib's), and a page frame is `layout`'s, later.

## layout (feature `layout`)

Boxes that title and hold content. No dependencies beyond `core`.

| Use it for | Use |
|---|---|
| a page's own title and content, or a titled section of a page, with its buttons (add, search) | `Panel` |
| a section most users skip, or one that loads data only some need | `Panel collapsible=true` |
| one thing in a list (a record, an item, a choice), as a whole a link or a button if it opens something | `Card` |

### `Panel`

A `<section>` named by a real heading: a header band with the title on the
left and the actions on the right, then the body. One component serves a
page's own title (`H1`) and a section inside a page (`H2`, the default).

```rust
use ferrox_icons::{Icon, PLUS, SEARCH};
use ferrox_ui::{HeadingLevel, IconButton, Panel, PanelTone};

view! {
    <Panel title="Shopping Lists" heading_level=HeadingLevel::H1>
        …
    </Panel>

    <Panel
        title="Items"
        actions=move || view! {
            <IconButton label="Search items" on_click=move |_| search()><Icon icon=SEARCH /></IconButton>
            // The app's own permission decides whether "add" shows.
            <Show when=move || can_add_items.get()>
                <IconButton label="Add item" on_click=move |_| new_item()><Icon icon=PLUS /></IconButton>
            </Show>
        }
    >
        …
    </Panel>

    <Panel title="Load Data" heading_level=HeadingLevel::H1>…</Panel>   // a bar with no buttons
    <Panel title="Transactions" flush=true>…</Panel>                     // a table runs to the edges
}
```

| Prop | Default | |
|---|---|---|
| `title` | required | the heading's text, and the section's name |
| `heading_level` | `H2` | `H1`…`H4`, for the page's outline. Every level looks the same |
| `actions` | none | a closure returning buttons (add, search, export: whatever the app needs), at the right of the band |
| `compact` | `false` | a slimmer band in body type and a tighter body, for one panel among several |
| `flush` | `false` | no padding round the body, for a table or a grid |
| `tone` | `Default` | `Warning`: something to act on. `Danger`: something that can't be undone (the title turns red too). Tints the band and the border |
| `help` | none | a `?` right after the title |
| `class` | | |
| children | | the body |

The title ends in an ellipsis rather than wrapping under the buttons, and a
band with no buttons is as tall as one with them. On a phone the band's
buttons are 44px. `panel_classes(collapsible, compact, flush, tone, extra)`
returns the class list.

**Collapsible.** The title becomes a button inside the heading, so the page's
outline keeps the heading and the whole band is the target.

```rust
view! {
    <Panel title="Danger zone" tone=PanelTone::Danger collapsible=true summary="Delete this book">
        …
    </Panel>
    <Panel title="Notes" collapsible=true expanded=notes_open summary=Signal::derive(move || format!("{} notes", count.get()))>
        …
    </Panel>
}
```

| Prop | Default | |
|---|---|---|
| `collapsible` | `false` | the title opens and closes the body (`aria-expanded`, `aria-controls`; Enter and Space) |
| `start_expanded` | `false` | where it starts |
| `expanded` | none | `RwSignal<bool>`, two-way, for an app that owns or remembers the state; overrides `start_expanded` |
| `summary` | none | one line in the band while it's shut, in body type: "3 overdue" |

- **The body isn't rendered until it's first opened**, so a shut panel that
  would load data doesn't. From then on it stays mounted and is `hidden`
  while shut, so whatever was typed in it survives.
- **While shut** the actions are hidden; the `?` stays, since
  help explains a shut panel too. On a collapsible panel the `?` sits after
  the toggle.
- The section is named by the title alone, never by the summary.
- The chevron points right while shut and down once open (no animation
  under reduced motion). Hovering the toggle tints the whole band.

### `Card`

A surface for one thing: an `<article>` with an optional heading, its own
buttons, and the body. It's the item template for a list or a grid of cards.

```rust
use ferrox_icons::{Icon, DELETE};
use ferrox_ui::{Card, IconButton};

view! {
    <Card
        title=employee.name.clone()
        href=format!("/employees/{}", employee.id)
        actions=move || view! {
            <IconButton label="Remove" danger=true on_click=move |_| remove()><Icon icon=DELETE /></IconButton>
        }
    >
        <p>{employee.role.clone()}</p>
    </Card>

    <Card title="Groceries" on_click=move || open_list()>…</Card>   // the whole card is a button
    <Card title="Note" compact=true>…</Card>                         // just a surface
}
```

| Prop | Default | |
|---|---|---|
| `title` | none | the heading; required for a clickable card |
| `heading_level` | `H3` | a card usually sits inside a panel or a section |
| `href` | none | the whole card follows this link |
| `on_click` | none | the whole card is this button (not with `href`) |
| `actions` | none | a closure returning the card's own buttons, in its top-right corner |
| `compact` | `false` | tighter padding and a body-type title, for dense lists |
| `class` | | |
| children | | the body |

**A whole card as one target, done accessibly.** With `href` the *title* is
the link; with `on_click` it's a button. Its hit area is stretched over the
card, so a click anywhere follows it, but a screen reader meets one link or
button, named by the title, not the whole card read out as a link. The
`actions` sit above that layer and stay separate targets: pressing one never
follows the card. A clickable card answers hover and press in colour (its
border and surface) and shows the focus ring round the whole card. A
clickable card without a `title` logs a warning.

`href` is a plain `<a href>`: inside a `<Router>` the router handles it, so
`layout` doesn't depend on leptos_router. Choosing records (a selected card)
is left to the list that holds them. `card_classes(clickable, compact,
extra)` returns the class list.

## data (feature `data`)

Record lists: many records drawn through one item template, laid out as
the screen needs, with their loading, error and empty states, and two ways
to reach the rest of a long list.

| Component | For |
|---|---|
| `DataView` | the list itself: one template per record (usually a `Card` in a grid, a row in a list), four layouts, its states beside it, optional infinite scroll |
| `DataLayoutSwitch` | lets the user pick the layout; sits in a `Panel`'s actions |
| `Pagination` | pages of anything; the app slices or fetches the page |

Putting them together:

```rust
use ferrox_ui::{Card, DataLayout, DataLayoutSwitch, DataView, Pagination, Panel};

let layout = RwSignal::new(DataLayout::default());
let page = RwSignal::new(1);

view! {
    <Panel title="Recipes" actions=move || view! { <DataLayoutSwitch layout=layout /> }>
        <DataView
            items=Signal::derive(move || recipes_on(page.get()))
            key_of=|r: &Recipe| r.id.clone()
            item=|r: Recipe| view! { <Card title=r.name.clone() href=format!("/recipes/{}", r.id)>…</Card> }
            label="Recipes"
            layout=layout
            loading=loading
            error=error
            on_retry=move || reload()
        />
        <Pagination page=page page_count=page_count />
    </Panel>
}
```

**Pages or infinite scroll?** The app's choice, per list. Pages
(`Pagination`) where users come back to a place: admin lists, search
results, anything worth linking to. Infinite scroll (`DataView`'s
`on_more`) for feeds read from the top. Not both on one list.

None of them fetch, sort, filter or route: whatever loads the records owns
that, and owns the signals (`layout`, `page`), so remembering a choice (in
storage, in the URL) is the app's. Selecting records, tables with columns
and sorting, and virtualised rendering for thousands of items are not here
(see Planned).

### `DataView<T>`

A record list rendered through one item template and laid out as the screen
needs, with its loading, error and empty states beside it.

```rust
use ferrox_ui::{Card, DataLayout, DataView};

view! {
    <DataView
        items=recipes
        key_of=|r: &Recipe| r.id.clone()
        item=|r: Recipe| view! { <Card title=r.name.clone() href=format!("/recipes/{}", r.id)>…</Card> }
        label="Recipes"
        layout=DataLayout::default()
        loading=loading
        error=error
        on_retry=move || reload()
        empty_label="No recipes yet."
    />
}
```

| Prop | Default | |
|---|---|---|
| `items` | required | `Signal<Vec<T>>`; `T: Clone + Send + Sync + 'static` |
| `key_of` | required | a stable key per item |
| `item` | required | renders one item: a `Card` in a grid, a row in a list |
| `label` | required | the list's accessible name |
| `layout` | `Grid(CardSize::Md)` | `DataLayout::Grid(CardSize::{Sm,Md,Lg})` (columns at least 10 / 14 / 20rem, as many as fit), `List` (full-width rows, a hairline between), `Row` (a sideways strip that snaps), `Wrap` (content-sized, wrapping: tags, chips). May be reactive |
| `loading` | `false` | no items yet: a spinner (`loading_label`, `"Loading…"`) and no list; with items: they stay, the list is `aria-busy` |
| `error` | none | an error `Alert` above the list; items already shown stay |
| `on_retry` | none | adds Retry to the error (`retry_label`, `"Retry"`) |
| `empty_label` | `"Nothing here yet."` | shown when there are no items, no error and no loading |
| `empty_view` | none | a view shown instead of `empty_label`, e.g. a line and an Add button |
| `on_more` | none | infinite scroll: called when the end of the list comes within 300px of the viewport and `has_more` |
| `has_more` | `false` | there are more pages to fetch |
| `loading_more` | `false` | a page is on its way: a spinner at the end (`loading_more_label`, `"Loading more…"`), and no new calls |
| `class` | | extra classes on the root |

**Infinite scroll.** With `on_more`, a marker (`<li role="none">`) follows
the last item, watched by an `IntersectionObserver`. Each sighting asks for
at most one page, and never while `loading` or `loading_more`. When a page
lands and the marker is still in sight (a page too short to fill the
screen), it asks again, until the screen is full or `has_more` is false. It
works whatever scrolls: the window, or an `overflow: auto` box (there it
counts once the marker shows in the box; the 300px head start applies to
the viewport only). The observer is disconnected when the view unmounts.
The first page is the app's to load; `on_more` only extends a list that has
items.

```rust
<DataView
    items=entries
    key_of=|e: &Entry| e.id.clone()
    item=entry_row
    label="Activity"
    layout=DataLayout::List
    on_more=move || load_next_page()
    has_more=has_more
    loading_more=loading_more
/>
```

The `<For>` is keyed and rendered once: an item whose key stays keeps its DOM
node and state (a loaded image, typed text) through updates, appends,
re-orders and layout changes, and is not rendered again. To show a record
that changed, give it a key that changes with it (an id plus a version), or
have the template read signals. It doesn't fetch, sort or filter.
`data_view_classes(layout, extra)` returns the class list.

### `DataLayoutSwitch`

Lets the user pick a `DataView`'s layout: a group (`role="group"`, named
"Layout") of `IconButton`s, one per choice, the current one pressed
(`aria-pressed`). It sits wherever the app wants, typically a `Panel`'s
actions.

```rust
use ferrox_ui::{DataLayout, DataLayoutSwitch, DataView, Panel};

let layout = RwSignal::new(DataLayout::default());

view! {
    <Panel title="Recipes" actions=move || view! { <DataLayoutSwitch layout=layout /> }>
        <DataView items=recipes key_of=… item=… label="Recipes" layout=layout />
    </Panel>
}
```

| Prop | Default | |
|---|---|---|
| `layout` | required | `RwSignal<DataLayout>`, two-way: the app owns it, so remembering the choice is the app's |
| `choices` | `[Grid(Md), List]` | the layouts offered, in order |
| `labels` | `DataLayoutLabels::default()` | the group's name ("Layout") and each layout's: "Small grid", "Grid", "Large grid", "List", "Strip", "Cloud". Change some with `DataLayoutLabels { list: "Rows".into(), ..Default::default() }` |
| `class` | | extra classes on the group |

Icons (`layout_icon(layout)`): `LAYOUT_GRID` for every grid size, `LIST`,
`GALLERY_HORIZONTAL` (Row), `TAGS` (Wrap).

### `Pagination`

Moves through pages of anything: a `DataView`, a table, search results.

```text
‹  1  …  5  [6]  7  …  20  ›
```

```rust
use ferrox_ui::Pagination;

let page = RwSignal::new(1);
let page_count = Signal::derive(move || total.get().div_ceil(20));

view! {
    <DataView items=Signal::derive(move || slice(page.get())) … />
    <Pagination page=page page_count=page_count />
}
```

| Prop | Default | |
|---|---|---|
| `page` | required | `RwSignal<usize>`, 1-based, two-way |
| `page_count` | required | `Signal<usize>` |
| `label` | `"Pages"` | the `<nav>`'s accessible name |
| `labels` | `PaginationLabels::default()` | `previous` ("Previous page"), `next` ("Next page"), `page` ("Page {n}", each number's name), `position` ("Page {page} of {count}", read beside the phone's `6 / 20`) |
| `class` | | extra classes on the `<nav>` |

- A `<nav aria-label>` with Previous, the numbers and Next. The current page
  is `aria-current="page"`. Previous and Next are disabled at the ends.
  With one page or none it renders nothing.
- **Which numbers show** is `page_window(current, count) -> Vec<PageItem>`
  (`Page(n)` or `Gap`), public and unit-tested. Up to seven pages all show.
  Beyond that it's always seven slots: the first and last, the current
  with one neighbour each side, and a gap (`…`, `aria-hidden`) for each
  hidden run, never a gap for a single page. Slots are a fixed width, so
  paging never moves Previous or Next.
- The numbers are keyed by page, so the one you pressed keeps focus as the
  window moves.
- **On a phone** (≤ 768px): Previous, `6 / 20`, Next, as 44px targets.
- It doesn't slice, fetch or route. The app owns `page`, so putting it in
  the URL is the app's choice. Pagination or infinite scroll is the app's
  call per list: pages where users return to a place (admin, results),
  infinite scroll for feeds.

## menus (feature `menus`)

Lists of actions that hang off the control or area that offered them.

| Need | Use |
|---|---|
| a row's or card's actions, an overflow "More" | `Menu` with `MenuTrigger::icon(MORE_VERTICAL, "More actions")` |
| a button that offers a few related actions ("Export ▾") | `Menu` with `MenuTrigger::button("Export")` |
| a user menu, a book or workspace switcher | `Menu` with `MenuTrigger::custom(view, label)`, a `header`, `current` items |
| one main action and its variants ("Save ▾") | `SplitButton` |
| the same actions on right-click, Shift+F10 or a long press | `ContextMenu` around the area, alongside a visible `Menu` |
| choosing a value, not running an action | not a menu: `Select` or `Combobox` (`forms`) |

All of them share one panel (top layer, never clipped, kept inside the
window, never scrolling sideways), one set of items (`MenuItem`,
`MenuLink`, `MenuDivider`) and one keyboard contract (ARIA's menu button).
Checkable items, submenus, a menubar and a command palette are not here
(see Planned).

### `Menu`

A list of actions behind a trigger, as ARIA's menu button has it.

```rust
use ferrox_icons::MORE_VERTICAL;
use ferrox_ui::{Menu, MenuDivider, MenuItem, MenuTrigger};

view! {
    <Menu label="Recipe actions" trigger=MenuTrigger::icon(MORE_VERTICAL, "More actions")>
        <MenuItem on_select=move || edit()>"Edit"</MenuItem>
        <MenuItem on_select=move || duplicate()>"Duplicate"</MenuItem>
        <MenuDivider />
        <MenuItem on_select=move || ask_delete()>"Delete"</MenuItem>
    </Menu>
}
```

| Prop | Default | |
|---|---|---|
| `label` | required | the menu's accessible name |
| `trigger` | required | `MenuTrigger::button("Export")` (a secondary `.btn` with a chevron; `MenuTrigger::Button { text, family }` for another family), `MenuTrigger::icon(icon, label)` (an icon button named by `label`), `MenuTrigger::custom(view, label)` (the caller's view in a plain button), `MenuTrigger::chevron(label, family)` (a `.btn` showing only a chevron: `SplitButton`'s second half) |
| `align` | `MenuAlign::End` | which edge of the trigger the panel lines up with: `End` for a menu at a row's end, `Start` for one read from its button |
| `header` | none | a view above the items, outside the menu itself (who is signed in) |
| `disabled` | `false` | the trigger is disabled |
| `class` | | extra classes on the `.menu` wrapper |
| children | | `MenuItem`s, `MenuLink`s and `MenuDivider`s |

- **Opening:** a click, Enter or Space on the trigger opens it with the first
  item focused; ↓ opens on the first item, ↑ on the last.
- **In the menu:** ↑ / ↓ move, wrapping; Home / End; a letter moves to the
  next item starting with it; Enter or Space chooses.
- **Closing:** choosing (the menu closes, focus returns to the trigger, then
  the action runs, so a dialog it opens takes focus from there), Esc (focus
  back to the trigger; an enclosing `Dialog` stays open), Tab (focus moves
  on), a press outside.
- **The panel** is a popover in the top layer: below the trigger, above
  when there's no room, lined up by `align`, kept inside the viewport,
  following the trigger while open. No `overflow` parent clips it. Items
  are built on first opening, then kept, so a long list of row menus costs
  little. On a phone, items are 44px tall.
- **Width:** at least 12rem, at most `--menu-max-width` (22rem by default,
  never wider than the window). Past it, a label ends in an ellipsis; a
  panel never scrolls sideways (a very long menu scrolls down). Set the
  variable on `:root` for every menu, or on one menu's `class`:
  `.wide-menu { --menu-max-width: 34rem; }`.

### Items

```rust
<MenuItem icon=EDIT on_select=move || edit()>"Edit"</MenuItem>
<MenuItem on_select=move || archive() disabled=Signal::derive(move || !can_archive.get())>"Archive"</MenuItem>
<MenuDivider />
<MenuItem icon=DELETE danger=true on_select=move || ask_delete()>"Delete"</MenuItem>

<MenuItem current=Signal::derive(move || book.get() == "Household") on_select=move || book.set("Household")>
    "Household"
</MenuItem>
<MenuLink href="/settings">"Settings"</MenuLink>
```

| | | |
|---|---|---|
| `MenuItem` | `on_select` (required), `icon`, `danger`, `disabled`, `current` | a `<button role="menuitem">` |
| `MenuLink` | `href` (required), `icon` | an `<a role="menuitem" href>`: inside a `<Router>` the router follows it, no page load. Space follows it too, as on a button |
| `MenuDivider` | | a `role="separator"` line between groups |

- **`icon`** is drawn before the text, muted.
- **`danger`**: a destructive action, in the danger colour with a danger
  tint on hover and focus.
- **`disabled`**: still shown and reachable with the arrows and letters (ARIA
  asks this of menu items), `aria-disabled`, faded; choosing it does
  nothing and the menu stays open.
- **`current`**: the one already in force (the open book). A check at the
  end, the primary's text colour, `aria-current="true"`, and still
  choosable: saying "you are here" isn't refusing to go there.

Choosing a `MenuItem` closes the menu and returns focus to the trigger
before `on_select` runs. Choosing a `MenuLink` closes the menu; the link
then navigates. `menu_item_classes(danger, current)` returns an item's
classes.

### `SplitButton`

A main action and, joined to it, a chevron that opens related actions.

```rust
use ferrox_ui::{MenuItem, SplitButton};

view! {
    <SplitButton text="Save" on_click=move |_| save() menu_label="More save options" saving=saving>
        <MenuItem on_select=move || save_as_draft()>"Save as draft"</MenuItem>
        <MenuItem on_select=move || save_and_close()>"Save and close"</MenuItem>
    </SplitButton>
}
```

| Prop | Default | |
|---|---|---|
| `text` | required | the main button's text |
| `on_click` | required | the main action |
| `menu_label` | required | the chevron's name, and the menu's |
| `family` | `Primary` | both halves |
| `saving` | `false` | the main button sweeps and refuses clicks; the menu can't open |
| `disabled` | `false` | both halves |
| `class` | | extra classes on the group |
| children | | the menu's items |

Two buttons, two tab stops, two names. The halves join into one shape
(square inner corners, a faint hairline between) and don't grow under the
pointer, so one never rides over the other. The menu hangs from the
group's end.

### `ContextMenu`

The same panel, items and keyboard as `Menu`, opened from an area (a row,
a card) instead of a trigger.

```rust
use ferrox_ui::{ContextMenu, MenuItem};

view! {
    <ContextMenu label="Recipe actions" menu=move || view! {
        <MenuItem icon=EDIT on_select=move || edit()>"Edit"</MenuItem>
        <MenuItem icon=DELETE danger=true on_select=move || ask_delete()>"Delete"</MenuItem>
    }>
        <Card title=recipe.name.clone() href=…>…</Card>
    </ContextMenu>
}
```

| Prop | Default | |
|---|---|---|
| `label` | required | the menu's accessible name |
| `menu` | required | a closure returning the `MenuItem`s, `MenuLink`s and `MenuDivider`s |
| `header` | none | a view above the items |
| `disabled` | `false` | the browser's own menu stays |
| `class` | | extra classes on the area's wrapper |
| children | | the area |

- **Opening:** a right-click in the area, at the pointer (the browser's own
  menu is suppressed there, and only there); the Menu key or Shift+F10
  while focus is in the area, under the focused element; a long press on
  touch (half a second; a touch that moves, as in scrolling, never opens
  it, and the lift doesn't click what was under it).
- At the pointer it opens below and to the right, flipping left and up near
  the window's edges. The first item takes focus; closing (choosing, Esc,
  Tab) returns focus to what had it before. A press outside closes it
  without moving focus; a right-click elsewhere in the area moves it there.
- The wrapper is `display: contents`: the area keeps its own layout.
- **Nothing shows that a context menu exists.** Offer the same actions
  somewhere visible too, such as a row's ⋮ `Menu`. It's a shortcut, never
  the only way.

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
| `SideMenu` | in the title bar, just before the × | a request names it as a `sheet` |
| `Panel` | right after the title (after the toggle, on a collapsible panel) | |

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
<HelpScope context=[("domain", "bookkeeping")]>
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
rely on them:

| Key | Meaning | Example |
|---|---|---|
| `domain` | the app or subject area | `bookkeeping` |
| `screen` | the screen the user is on | `reconcile` |
| `section` | a part of that screen | `splits` |
| `entity` | the kind of thing in view | `transaction` |
| `<entity>_id` | its id, so a tool can fetch it | `transaction_id` |

Put `domain` in a scope at the app's root, and `screen`, `section` and ids
where they are known. Stable mappings can live in the app's `on_ask`
handler instead. A rule of thumb: the page declares what only the page
knows, such as the current entity's id.

Because the keys are free-form, an app (or the adapter that connects it to
its assistant) can add its own, such as routing hints for which tools an
assistant should try. Document those with the app or adapter, not here.
Treat any such key as a **hint**: what a model may do is decided by the
assistant's client, never by the page.

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
| `component` | what carries the `?`: `field`, `checkbox`, `switch`, `toolbar`, `dialog`, `alertdialog`, `sheet`, `panel`, or `help` for a bare `HelpButton` |
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
| `alert.css` | `.alert`, `.alert--{info,success,warning,error}`, `.alert__icon`, `.alert__body`, `.alert__close` |
| `help.css` | `.help-btn` (the `?`, with `.icon-btn`), `.help-popover`, `__title`, `__body`, `__paragraph`, `__list`, `__table` / `__heading` / `__cell`, `__actions`, `__link`, `__ask`; `[data-side]` below or above |

## CSS classes (overlay)

| Sheet | Classes |
|---|---|
| `modal.css` | `.dialog-open` on `<body>`: no page scroll while a modal is open |
| `dialog.css` | `.dialog`, `.dialog--wide`, `.dialog::backdrop`, `.dialog__title-bar`, `.dialog__title`, `.dialog__help`, `.dialog__close`, `.dialog__body`, `.dialog__footer` |
| `sheet.css` | `.sheet`, `.sheet--{left,right,bottom}`, `.sheet::backdrop`, `.sheet__title-bar`, `.sheet__title`, `.sheet__help`, `.sheet__close`, `.sheet__body` |

These are injected only when the `overlay` feature is on.

## CSS classes (forms)

| Sheet | Classes |
|---|---|
| `field.css` | `.field`, `.field__label`, `.field__label-row` (a label with its `?`), `.field__hint`, `.field__error` (also under a `Checkbox`) |
| `controls.css` | `.form-input`, `.form-textarea`, `.form-select`, `.is-invalid` on each (the danger border); `.select` (a select and its refresh button) |
| `password.css` | `.password-input`, `.password-input__toggle`; `.password-strength`, `.password-strength--{very-weak,weak,fair,strong}`, `.password-strength__bar`, `.password-strength__fill` |
| `choices.css` | `.checkbox`, `.checkbox.is-invalid`, `.checkbox__row` (the `<label>`), `.checkbox__input`, `.checkbox__label`, `.checkbox__line` (the row with its `?`); `.switch`, `[aria-checked='true']`, `.switch__track`, `.switch__knob`, `.switch-line` (the switch with its `?`) |
| `combobox.css` | `.combobox`, `.combobox__controls`, `.combobox__control`, `.combobox__popup`, `.combobox__list`, `.combobox__option`, `.combobox__option.is-active`, `.combobox__empty`, `.combobox__create` |

No rule touches a bare `input`, `select` or `label`: an app's own unclassed
controls are left alone. On a phone (≤ 768px) the controls' text is at least
16px, so iOS doesn't zoom in when one takes focus. These are injected only
when the `forms` feature is on.

## CSS classes (navigation)

| Sheet | Classes |
|---|---|
| `nav.css` | `.nav-list`, `.nav-list__items`; `.nav-link`, `.nav-link__icon`, `.nav-link[aria-current='page']`; `.nav-group`, `.nav-group--collapsible`, `.nav-group__heading`, `.nav-group__toggle` (`[aria-expanded='true']`), `.nav-group__label`, `.nav-group__chevron`, `.nav-group__list` |
| `side-menu.css` | `.side-menu` (with `.sheet`), `.side-menu__content`, `.side-menu__footer` |

These are injected only when the `navigation` feature is on.

## CSS classes (layout)

| Sheet | Classes |
|---|---|
| `panel.css` | `.panel`, `.panel--{compact,flush,warning,danger,collapsible}`, `.panel__header` (the band), `.panel__heading` (the title and its `?`), `.panel__title`, `.panel__actions`, `.panel__body`; collapsible: `.panel__toggle` (`[aria-expanded='true']`), `.panel__title-text`, `.panel__summary`, `.panel__chevron`, `.panel__body[hidden]` |
| `card.css` | `.card`, `.card--clickable`, `.card--compact`, `.card__header`, `.card__title`, `.card__link` (the title as a link or button, its `::after` the stretched target), `.card__actions`, `.card__body` |

These are injected only when the `layout` feature is on.

## CSS classes (data)

| Sheet | Classes |
|---|---|
| `data-view.css` | `.data-view`, `.data-view--{grid-sm,grid-md,grid-lg,list,row,wrap}`, `.data-view__list` (`[hidden]` while there are no items), `.data-view__item`, `.data-view__more` (the end marker), `.data-view__error`, `.data-view__empty` |
| `layout-switch.css` | `.data-layout-switch` |
| `pagination.css` | `.pagination`, `.pagination__list`, `.pagination__step` (Previous / Next), `.pagination__slot`, `.pagination__page` (`[aria-current='page']`), `.pagination__gap`, `.pagination__position` (the phone's `6 / 20`) |

These are injected only when the `data` feature is on.

## CSS classes (menus)

| Sheet | Classes |
|---|---|
| `menu.css` | `.menu` (the wrapper), `.menu__trigger` (`[aria-expanded='true']`), `.menu__chevron`, `.menu__trigger--custom`, `.menu__panel` (the popover; `data-side`), `.menu__header`, `.menu__items` (`role="menu"`), `.menu__item` (`--danger`, `--current`, `[aria-disabled='true']`), `.menu__icon`, `.menu__label`, `.menu__check`, `.menu__divider`, `.menu__trigger--chevron` |
| `split-button.css` | `.split-button`, `.split-button__main`, `.split-button__menu` |

`menu.css` also holds `.context-menu` (the area's wrapper) and reads
`--menu-max-width`.

These are injected only when the `menus` feature is on.

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
- `navigation`: leptos_router 0.8, with every component inside a
  `<Router>`.
- Help and `Combobox`: a browser with the Popover API (`popover`, `popovertarget`; Chrome
  114+, Firefox 125+, Safari 17+).
- `data`: `IntersectionObserver` for `DataView`'s infinite scroll (every
  current browser). Uses `js-sys`.

## Testing

- `cargo test -p ferrox-ui`: class lists, roles, and the class-only rule on
  every sheet.
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
