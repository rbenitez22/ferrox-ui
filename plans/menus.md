# menus: Menu, its items, SplitButton, ContextMenu

The `menus` step of piece 6 in `plans/ferrox-ui.md`, scheduled after `data`
(2026-10-23): row actions, a user menu and an overflow "More" all need one.
A list of actions that hangs off the control that offered it, with ARIA's
menu-button keyboard contract.

Status: approved (2026-10-24), all open points as recommended, with
`ContextMenu` added as piece 4 at the user's request.

Notes from piece 1 (done 2026-10-24):
- The items get the menu's context through a `Provider` inside the lazy
  `Show`: `provide_context` in the component leaked between sibling menus
  (focus went back to the wrong trigger).
- The panel is `popover="auto"` (outside press and the browser's Esc for
  free) with `popovertarget` on the trigger; Esc is handled on the panel
  first, so focus goes back and a `Dialog` stays open. Focus leaving the
  panel (Tab) closes it.
- The first opening builds the items and focuses one a frame later; later
  openings focus at once.
- `core::anchor` gained `Align::{Start, End}`.
- e2e: 26 menu tests; full suite 512.

Notes from piece 2 (done 2026-10-24):
- A disabled item ignores the click and keeps the menu open. Playwright
  treats `aria-disabled` as disabled, so its test clicks with `force`.
- `MenuLink` closes the menu on click and lets the click reach the router;
  Space is turned into a click.
- `current` draws `CHECK` at the item's end; `menu_item_classes` is public.
- e2e: 40 menu tests.

Notes from piece 3 (done 2026-10-24):
- The chevron is a public `MenuTrigger::Chevron { label, family }` (a
  `.btn` showing only a chevron), so `SplitButton` is a `Button` and a
  `Menu` side by side, end-aligned.
- Saving disables the chevron too; disabled reaches both halves.
- Joined halves drop the `.btn` hover grow (one would ride over the other).
- e2e: 54 menu tests.

Notes from piece 4 (done 2026-10-24):
- The area is a `display: contents` wrapper; the items are a prop
  (`items: ViewFn`), the area its children (no slot).
- `Menu` and `ContextMenu` share a crate-private `MenuPanel` (keys,
  focus-out, Esc, the items' `Provider`).
- The context menu is a **manual** popover with its own outside-press
  listener: as an auto popover, Firefox on Linux (which fires `contextmenu`
  on mousedown) took the release of the opening right-click for a press
  outside and closed it at once — the user hit this live: the menu only
  stayed while the button was held.
- `core::anchor` gained `place_at` (a point, flipping left and up).
- Long press: 500ms, 10px slop, the lift's click suppressed; e2e in
  Chromium only (synthetic `Touch`).
- Also from the user, live: a `MenuLink` (`<a>`, content-box) overflowed the
  panel, which scrolled sideways. Items are `border-box`, the panel never
  scrolls sideways, labels end in an ellipsis, and `--menu-max-width`
  (22rem) is the app's to set.
- Menu e2e wait for `document.fonts.ready`: a font landing mid-test moved
  buttons 2px in Firefox.
- e2e: 73 menu tests (Chromium) / 72 (Firefox).

## Where it comes from

- **conrad's `Menu`** (`ui/src/components/Menu.tsx`, 166 lines;
  `theme/components.css:543-613`): the model. The trigger is the caller's,
  the panel and the keyboard are the menu's. Opening focuses the first
  item, ↑ / ↓ move (wrapping), Esc closes and hands focus back to the
  trigger, Tab closes and moves on, a press outside closes. `MenuItem`
  (`onSelect`, `current`: marked, still choosable), `MenuDivider`, a
  `header` above the items (who is signed in), three placements
  (`below-start`, `below-end`, `above-end`). Used for the user menu, the
  book switcher and the tab bar's "More". Its panel is `position: absolute`
  in the page, so an `overflow` parent clips it.
- **rusty-skillet's `KebabMenu`** (2 uses) and **`SplitButton`** (1 use):
  a "⋮" trigger with a `<ul>` of clickable `<li>`s, and a main button with
  a dropdown of extras. No roles, no keyboard, no outside-press close, and
  `<li on:click>` items a keyboard can't reach.
- **webapp-lib** has no menu.

What this keeps and fixes:

| Today | Problem | ferrox-ui |
|---|---|---|
| conrad: the panel is absolutely positioned in the page | Clipped by an `overflow` parent (a `Dialog` body, a scrolling `DataView` strip) | A popover in the top layer, placed by `core::anchor` like the Combobox list; flips above when there's no room below |
| conrad: three fixed placements | The caller must know where the trigger sits | Below by default, above when it doesn't fit, aligned to the trigger's start or end (`align`), kept inside the viewport |
| rusty-skillet: `<li on:click>` items | Not reachable or announced | `role="menuitem"` buttons (or links) with focus moving among them |
| rusty-skillet: stays open until an item is clicked | No Esc, no outside press | Esc, Tab, outside press, choosing |

