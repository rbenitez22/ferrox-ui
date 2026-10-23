# core, part 2: Spinner, LoadingOverlay, Alert

Piece 6a of `plans/ferrox-ui.md` ("`core` rest: `Spinner`, `Alert`"). Ports
the behaviour and CSS of conrad's `Spinner.tsx` / `Alert.tsx` and replaces
webapp-lib's `Spinner` and `Message`.

Status: approved (2026-10-23). Pieces 1–3 done (2026-10-23); piece 4 (docs) next.

Notes from piece 1:
- `LoadingOverlay` without `text` gives screen readers a hidden "Loading…",
  so the status is never silent.
- A region spinner's wrapper (`.spinner-status--region`) centres it with
  `--space-2xl` above and below. The ring itself has no margin.
- Piece 2: the severity icons are named for what they mean (`INFO`,
  `SUCCESS`, `WARNING`, `ERROR`), following ferrox-icons' "our own names"
  rule, not Lucide's names (`info`, `circle-check`, `triangle-alert`,
  `circle-alert`) as written above.
- Piece 3: `Alert` gained `dismiss_label` (default "Dismiss") for the close
  button, per review item X4 (text props). The icon and the close button are
  centred on the first line with `1lh`.
- Seen in piece 3, not fixed here: Patina light's `--success-text` is low
  contrast on `--success-tint`. That's a theme token question for
  ferrox-theme, not for Alert.

## Sources

| Today | Where | Becomes |
|---|---|---|
| webapp-lib `Spinner { message }`: a full-page `.spinner-overlay` | `lib/src/components/mod.rs:233`, `lib/style/loading.css` | `LoadingOverlay` |
| webapp-lib `.delete-spinner`, `.input-refresh-spinning`, `.item-status-saving` (CSS classes, `!important`, glyph overrides) | `base.css:344`, `loading.css:64-97` | `Spinner` with `SpinnerKind::Inline` / `Field` |
| webapp-lib `Message { RwSignal<MessageModel> }`, glyph icon classes | `mod.rs:336` | `Alert`. `Message`/`MessageModel` stay in webapp-lib as a thin adapter (its piece 3) |
| conrad `RegionSpinner` / `InlineSpinner` / `FieldSpinner` / `LoadingOverlay` | `ui/src/components/Spinner.tsx`, `theme/components.css:1228-1320` | `Spinner` + `LoadingOverlay` |
| conrad `Alert { severity }` | `Alert.tsx`, `components.css:990-1030` | `Alert` |
| conrad `.sr-only` | `theme/utilities.css:129` | `.sr-only` in core |

## Design

### Spinner: conrad's rules, kept

Choose a spinner by **what stays usable while the work runs**, not by how
long the work takes:

| Pattern | Use it when | Component |
|---|---|---|
| overlay | nothing on screen is valid any more: boot, resolving auth, the post-login redirect. Never a route change or a data fetch. | `LoadingOverlay` |
| saving button | any committing action | `Button saving=…` (exists) |
| region | a pane fetching data whose shape it can't promise | `Spinner` (`Region`, the default) |
| inline | an icon-only row action; the spinner takes the icon's place | `Spinner kind=Inline` |
| field | async work that belongs to one field (lookup, typeahead) | `Spinner kind=Field` |

There's one indicator per action: a saving button never also raises an
overlay.

