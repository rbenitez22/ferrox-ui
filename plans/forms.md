# forms: Field, Input, PasswordInput, Textarea, Checkbox, Select, Combobox

The `forms` step of piece 6 in `plans/ferrox-ui.md`: Field, Input with
password strength, Checkbox, Select, Combobox. Ports the behaviour and CSS of
conrad's `FormField.tsx`, `Autocomplete.tsx` and its `.form-*` rules, and
replaces webapp-lib's form primitives. The data-loading halves stay in
webapp-lib as adapters.

Status: approved (2026-10-23). Pieces 1–3 done (2026-10-23); piece 4 next.

Notes from piece 1:
- `Field` hands its wiring down through `<Provider>`, not `provide_context`.
  A Leptos 0.8 component has no reactive owner of its own, so a plain
  `provide_context` leaked to the controls *after* the field, and an input
  outside any field took the previous field's id. The e2e unique-id test
  caught it.
- `unique_id` moved from the overlay modal module to `src/ids.rs`, shared by
  `overlay` and `forms`.
- Controls show the theme's focus ring (`:focus-visible` on the focus-ring
  tokens), as buttons do.
- The release check now runs `--feature-powerset` (8 combinations for
  ferrox-ui).
- Piece 2: the show/hide button keeps one label ("Show password") and says
  whether it's on through `aria-pressed`, the ARIA way for a toggle. The
  strength code is split: `password_entropy` (moved from webapp-lib,
  unchanged), `strength_level` (the cut-offs: 25, 40, `min_bits`), and
  `password_strength`. `min_bits` defaults to 80, webapp-lib's
  `min_password_entropy`. The meter's word comes from `StrengthLabels`
  (English by default), and the bits are on `data-bits`, not in the visible
  text. The field is described by its hint and the meter together.
- Piece 3: Checkbox carries its own `hint` / `error` (as a Field does),
  reusing `.field__hint` / `.field__error`. Select marks the chosen option
  with `prop:selected` per option rather than `prop:value` on the select, so
  the choice holds whatever order the options and the value update in. The
  options are a keyed `<For>`. While refreshing, an inline `Spinner` takes
  the refresh icon's place inside the button, which stays put and is
  disabled. Select's texts (placeholder "Select…", "Loading…", "Refresh",
  "Refreshing…") are props.

## Sources

| Today | Where | Becomes |
|---|---|---|
| conrad `FormField` (label, one message below: hint or error, `aria-describedby`, `aria-invalid`, `.is-invalid`) | `ui/src/components/FormField.tsx`, `theme/components.css:787-930` | `Field` |
| bare `<input>`s with hard-coded ids (`password`, `email`, `name`, …: review H12) | webapp-lib `components/mod.rs`, `auth.rs` | `Input`, `Textarea` |
| `VerifiedPassword` + `EntropyIndicator` + `calc_password_entropy`. The toggles are `<span>`s with glyph classes (H13), the indicator uses inline colour styles (S2), and the markup is repeated 3× (Q6). | `mod.rs:491-640` | `PasswordInput` (show/hide button, optional strength meter) + `password_strength()`. The new-password + confirm pair stays in webapp-lib, built from two `PasswordInput`s |
| `CheckBox { id, value, on_change, style }` | `mod.rs:305` | `Checkbox` |
| `render_options` / `SelectMenu`'s `<select>`: rebuilt on every change (Q9), `<span>` refresh (H13), inline styles | `mod.rs:121, 745` | `Select`. `SelectMenu` (loading from the API) stays in webapp-lib as an adapter over it |
| `AutocompleteInput<T>`: no combobox semantics (H13), inline styles | `autocomplete.rs` | `Combobox<T>`. `AutocompleteInputModel` (fetching) stays in webapp-lib |
| conrad `Autocomplete` (574 lines: groups, create-new, clear, chevron, refresh, multi) | `Autocomplete.tsx`, `components.css:1791-1965` | `Combobox`: the core now, the rest later (open point 4) |

Review items covered: H12 (ids), H13 (real buttons, combobox ARIA), Q6 (one
password control), Q9 (`Select` renders once), S2 (no inline styles), X3, X4,
H7.

## Design

### Field: label, control, one message

