# overlay: Dialog, ConfirmDialog, Sheet

The `overlay` step of piece 6 in `plans/ferrox-ui.md`: Dialog with conrad's
footer band, Alert dialog, Sheet. Ports conrad's `Dialog.tsx` /
`ConfirmDialog.tsx` behaviour and CSS, and replaces webapp-lib's dialog
helpers and the drawer half of `SideMenu`.

Status: approved (2026-10-23). Done (2026-10-23): pieces 1–4.

Notes from piece 1:
- Callbacks with no input are zero-argument closures:
  `on_close=move || open.set(false)`, `DialogAction::new("Save", move || save())`.
  (`UnsyncCallback<()>` converts from `Fn()`.)
- `close_label` is a `String`, not a signal, because `IconButton`'s `title` is
  a `String`.
- Enter also leaves a focused link or button alone, so Enter on "Save as
  draft" presses that button rather than submitting.
- A `<dialog>` stays where it's declared in the DOM, so an app's descendant
  selectors (`.section h2`) reach into it. The demo's `.demo-section h2`
  pulled the title's margin and pushed the × against the title; the demo rule
  is now `.demo-section > h2`, and a test checks the title bar layout. Worth
  saying in the README: scope app rules to direct children near dialogs.
- Piece 2: `Dialog` gained a public `alert` prop (`role="alertdialog"`), which
  `ConfirmDialog` sets. An app may want a non-destructive alert dialog too.
  `ConfirmDialog` also takes `close_label`. Focus opens on the ×, never on
  Delete, so a reflexive Enter closes rather than deletes.
- Piece 3: a backdrop click closes a Sheet only if the press also started on
  the backdrop, so a text selection dragged out of the sheet doesn't close
  it. The slide-in is CSS only (`@starting-style`, `ease-out` over
  `--react-time`, not the overshooting `--react-ease`); closing has no
  animation. The left and right sheets are pinned with `inset`, the bottom
  one has rounded top corners.

## Sources

| Today | Where | Becomes |
|---|---|---|
| webapp-lib `open_dialog` / `close_dialog(NodeRef<Dialog>)`, `DialogTitle` (glyph close button) | `lib/src/components/mod.rs:88-175` | `Dialog`, driven by an `open` signal |
| webapp-lib `DeleteConfirmDialog` (hard-coded "Confirm Delete", inline styles) | `mod.rs:359` | `ConfirmDialog` |
| webapp-lib `ConfirmDialog` (`open=is_visible.get()` read once, non-modal: review C6) | `mod.rs:390` | `ConfirmDialog` |
| webapp-lib `SideMenu` drawer (`.side-menu`, `.side-menu-overlay`, open flag in context) | `mod.rs:426` | `Sheet`. `SideMenu` itself is rebuilt on `Sheet` in `navigation` |
| conrad `Dialog` (title bar, body, footer band; focus in, trap, return; `inert`; scroll lock; Esc; Enter submits) | `ui/src/components/Dialog.tsx`, `theme/components.css:1575-1780` | `Dialog` |
| conrad `ConfirmDialog` (danger affirm, the verb as label) | `ConfirmDialog.tsx` | `ConfirmDialog` |

Review items covered: C6 (confirm dialog state), X3 (`class`, no `style`), X4
(every text a prop with an English default), H7 (`#[non_exhaustive]`).

## Design

### Built on the native `<dialog>` with `showModal()`

conrad builds its dialog by hand, because jsdom has no `showModal`. We test
in real browsers (Playwright), so that reason doesn't apply here. `showModal`
gives us these for free and correctly:

- The top layer: no `z-index` battles, and it sits above `LoadingOverlay`'s
  9999.
- The rest of the page is inert: out of the tab order and the accessibility
  tree.
- Focus stays inside the dialog.
- `::backdrop` for the scrim.
- Esc closes it.

A shared internal module (`overlay/modal.rs`) drives one `<dialog>` from an
`open: Signal<bool>`. Dialog, ConfirmDialog and Sheet all use it.

