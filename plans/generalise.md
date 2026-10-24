# Generalise: take the user's stack out of the ported components

A short pass before the `data` group. The user's rule (2026-10-23): ferrox-ui
components stay general. Conventions of the user's own apps go in webapp-lib
adapters; ferrox-ui offers the general hook. Optional features are opt-in,
and their mechanism (what refreshing or creating *does*) is entirely the
app's: a static list, a REST endpoint, anything.

Status: approved (2026-10-23), the open point as recommended. Piece 1 done
(2026-10-23): `on_add`, `add_label`, `can_add` removed from `Panel`; the
demo puts its `+` in `actions` (behind its own `Show` for the permission).
Piece 2 done (2026-10-24): Combobox `on_create` / `create_label` (a
"Create “…”" button under the list, in a popover that now wraps the
listbox), Select `on_create` / `create_label` (a `+` after refresh). Piece 3
(docs) next.

## The audit

Every public component of `core`, `overlay`, `forms`, `navigation` and
`layout`, and ferrox-theme / ferrox-theme-build, was checked for features
that exist because of how the user's apps work.

| Found | Verdict | Here |
|---|---|---|
| `Panel`'s `on_add`, `add_label`, `can_add`: a `+` on every panel, from the user's list pages | the user's convention | piece 1: removed; `actions` is the general hook. webapp-lib adds a `ListPanel` (or its deprecated `ComponentTitleBar` wrapper) with the `+` |
| `Select` / `Combobox` `on_refresh` + `refreshing` | general, kept: opt-in, and ferrox-ui only calls the app's callback | piece 2 adds its sibling, new entry, on the same terms |
| ferrox-theme falls back to the `webapp-lib.theme` storage key (`persist.rs`, `prepaint.rs`) | a migration for the user's apps | deferred to webapp-lib piece 2: webapp-lib copies the old key to the new one at startup, then ferrox-theme drops the fallback (order: webapp-lib first, or saved choices are lost) |
| Help's recommended context keys `mcp_server` / `mcp_tool`, and the mapping to Patina's tags (README) | the user's assistant setup; the mechanism (free-form pairs) is general | piece 3: the README keeps the general keys (`domain`, `screen`, `section`, `entity`, `*_id`); the MCP and Patina conventions move to webapp-lib's assistant-adapter docs |
| The Classic theme's comments ("so apps moving off webapp-lib 0.x can keep their look"); `min_bits` described as "webapp-lib's `min_password_entropy`" | wording only | piece 3: reworded (an indigo theme; 80 bits as a plain default) |

Checked and general: Button, IconButton, Toolbar, Spinner, LoadingOverlay,
Alert, Dialog, ConfirmDialog, Sheet, Field, Input, Textarea, PasswordInput,
Checkbox, Switch, Combobox's core, the help mechanism, NavList, NavLink,
NavGroup, SideMenu, SideMenuButton, collapsible Panel, Card, and the `data`
plan.

## Design

### 1. Panel without the `+`

`on_add`, `add_label` and `can_add` go; `actions` stays. The same result,
written by the app:

```rust
<Panel title="Items" actions=move || view! {
    <Show when=move || can_add_items.get()>
        <IconButton title="Add item" on_click=move |_| new_item()><Icon icon=PLUS /></IconButton>
    </Show>
}>
```

Panel's demo, tests and README change with it; the band's buttons keep 44px
on a phone (that rule is on `.panel__actions .icon-btn`, not on the `+`).

### 2. New entry on Select and Combobox (opt-in)

**Combobox**, ported from conrad's `Autocomplete` `onCreateNew`:

```rust
    /// Offers to create an item from the typed text when nothing matches.
    #[prop(optional, into)] on_create: Option<UnsyncCallback<String>>,
    /// The offer's text, given the typed text; `Create “{text}”` by default.
    #[prop(optional, into)] create_label: Option<Callback<String, String>>,
```

- While the typed text (trimmed, not empty) matches no item, the list shows
  a **"Create “milk”" button** under "No matches". Enter in the field (with
  no active option) presses it, as does a click. It's a button, not an
  option: creating is a different act from choosing.
- `on_create` gets the trimmed text. What it does is the app's: add it to a
  static list, `POST` it, open a dialog to fill in more. Typically the app
  then adds the new item to `items` and sets `text` to its label.
- Without `on_create`, nothing changes.

**Select**:

```rust
    /// Adds a "new" button beside the select.
    #[prop(optional, into)] on_create: Option<UnsyncCallback<()>>,
    #[prop(into, default = "New…".to_string())] create_label: String,
```

- An `IconButton` drawing `PLUS` after the select (and after refresh, when
  both are on), labelled `create_label`. The app decides what "new" means;
  usually a dialog, after which it adds the option and sets `value`.

Both are e2e-tested in the demo with a static list in the page: nothing in
ferrox-ui fetches.

### 3. Docs

- Help: the recommended keys table keeps `domain`, `screen`, `section`,
  `entity`, `<entity>_id`; the `mcp_server` / `mcp_tool` rows and the Patina
  mapping move out. A sentence says the keys are free-form, so an app's own
  assistant conventions (such as MCP routing hints) go in its own docs.
  (webapp-lib's adapter docs get them in its piece 3.)
- Classic theme comments, `min_bits`'s description: reworded.
- `plans/forms.md`: create-new moves from "later" to done.

## Pieces

1. **Panel without the `+`.** e2e: the Items panel's `+` comes from its
   actions behind the switch; the rest of the panel tests unchanged.
2. **New entry.** e2e (Combobox): the offer appears only when the text
   matches nothing, with the text in its label; click and Enter call
   `on_create` once with the trimmed text; no offer without `on_create`;
   the demo adds the item, and it's then chosen. (Select): the `+` button
   after the select (and after refresh), calls `on_create` once.
3. **Docs** as above, and this plan's status.

## Open points

1. **Combobox's offer is a button under the list** (conrad's), recommended:
   a screen reader hears it as an action, not as one of the items. The
   alternative is a last `option` in the listbox ("+ Create “milk”"),
   reachable with the arrow keys.