conrad's rule: **a problem one field can fix goes next to that field.**
Anything cross-field goes in an `Alert` at the top of the form, and never
both. A hint and an error are never shown together; once a field is wrong,
the message under it says what's wrong.

```rust
#[component]
pub fn Field(
    #[prop(into)] label: Signal<String>,
    /// Guidance while the field is fine. Hidden while there is an error.
    #[prop(optional, into)] hint: MaybeProp<String>,
    /// The field's problem. Present means the control is invalid.
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView
```

```rust
<Field label="Name" hint="As it appears on statements" error=name_error>
    <Input value=name autofocus=true />
</Field>
```

**Wiring through context, not ids at the call site.** `Field` provides a
`FieldContext` newtype (H11) holding a unique id (H12), the message id and
the invalid signal. Every control in this group reads it:

- `id` = the field's id, so the `<label for>` works;
- `aria-describedby` = the message;
- `aria-invalid` and `.is-invalid` while there's an error.

A control used outside a `Field` generates its own id. This is the Leptos
form of conrad's render prop, and call sites don't change shape.

### The controls

Values are **two-way `RwSignal`s** (open point 1): `value: RwSignal<String>`,
`checked: RwSignal<bool>`. That's what every form in webapp-lib does today.

| Component | Props (besides `class`, `disabled`, `name`) | Notes |
|---|---|---|
| `Input` | `value`, `kind: InputKind` (Text, Email, Search, Tel, Url, Number), `placeholder`, `required`, `readonly`, `autofocus`, `autocomplete` | `.form-input` |
| `Textarea` | `value`, `rows`, `placeholder`, `required`, `autofocus` | `.form-textarea`; Enter is a newline (Dialog already respects that) |
| `PasswordInput` | `value`, `autocomplete` (`current-password` / `new-password`), `strength: bool`, `strength_labels`, `show_label` / `hide_label` | a real `<button>` toggles show/hide (`EYE` / `EYE_OFF`, `aria-pressed`). With `strength`, a meter and its label below, from `password_strength()`; colours come from classes, not inline styles |
| `Checkbox` | `checked`, `label` (required: the box and its text are one `<label>`), `hint`, `error` | the native checkbox, styled with `accent-color` and focus ring; not inside a `Field`, since it carries its own label |
| `Select` | `value`, `options: Signal<Vec<SelectOption>>` (`value`, `label`), `placeholder` (a hidden, disabled first option), `loading` (shows "Loading…", disabled), `on_refresh` + `refreshing` (a refresh `IconButton`, a spinner while refreshing) | the `<select>` renders once; only the options and `prop:value` react (Q9) |
| `Combobox<T>` | below | |

`password_strength(&str) -> PasswordStrength` (`VeryWeak`, `Weak`, `Fair`,
`Strong`, plus the bits) is the entropy calculation moved from webapp-lib.
The cut-off for `Strong` is a prop (`min_bits`), so webapp-lib's
`min_entropy` keeps working.

### Combobox: a text field that filters a list

The ARIA combobox pattern (H13):

- `<input role="combobox" aria-expanded aria-controls aria-activedescendant
  aria-autocomplete="list">` and a `role="listbox"` of `role="option"`s.

```rust
#[component]
pub fn Combobox<T>(
    /// The items to choose from (whatever loaded them owns them).
    #[prop(into)] items: Signal<Vec<T>>,
    /// Stable identity per item: keys and the active option's id.
    key_of: impl Fn(&T) -> String + Clone + Send + Sync + 'static,
    /// The text shown, and the text matched against.
    label_of: impl Fn(&T) -> String + Clone + Send + Sync + 'static,
    /// The text in the field.
    text: RwSignal<String>,
    /// An item was chosen, or the field was left matching nothing (None).
    #[prop(into)] on_select: UnsyncCallback<Option<T>>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(default = true)] clearable: bool,      // × clears the choice
    #[prop(default = true)] toggleable: bool,     // chevron shows the whole list
    #[prop(optional, into)] on_refresh: Option<UnsyncCallback<()>>,
    #[prop(optional, into)] refreshing: Signal<bool>,
    #[prop(into, default = "No matches".into())] empty_label: Signal<String>,
    // … class, disabled, required, autofocus
) -> impl IntoView
```

How it behaves:

- **Filtering:** case-insensitive "contains" on `label_of`.
- **Keyboard:**
  - ↓ / ↑ move the active option (opening the list if it's closed).
  - Enter chooses the active option.
  - Esc closes the list, or clears the text if the list is already closed.
  - Tab chooses the active option and moves on.
- **Mouse:** a click chooses. Leaving the field with text that matches no
  item calls `on_select(None)`.
- **Its controls are real buttons, out of the Tab order** (as in conrad):
  clear, the chevron, and refresh. The refresh button shows a spinner while
  refreshing.
- **The list escapes clipping** (open point 3). It's a `popover="manual"`
  element in the top layer, placed under the field with `position: fixed`
  from the field's box, and it's re-placed on scroll and resize. A list
  inside a `Dialog` body (`overflow: auto`) isn't cut off, and it sits above
  the modal.

### CSS: `forms/field.css`, `forms/controls.css`, `forms/combobox.css`

- Ported from conrad's `.form-*` and `.autocomplete*` rules, renamed:
  `.form-group` → `.field`, `.autocomplete` → `.combobox`.
- Class-only and tokens only.
- No rule touches a bare `input` or `label`: an app's own unclassed inputs
  are left alone.
- Hover, focus and `.is-invalid` states.
- Phone: 16px font in inputs, so iOS doesn't zoom in.

### The `forms` feature, the icons, and the release check

- `forms = []` in `ferrox-ui/Cargo.toml`.
- New icons from Lucide (pinned commit): `EYE`, `EYE_OFF`.
- ferrox-ui now has two group features, so the release check moves from
  `--each-feature` to `--feature-powerset`. That's planned since piece 4.

## Pieces

Each one passes `scripts/release-check.sh` and can be reviewed on its own.
The demo gets a `/forms` page, built up piece by piece.

1. **Feature, Field, Input, Textarea.** `forms` feature, `FieldContext`,
   `field.css` + `controls.css`, `--feature-powerset`. e2e:
   - the label names the control;
   - the hint or the error is the description, never both;
   - error → `aria-invalid` + `.is-invalid` + the danger border;
   - two Fields on a page have different ids;
   - an Input outside a Field still gets an id;
   - two-way value;
   - phone font size.
2. **PasswordInput.** `EYE` / `EYE_OFF` icons, `password_strength()` with
   unit tests (moved from webapp-lib, including its non-ASCII case), the
   meter. e2e:
   - the toggle is a button with `aria-pressed` that flips the `type`;
   - the meter's level and label per strength;
   - the meter is hidden while empty;
   - `autocomplete` passes through.
3. **Checkbox, Switch and Select.** Switch is conrad's `Toggle`: a
   `<button role="switch" aria-checked>` with the label inside it. e2e (Switch:
   role, `aria-checked` flips on click and Space, label is its name):
   - the label toggles the checkbox, which is keyboard operable;
   - Select's two-way value;
   - the placeholder can't be chosen;
   - loading disables it and says so;
   - refresh is a button that shows a spinner;
   - focus survives an options change (Q9).
4. **Combobox.** e2e:
   - the ARIA attributes at each step;
   - filtering;
   - the keyboard (↓ ↑ Enter Esc Tab);
   - click to choose;
   - leaving the field matching nothing → `None`;
   - clear and chevron;
   - refresh spinner;
   - the list isn't clipped inside a `Dialog`, and follows the field when the
     page scrolls.
5. **Docs.** The README forms section (the placement rule, the context
   wiring, each control with an example, the class reference). Update the
   root README, the demo README and the plan status.

Not here:
- `Switch` (conrad's `Toggle`, open point 2).
- NumberInput, DateInput, Radio, MultiSelect, Slider, OTP, Phone, Chips,
  Dropzone (inventory, built when an app needs them).
- `TypeToConfirmDialog` (overlay, after this group).
- webapp-lib's adapters and its password pair (its piece 3).

## Decisions (2026-10-23)

All as recommended:

1. Values are two-way `RwSignal`s.
2. `Switch` is added in piece 3.
3. The Combobox list goes in the top layer (`popover` + fixed positioning).
4. Combobox core now; groups, create-new, multi-select and search-start come
   later.
5. The new-password + confirm pair stays in webapp-lib.
6. New names: `.field`, `.form-input`, `.combobox`.