## Design

### `Menu`: one component, three kinds of trigger

```rust
#[component]
pub fn Menu(
    /// The menu's accessible name: "Recipe actions".
    #[prop(into)] label: Signal<String>,
    /// What opens it: a button with a chevron, an icon, or the caller's view.
    trigger: MenuTrigger,
    /// Which edge of the trigger the panel lines up with.
    #[prop(optional)] align: MenuAlign,          // End (default), Start
    /// Shown above the items, outside the menu itself: who is signed in.
    #[prop(optional, into)] header: Option<ViewFn>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// `MenuItem`, `MenuLink`, `MenuDivider`.
    children: ChildrenFn,
) -> impl IntoView
```

```rust
pub enum MenuTrigger {
    /// A `Button` with the text and a chevron: "Export ▾".
    Button { text: String, family: ButtonFamily },
    /// An `IconButton`; the text is its label. Row actions, "More".
    Icon { icon: IconData, label: String },
    /// The caller's view inside a plain button; `label` names it.
    Custom { view: ViewFn, label: String },
}
// MenuTrigger::button("Export"), ::icon(MORE_VERTICAL, "More actions"),
// ::custom(|| view! { <Avatar …/> "Ada" }, "Account")
```

```rust
<Menu label="Recipe actions" trigger=MenuTrigger::icon(MORE_VERTICAL, "More actions")>
    <MenuItem icon=EDIT on_select=move || edit()>"Edit"</MenuItem>
    <MenuItem on_select=move || duplicate()>"Duplicate"</MenuItem>
    <MenuDivider />
    <MenuItem icon=DELETE danger=true on_select=move || ask_delete()>"Delete"</MenuItem>
</Menu>
```