- **The signal is the source of truth.** When `open` becomes true the module
  calls `show_modal()`, and when it becomes false it calls `close()`.
- **Every way of closing goes through `on_close`.** Esc fires `cancel`; the
  module prevents the default and calls `on_close`. A `close` event while
  `open` is still true calls `on_close` too. That covers Chrome closing anyway
  on a repeated Esc. So the app's signal and the element never disagree. This
  is what C6 got wrong.
- **Focus on open:** an element marked `autofocus` (a form's first field),
  otherwise the first control, which is the close button. On close, focus
  returns to the element that opened the dialog. The browsers do this for
  `showModal`; the module checks it and does it itself if the browser didn't.
- **Scroll lock:** while any modal is open, `<body>` carries `.dialog-open`
  (`overflow: hidden`). A count handles nested modals, such as a confirm
  opened from a dialog.
- **Content is mounted only while open** (`ChildrenFn`), as in conrad. Each
  opening starts fresh: no stale form state, and `autofocus` works every time.

### Dialog

conrad's rule: **the footer is not a slot.** Callers pass a dismiss label and
the actions, and the component fixes the order: dismissive first, then the
alternate, then the affirmative action last and heaviest. Every time.

```rust
#[component]
pub fn Dialog(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] title: Signal<String>,
    /// Esc, the × and the dismiss button: all three are the same act.
    #[prop(into)] on_close: UnsyncCallback<()>,
    /// The dismissive button's label (default "Cancel").
    #[prop(into, default = "Cancel".into())] dismiss_label: Signal<String>,
    /// No dismiss button (the × and Esc still close).
    #[prop(optional)] no_dismiss: bool,
    /// The affirmative action: last, primary or danger.
    #[prop(optional)] affirm: Option<DialogAction>,
    /// A second, lighter way to finish ("Save as draft" beside "Post"):
    /// secondary, between dismiss and affirm. Enter never reaches it.
    #[prop(optional)] alternate: Option<DialogAction>,
    /// Enter fires `affirm` (not in a textarea, not while saving or disabled).
    #[prop(default = true)] submit_on_enter: bool,
    #[prop(optional)] size: DialogSize,            // Md (32rem) | Wide (52rem)
    #[prop(into, default = "Close".into())] close_label: Signal<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView

/// One footer button. `#[non_exhaustive]`; built with `DialogAction::new`.
pub struct DialogAction { label, on_click, family, saving, disabled }
impl DialogAction {
    pub fn new(label: impl Into<Signal<String>>, on_click: impl Into<UnsyncCallback<()>>) -> Self;
    pub fn danger(self) -> Self;                         // affirm only: a destructive commit
    pub fn saving(self, saving: impl Into<Signal<bool>>) -> Self;
    pub fn disabled(self, disabled: impl Into<Signal<bool>>) -> Self;
}
```

Markup: `<dialog class="dialog" aria-labelledby=…>` containing:

- `.dialog__title-bar`: an `<h2 class="dialog__title">` and an `IconButton`
  with `CLOSE`.
- `.dialog__body`
- `.dialog__footer`: the **footer band**, a padded strip on `--surface-2`
  with a hairline above it, as you asked for (2026-10-23).

The title id comes from a per-instance counter, so two dialogs never share
an id (H12).

### ConfirmDialog (the alert dialog)

conrad's standard destructive confirmation. Every destructive action opens
one, with no "undo toast" substitute. It's a `Dialog` with:

- `role="alertdialog"`.
- The affirm button always in the danger family.
- `confirm_label` required: the verb, not "OK" (Delete, Void, Archive).
- The body names exactly what is about to be lost.

```rust
#[component]
pub fn ConfirmDialog(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] confirm_label: Signal<String>,
    #[prop(into)] on_confirm: UnsyncCallback<()>,
    #[prop(into)] on_cancel: UnsyncCallback<()>,
    #[prop(optional, into)] saving: Signal<bool>,
    #[prop(into, default = "Cancel".into())] cancel_label: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView
```

The caller closes it after `on_confirm`, usually once the request succeeds,
with `saving` true meanwhile. That's what replaces `DeleteConfirmDialog` and
the old `ConfirmDialog`.

### Sheet

A modal panel that slides in from an edge, using the same modal module.
webapp-lib's side menu is its first user (`navigation` will build `SideMenu`
on it).

```rust
#[component]
pub fn Sheet(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_close: UnsyncCallback<()>,
    #[prop(optional)] side: SheetSide,             // Left (default) | Right | Bottom
    /// Shown in the sheet's title bar; also its accessible name.
    #[prop(into)] title: Signal<String>,
    #[prop(into, default = "Close".into())] close_label: Signal<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView
```

- **A click on the backdrop closes a Sheet** (open point 2), as it closes
  today's side menu. A Dialog doesn't close that way, because a stray click
  must not throw away typed input.
- It slides in over `--react-time`. Under reduced motion it appears without
  sliding, and closing never waits on an animation.
- Left and Right are `min(20rem, 85vw)` wide and full height. Bottom is full
  width, at most 85vh tall.

### CSS: `overlay/dialog.css`, `overlay/sheet.css`

These are ported from conrad. They're class-only (`.dialog::backdrop` starts
with a class) and use tokens only. They're injected by `init()` only when the
`overlay` feature is on: `css::sheets()` gains a `cfg` block per group.

- **Phone widths** (≤ 768px, conrad's breakpoint): the dialog is full width
  inside a small margin, the close button is 44px, and long strings wrap.
- **Not ported yet:** conrad's `sheetOnMobile` and `fillScreen` (open point 5).

### The `overlay` feature

- `ferrox-ui/Cargo.toml`: `overlay = []`.
- The release check already builds each feature alone (`--each-feature`).
  This is ferrox-ui's first group feature, so `--feature-powerset` is still
  not needed until a second group lands.
- The demo enables `overlay`.

## Pieces

Each one passes `scripts/release-check.sh` and can be reviewed on its own.

1. **Feature, modal module and Dialog.** `overlay` feature, `modal.rs`,
   `Dialog`, `DialogAction`, `DialogSize`, `dialog.css`. Demo page `/overlay`
   with:
   - a form dialog (an `autofocus` field, Save + Save as draft, `saving`);
   - a wide dialog;
   - a dialog with no dismiss button;
   - a dialog opened from inside a dialog.

   e2e:
   - opens modal (the page behind is inert);
   - focus goes to `autofocus` or the close button, Tab stays inside, and
     focus returns to the trigger on close;
   - Esc, the × and dismiss each call `on_close` once, and the signal and the
     element agree;
   - Enter submits, but not from a textarea and not while saving;
   - footer order;
   - page scroll locked while open, also for nested dialogs;
   - the title labels the dialog;
   - phone width.
2. **ConfirmDialog.** On `/overlay`: "Delete account…" with a saving step.
   e2e:
   - `role="alertdialog"` and a danger confirm button with the verb label;
   - Cancel and Esc call `on_cancel`;
   - confirm runs once, and the button refuses clicks while saving;
   - reopening starts fresh.
3. **Sheet.** On `/overlay`: left, right and bottom sheets. e2e:
   - it sits at its edge with the right size;
   - the backdrop click, Esc and the × close it;
   - focus, inertness and scroll lock as for Dialog;
   - reduced motion means no slide.
4. **Docs.** `ferrox-ui/README.md` gets an overlay section (which modal to
   use, the footer rule, each component with an example, the class reference).
   Update the root README, the demo README and the plan status.

Not here: Toast, Popover, Tooltip, Hover card (overlay inventory, built when
an app needs them). TypeToConfirmDialog (open point 4). webapp-lib's switch
to these components is its own piece 3.

## Decisions (2026-10-23)

All as recommended:

1. Native `<dialog>` + `showModal`.
2. A backdrop click closes a Sheet, not a Dialog.
3. Content is mounted only while open.
4. TypeToConfirmDialog comes after `forms`.
5. Mobile variants (`sheetOnMobile`, `fillScreen`) come later, when an app
   needs them.
6. One `/overlay` demo page.