**Armed on a delay.** Every spinner starts at `opacity: 0`. A second animation
reveals it after `--loading-delay` (400ms, already in ferrox-theme's tokens),
so work that finishes faster never shows a spinner at all.

**Reduced motion** is handled surgically, as in conrad. The spin slows to
2.4s but doesn't stop, because a still spinner reads as "hung". The reveal
animation stays, because it's what makes the spinner visible.

```rust
#[component]
pub fn Spinner(
    /// What is loading, for screen readers ("Loading accounts"). Required: a
    /// spinner with no label is an unexplained `role="status"`.
    #[prop(into)] label: Signal<String>,
    #[prop(optional)] kind: SpinnerKind,          // Region (default) | Inline | Field
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
// <span role="status" class="spinner-status {class}">
//   <span class="spinner spinner--region"></span><span class="sr-only">{label}</span>
// </span>

#[component]
pub fn LoadingOverlay(
    /// Visible text under the spinner; also the status text.
    #[prop(optional, into)] text: MaybeProp<String>,
) -> impl IntoView
// <div class="spinner-overlay" role="status" aria-live="polite">
//   <span class="spinner"></span><span class="spinner-overlay__text">{text}</span>
// </div>
```

`SpinnerKind` is `#[non_exhaustive]`, like the button enums (H7). Text props
take `Signal<String>`, per review item X4.

### Alert

An in-page or in-form message that **never times out**: the reason a submit
failed, a server refusal that no single field owns. Messages that dismiss
themselves are Toasts, which come later in `overlay`. Anything the user must
act on is never a toast.

```rust
#[component]
pub fn Alert(
    severity: AlertSeverity,                       // Info | Success | Warning | Error
    /// Draw the severity's icon before the text (default true).
    #[prop(default = true)] icon: bool,
    /// Shows a close button that calls this; without it, no button.
    #[prop(optional, into)] on_dismiss: Option<UnsyncCallback<()>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView
// <div class="alert alert--error" role="alert">
//   <Icon icon=CIRCLE_ALERT class="alert__icon"/>
//   <div class="alert__body">{children}</div>
//   <IconButton title="Dismiss" class="alert__dismiss" …><Icon icon=CLOSE/></IconButton>
// </div>
```

- **Roles:** `role="alert"` for `Error` (announced at once), `role="status"`
  for the rest. That's conrad's rule.
- **Status roles only:** there's no `primary` alert, because an alert reports
  a state, it doesn't act. `warning` is the non-blocking tier; if something
  should block a submit, it's an `error`.
- **Icons by default** (open point 1). conrad marks severity with colour only
  (a 3px left border plus a tint). An icon per severity makes the severity
  readable without colour (WCAG 1.4.1). The icons are `INFO` (Lucide `info`),
  `CIRCLE_CHECK` (`circle-check`), `TRIANGLE_ALERT` (`triangle-alert`) and
  `CIRCLE_ALERT` (`circle-alert`), all new in ferrox-icons.
- **ferrox-ui starts depending on ferrox-icons here**, for the severity icons
  and the dismiss `CLOSE`. This is the dependency the ferrox-icons plan
  deferred to piece 6.

### CSS (new core sheets, class-only, tokens only)

- `core/a11y.css`: `.sr-only`.
- `core/spinner.css`: `.spinner`, `--region` / `--inline` / `--field`,
  `.spinner-status`, `.spinner-overlay`, `__text`, the keyframes
  `ferrox-spin` and `ferrox-spinner-appear` (prefixed like
  `ferrox-border-sweep`), and the reduced-motion block.
- `core/alert.css`: `.alert`, one modifier per severity (tint, left border,
  text colour from `--{info,success,warning,danger}-*`), `__icon`, `__body`,
  `__dismiss`.

There are no `!important` declarations and no glyph `::before` overrides.
Those were webapp-lib workarounds for spinners drawn over glyph classes.

## Pieces

Each one passes `scripts/release-check.sh` and can be reviewed on its own.

1. **Spinner + LoadingOverlay.** `a11y.css`, `spinner.css`, both components,
   `SpinnerKind`. Demo page `/feedback` with a "Spinners" section (each kind,
   plus a button that shows the overlay for 2s). e2e:
   - `role="status"` with the label.
   - Invisible at first, visible after `--loading-delay`.
   - Each kind's size.
   - Under `reducedMotion: 'reduce'`: still revealed, and the spin is 2.4s.
   - The overlay covers the page and hides again.
2. **Icons for Alert.** Add `info`, `circle-check`, `triangle-alert`,
   `circle-alert` to ferrox-icons at the pinned Lucide commit, with the README
   table. ferrox-ui takes the ferrox-icons dependency.
3. **Alert.** `alert.css`, the component, `AlertSeverity`. A "Alerts" section
   on `/feedback`: each severity, one without an icon, one dismissible. e2e:
   - The role per severity.
   - The icon per severity, and none with `icon=false`.
   - Dismiss calls back and the demo removes the alert.
   - The text and border colour follow the theme's severity tokens.
4. **Docs.** Create `ferrox-ui/README.md`, which doesn't exist yet: purpose,
   `init()`, every public item with an example, the spinner choice table,
   class reference. Update the root README and the `plans/ferrox-ui.md` status.

Not here: Skeleton (conrad has it; it's in the inventory, not "now"), Toast
(`overlay`), and webapp-lib's `Message` adapter (webapp-lib piece 3).

## Decisions (2026-10-23)

1. **Alert icons:** a default icon per severity; `icon=false` turns it off.
2. **Dismissible alerts:** `on_dismiss` is included now.
3. **Demo page:** one `/feedback` page (spinners and alerts; later Skeleton
   and Toast).