- **Markup:** the trigger is a `<button aria-haspopup="menu"
  aria-expanded aria-controls>`; the panel is a `<div popover="manual"
  class="menu__panel">` holding the `header` and a `<div role="menu"
  aria-label>` of items. Items are rendered once, on first opening, then
  kept (as the collapsible Panel's body).
- **Keyboard** (WAI-ARIA menu button):
  - On the trigger: Enter, Space or a click opens it and focuses the first
    item; ↓ opens on the first, ↑ on the last.
  - In the menu: ↑ / ↓ move, wrapping; Home / End; a letter moves to the
    next item starting with it. Enter or Space chooses.
  - Esc closes and returns focus to the trigger (and stops there, so an
    enclosing `Dialog` stays open). Tab closes, and focus moves on from the
    trigger.
- **Pointer:** a click on an item chooses it; a press outside closes the
  menu without moving focus back.
- **Choosing** closes the menu, returns focus to the trigger, then runs the
  item's action, so an action that opens a dialog or navigates takes focus
  from there.
- **Placement:** below the trigger, above when there's no room below, its
  start or end edge lined up with the trigger's (`align`), at least as
  wide as the trigger (min 12rem), kept inside the viewport, following the
  trigger while open. `core::anchor` gains the end alignment.
- **Phone:** items 44px tall.

### Items

```rust
#[component]
pub fn MenuItem(
    #[prop(into)] on_select: UnsyncCallback<()>,
    #[prop(optional)] icon: Option<IconData>,
    /// A destructive action: danger colour.
    #[prop(optional)] danger: bool,
    /// Shown, focusable, announced as unavailable, does nothing.
    #[prop(optional, into)] disabled: Signal<bool>,
    /// The one already in force (the open book): marked, still choosable.
    #[prop(optional, into)] current: Signal<bool>,
    children: Children,
) -> impl IntoView

#[component]
pub fn MenuLink(#[prop(into)] href: String, #[prop(optional)] icon: Option<IconData>, children: Children)
// an <a role="menuitem" href>: inside a <Router> the router follows it

#[component]
pub fn MenuDivider() // role="separator"
```

`MenuItem` is a `<button role="menuitem">`; disabled is `aria-disabled`
(not `disabled`), so it stays reachable, as ARIA asks of menu items. A
`current` item has `aria-current="true"` and a check mark.

### `SplitButton`

```rust
<SplitButton
    text="Save"
    on_click=move |_| save()
    menu_label="More save options"
>
    <MenuItem on_select=move || save_as_draft()>"Save as draft"</MenuItem>
    <MenuItem on_select=move || save_and_close()>"Save and close"</MenuItem>
</SplitButton>
```

A main `Button` and, joined to it, a chevron `IconButton` that opens a
`Menu` (`align` End). Two tab stops, two names ("Save", "More save
options"). `family`, `saving` and `disabled` pass to both halves.
rusty-skillet's `SplitButton` moves to it.

### `ContextMenu`

```rust
<ContextMenu label="Recipe actions">
    <ContextTarget slot>
        <Card title=recipe.name …>…</Card>
    </ContextTarget>
    <MenuItem icon=EDIT on_select=move || edit()>"Edit"</MenuItem>
    <MenuItem icon=DELETE danger=true on_select=move || ask_delete()>"Delete"</MenuItem>
</ContextMenu>
```

The same panel, items and keyboard as `Menu`, opened from an area rather
than a trigger (the exact shape of "the area" — a slot, or a wrapper with
the items in a prop — is settled in piece 4):

- **Opening:** a right-click in the area (the browser's own menu is
  suppressed there only); the Menu key or Shift+F10 while focus is in it;
  a long press on touch (about 500ms, cancelled by moving, so scrolling
  never opens it).
- **Placement:** at the pointer, or for the keyboard at the focused
  element's corner; flipped and kept inside the viewport. `core::anchor`
  gains placing at a point.
- **Closing** returns focus to what had it before the menu opened.
- **Hidden by nature:** nothing shows that it exists, so the docs say to
  offer the same actions somewhere visible (a row's ⋮ `Menu`); the demo
  does.

### Feature and CSS

- `menus = []`. It uses `core` (Button, IconButton, the anchor). The
  powerset goes from 32 to 64 combinations.
- `menus/menu.css`, `menus/split-button.css`. Class-only, tokens only.
- New Lucide icons: `MORE_VERTICAL` (`ellipsis-vertical`), and `EDIT`
  (`pencil`) for the demo's row actions.

### What changes in webapp-lib and the apps

rusty-skillet's `KebabMenu` and `SplitButton` move to `Menu` and
`SplitButton`. webapp-lib gets no menu adapter; the app-shell's user menu
(the name that left the side menu's footer, 2026-10-23) is a `Menu` with a
`Custom` trigger and a `header`, built in webapp-lib's app-shell story.

## Pieces

Each passes `scripts/release-check.sh`. The demo gets a `/menus` page.

1. **Feature, Menu and MenuItem.** `MenuTrigger` (all three), `MenuAlign`,
   `MenuItem` (plain), `MenuDivider`, `menu.css`, anchor's end alignment,
   `MORE_VERTICAL`. e2e:
   - the trigger's `aria-haspopup` / `aria-expanded` / `aria-controls`; a
     menu named by `label` of `menuitem`s;
   - opening by click, Enter, Space (first item focused), ↓ (first), ↑
     (last);
   - ↑ / ↓ wrap, Home / End, a letter;
   - choosing (click, Enter, Space) closes, returns focus, runs the action
     once;
   - Esc returns focus; Tab moves on; an outside press closes without
     taking focus back;
   - inside a `Dialog`: not clipped, Esc closes only the menu;
   - placement: below, above near the bottom, start / end alignment,
     inside the viewport on a phone, follows the trigger on scroll;
   - each trigger kind; a disabled `Menu` doesn't open.
2. **Item variants.** `icon`, `danger`, `disabled` (reachable, announced,
   inert), `current` (marked, choosable), `MenuLink` (router navigation,
   no page load), `header`, `EDIT`. e2e for each, and 44px items on a phone.
3. **SplitButton.** `split-button.css`. e2e: two buttons with their own
   names; the main one runs its action; the chevron opens the menu aligned
   to the group's end; `saving` / `disabled` reach both; the halves join
   with no gap.
4. **ContextMenu.** Point placement in `core::anchor`. e2e: a right-click
   in the area opens it at the pointer (and not the browser's); outside the
   area nothing changes; Shift+F10 and the Menu key open it at the focused
   element with the first item focused; a long press opens it on touch, a
   moving one doesn't; it flips and stays inside the viewport near an edge;
   closing returns focus to what had it; the items behave as in `Menu`.
5. **Docs.** A menus section in `ferrox-ui/README.md`, root and demo
   READMEs, this plan's status.

Not here: checkable items (`menuitemcheckbox`, `menuitemradio`), submenus,
Menubar, a command palette. They wait for an
app that needs them.

## Open points

1. **One `Menu` with a `MenuTrigger`** (button with chevron, icon, custom),
   recommended: one component holds the keyboard contract, and the three
   triggers every use so far needs are one argument apart. The alternatives
   are separate components (`MenuButton`, `MoreMenu`, `Menu` with a
   caller-built trigger), or conrad's way, a trigger the caller renders
   with attributes the menu hands it, which in Leptos means a render
   closure taking a props struct.
2. **Real focus on the items** (conrad's; arrows move the browser's
   focus), recommended, over `aria-activedescendant` with focus left on the
   trigger (the Combobox's way: right for a text field, not for a menu).
3. **↑ / ↓ wrap** at the ends (conrad's; ARIA allows either), recommended.
4. **SplitButton in this group**, recommended: rusty-skillet has one, and
   it's a `Menu` with a different trigger. The alternative is leaving it
   for when a second app needs it.
