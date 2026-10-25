# ferrox-ui: index

Every public component (with its props) and every other public item, by
group, with the first line of its documentation. Generated from the source
by `tests/docs_current.rs`: don't edit it; a test fails when it's stale.
Each component's page (`docs/<name>.md`) has its example and behaviour.

## core (always on)

### `AlertSeverity` (enum)
How serious an [`Alert`] is: its colour, its icon, and its role (`status` for info and success, `alert` for warning and error). Variants: Info, Success, Warning, Error.

### `alert_classes` (function)
`fn alert_classes(severity: AlertSeverity, extra: &str) -> String`: The class list for an `.alert`.

### `Alert` (component)
An in-page or in-form message that **never times out**: the reason a submit failed, a server refusal no single field owns.
- `severity: AlertSeverity`
- `icon: bool`, default `true`
- `on_close: Option<UnsyncCallback<()>>`, optional
- `close_label: Signal<String>`, default `"Close"`
- `class: MaybeProp<String>`, optional
- `children: Children`

### `ButtonFamily` (enum)
What a [`Button`] does, which sets its look: affirmative, committing, dismissive or destructive. Variants: Primary, Secondary, Cancel, Danger.

### `ButtonSize` (enum)
A [`Button`]'s size: `Sm`, `Md` (the default), `Lg`. Variants: Sm, Md, Lg.

### `button_classes` (function)
`fn button_classes(family: ButtonFamily, quiet: bool, size: ButtonSize, saving: bool, extra: &str) -> String`: The class list for a `.btn`: base, family, emphasis, size, state, extra.

### `Button` (component)
A standard padded button (`.btn`).
- `family: ButtonFamily`, optional
- `quiet: bool`, optional
- `size: ButtonSize`, optional
- `saving: Signal<bool>`, optional
- `disabled: Signal<bool>`, optional
- `button_type: &'static str`, default `"button"`
- `on_click: Option<UnsyncCallback<()>>`, optional
- `on_refused: Option<UnsyncCallback<()>>`, optional
- `title: MaybeProp<String>`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `help_button_classes` (function)
`fn help_button_classes(extra: &str) -> String`: The class list for a help button: an `.icon-btn` sized to sit by text.

### `HelpButton` (component)
A `?` that opens a help topic in a popover beside it.
- `topic: HelpTopic`
- `skip_tab: bool`, optional
- `help_label: Signal<String>`, default `"Help"`
- `guide_label: Signal<String>`, default `"Full guide"`
- `ask_label: Signal<String>`, default `"Ask about this"`
- `suggested_label: Signal<String>`, default `", suggested"`
- `kind: &'static str`, default `"help"`
- `about: MaybeProp<String>`, optional
- `value: MaybeProp<String>`, optional
- `id: Option<String>`, optional
- `class: MaybeProp<String>`, optional

### `HelpKeys` (component)
F1 opens the help of whatever has focus; Shift+F1 asks the assistant about it directly (as F1, without a [`HelpAssistant`](crate::HelpAssistant)).
- `children: Children`

### `HelpRequest` (struct)
What "Ask about this" hands the app's assistant: which topic, on which component, and everything the app declared around it. Fields: topic_id, topic_title, component, label, context, value.

### `HelpContext` (struct)
Key/value pairs a `HelpScope` adds to every help request inside it: a fixed list, or a closure that follows the page.

### `HelpReveal` (enum)
When a control's `?` shows: always, or only while the control is in use. Variants: Always, WhileActive.

### `HelpScope` (component)
Declares what an assistant should know about everything inside it: this screen, this account, this section.
- `context: HelpContext`, optional
- `reveal: Option<HelpReveal>`, optional
- `children: Children`

### `HelpAssistant` (component)
Turns on "Ask about this" in every help popover inside it, usually once around the whole app.
- `on_ask: UnsyncCallback<HelpRequest>`
- `children: Children`

### `HelpSignal` (struct)
A fact that may mean the user needs help: reported by a component, or by the app. Fields: kind, component, about, topic_id, context, detail.

### `HelpSignalKind` (enum)
What happened. Variants: ErrorShown, ErrorCleared, ErrorPersisted, SubmitRefused, HelpOpened, HelpDismissed, NoMatch, App.

### `HelpSuggestions` (struct)
The topics an app currently suggests, marked on their `?`s.

### `HelpSignals` (component)
Receives every [`HelpSignal`] reported inside it: by its components (an error that persists, a refused submit, help dismissed at once, …) and by the app ([`help_signals`]).
- `on_signal: UnsyncCallback<HelpSignal>`
- `suggestions: HelpSuggestions`, optional
- `children: Children`

### `HelpReporter` (struct)
Reports signals to the enclosing [`HelpSignals`], with the context of the `HelpScope`s where it was made.

### `help_signals` (function)
`fn help_signals() -> HelpReporter`: A reporter for here: call it in a component's body, keep it, and report from handlers.

### `HelpBlock` (enum)
One part of a help topic. Variants: Paragraph, List, Table.

### `HelpTopic` (struct)
One thing a `?` can explain: a title, its blocks, and where the same words sit in a full guide.

### `MaybeHelp` (struct)
A component's `help` prop: a topic, or none.

### `icon_button_classes` (function)
`fn icon_button_classes(danger: bool, extra: &str) -> String`: The class list for an `.icon-btn`.

### `IconButton` (component)
A compact glyph button (`.icon-btn`): 1.75rem square, no border, answers in colour rather than size.
- `label: Signal<String>`
- `pressed: Option<Signal<bool>>`, optional
- `danger: bool`, optional
- `disabled: Signal<bool>`, optional
- `on_click: Option<UnsyncCallback<()>>`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `SpinnerKind` (enum)
Where a [`Spinner`] sits, which sets its size and spacing. Variants: Region, Inline, Field.

### `spinner_status_classes` (function)
`fn spinner_status_classes(kind: SpinnerKind, extra: &str) -> String`: The class list for a spinner's status wrapper.

### `Spinner` (component)
A spinner for work that leaves the rest of the screen usable (`role="status"`).
- `label: Signal<String>`
- `kind: SpinnerKind`, optional
- `class: MaybeProp<String>`, optional

### `LoadingOverlay` (component)
The full-page blocking overlay: a scrim with a large spinner.
- `label: MaybeProp<String>`, optional
- `class: MaybeProp<String>`, optional

### `Toolbar` (component)
A row of buttons (`.toolbar`).
- `title: MaybeProp<String>`, optional
- `compact: bool`, optional
- `help: MaybeHelp`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `ToolbarGroup` (component)
Buttons that belong together inside a `Toolbar`.
- `class: MaybeProp<String>`, optional
- `children: Children`

### `toolbar_classes` (function)
`fn toolbar_classes(compact: bool, extra: &str) -> String`: The class list for a `.toolbar`.

## overlay (feature `overlay`)

### `ConfirmDialog` (component)
The standard destructive confirmation (`role="alertdialog"`).
- `open: RwSignal<bool>`
- `title: Signal<String>`
- `confirm_label: Signal<String>`
- `on_confirm: UnsyncCallback<()>`
- `on_cancel: Option<UnsyncCallback<()>>`, optional
- `saving: Signal<bool>`, optional
- `cancel_label: Signal<String>`, default `"Cancel"`
- `close_label: Signal<String>`, default `"Close"`
- `help: MaybeHelp`, optional
- `children: ChildrenFn`

### `DialogSize` (enum)
How wide a [`Dialog`] is. Variants: Md, Wide.

### `DialogAction` (struct)
One footer button of a [`Dialog`]: its label, what it does, and its state. Fields: label, on_click, family, saving, disabled.

### `dialog_classes` (function)
`fn dialog_classes(size: DialogSize, extra: &str) -> String`: The class list for a `.dialog`.

### `Dialog` (component)
A modal dialog: a title bar with a close button, the body, and a footer band.
- `open: RwSignal<bool>`
- `title: Signal<String>`
- `on_close: Option<UnsyncCallback<()>>`, optional
- `cancel_label: Signal<String>`, default `"Cancel"`
- `no_cancel: bool`, optional
- `affirm: Option<DialogAction>`, optional
- `alternate: Option<DialogAction>`, optional
- `submit_on_enter: bool`, default `true`
- `size: DialogSize`, optional
- `alert: bool`, optional
- `help: MaybeHelp`, optional
- `close_label: Signal<String>`, default `"Close"`
- `class: MaybeProp<String>`, optional
- `children: ChildrenFn`

### `SheetSide` (enum)
Which edge a [`Sheet`] slides in from. Variants: Left, Right, Bottom.

### `sheet_classes` (function)
`fn sheet_classes(side: SheetSide, extra: &str) -> String`: The class list for a `.sheet`.

### `Sheet` (component)
A modal panel that slides in from an edge: a title bar with a close button, then the body.
- `open: RwSignal<bool>`
- `on_close: Option<UnsyncCallback<()>>`, optional
- `side: SheetSide`, optional
- `title: Signal<String>`
- `close_label: Signal<String>`, default `"Close"`
- `help: MaybeHelp`, optional
- `class: MaybeProp<String>`, optional
- `children: ChildrenFn`

## forms (feature `forms`)

### `checkbox_classes` (function)
`fn checkbox_classes(invalid: bool, extra: &str) -> String`: The class list for a `.checkbox`.

### `Checkbox` (component)
A checkbox and its words, as one `<label>` (the words are part of the target), bound two-way to `checked`, with at most one message under it.
- `checked: RwSignal<bool>`
- `label: Signal<String>`
- `hint: MaybeProp<String>`, optional
- `error: MaybeProp<String>`, optional
- `name: MaybeProp<String>`, optional
- `required: bool`, optional
- `disabled: Signal<bool>`, optional
- `help: MaybeHelp`, optional
- `help_skip_tab: bool`, optional
- `help_reveal: Option<HelpReveal>`, optional
- `help_shares_value: bool`, optional
- `class: MaybeProp<String>`, optional

### `matches_text` (function)
`fn matches_text(label: &str, text: &str) -> bool`: Whether an item's label matches what was typed: case-insensitive "contains".

### `Combobox` (component)
A text field that filters a list and lets one item be chosen: the ARIA combobox pattern (`role="combobox"` with a `listbox` of `option`s, `aria-activedescendant` for the active one).
- `items: Signal<Vec<T>>`
- `key_of: impl Fn(&T) -> String + Send + Sync + 'static`
- `label_of: impl Fn(&T) -> String + Send + Sync + 'static`
- `text: RwSignal<String>`
- `on_select: UnsyncCallback<Option<T>>`
- `placeholder: MaybeProp<String>`, optional
- `clearable: bool`, default `true`
- `toggleable: bool`, default `true`
- `on_refresh: Option<UnsyncCallback<()>>`, optional
- `refreshing: Signal<bool>`, optional
- `on_create: Option<UnsyncCallback<String>>`, optional
- `create_label: Option<Callback<String, String>>`, optional
- `empty_label: Signal<String>`, default `"No matches"`
- `clear_label: Signal<String>`, default `"Clear"`
- `toggle_label: Signal<String>`, default `"Show all"`
- `refresh_label: Signal<String>`, default `"Refresh"`
- `refreshing_label: Signal<String>`, default `"Refreshing…"`
- `name: MaybeProp<String>`, optional
- `required: bool`, optional
- `disabled: Signal<bool>`, optional
- `autofocus: bool`, optional
- `class: MaybeProp<String>`, optional

### `field_classes` (function)
`fn field_classes(extra: &str) -> String`: The class list for a `.field`.

### `Field` (component)
A label, one control, and at most one message under it.
- `label: Signal<String>`
- `hint: MaybeProp<String>`, optional
- `error: MaybeProp<String>`, optional
- `help: MaybeHelp`, optional
- `help_skip_tab: bool`, optional
- `help_reveal: Option<HelpReveal>`, optional
- `help_shares_value: bool`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `InputKind` (enum)
An [`Input`]'s `type`: what it holds, which sets the keyboard on a phone and the browser's own checks. Variants: Text, Email, Search, Tel, Url, Number.

### `Input` (component)
A one-line text field (`.form-input`), bound two-way to `value`.
- `value: RwSignal<String>`
- `kind: InputKind`, optional
- `placeholder: MaybeProp<String>`, optional
- `name: MaybeProp<String>`, optional
- `autocomplete: MaybeProp<String>`, optional
- `required: bool`, optional
- `readonly: Signal<bool>`, optional
- `disabled: Signal<bool>`, optional
- `autofocus: bool`, optional
- `class: MaybeProp<String>`, optional

### `PasswordInput` (component)
A password field (`.form-input` in a `.password-input`), bound two-way to `value`, with a button that shows and hides what was typed.
- `value: RwSignal<String>`
- `autocomplete: String`, default `"current-password"`
- `strength: bool`, optional
- `min_bits: f64`, default `80.0`
- `strength_labels: StrengthLabels`, optional
- `show_label: Signal<String>`, default `"Show password"`
- `placeholder: MaybeProp<String>`, optional
- `name: MaybeProp<String>`, optional
- `required: bool`, optional
- `disabled: Signal<bool>`, optional
- `autofocus: bool`, optional
- `class: MaybeProp<String>`, optional

### `SelectOption` (struct)
One choice of a [`Select`]: the value it sets and the text it shows. Fields: value, label.

### `Select` (component)
A dropdown of fixed choices (`.form-select`, the native `<select>`), bound two-way to `value` (the chosen option's `value`; empty for none).
- `value: RwSignal<String>`
- `options: Signal<Vec<SelectOption>>`
- `placeholder: Signal<String>`, default `"Select…"`
- `loading: Signal<bool>`, optional
- `loading_label: Signal<String>`, default `"Loading…"`
- `on_refresh: Option<UnsyncCallback<()>>`, optional
- `refreshing: Signal<bool>`, optional
- `refresh_label: Signal<String>`, default `"Refresh"`
- `refreshing_label: Signal<String>`, default `"Refreshing…"`
- `on_create: Option<UnsyncCallback<()>>`, optional
- `create_label: Signal<String>`, default `"New…"`
- `name: MaybeProp<String>`, optional
- `required: bool`, optional
- `disabled: Signal<bool>`, optional
- `class: MaybeProp<String>`, optional

### `StrengthLevel` (enum)
How strong a password is, in four levels, from its entropy. Variants: VeryWeak, Weak, Fair, Strong.

### `PasswordStrength` (struct)
A password's entropy and its level. Fields: bits, level.

### `StrengthLabels` (struct)
The words the strength meter shows for each level (English by default). Fields: very_weak, weak, fair, strong.

### `password_entropy` (function)
`fn password_entropy(password: &str) -> f64`: Entropy in bits: length × log2(the size of the character sets used).

### `strength_level` (function)
`fn strength_level(bits: f64, min_bits: f64) -> StrengthLevel`: The level for an entropy, given the minimum that counts as strong.

### `password_strength` (function)
`fn password_strength(password: &str, min_bits: f64) -> PasswordStrength`: How strong `password` is, against `min_bits` for "strong".

### `switch_classes` (function)
`fn switch_classes(extra: &str) -> String`: The class list for a `.switch`.

### `Switch` (component)
A setting said as a switch: on or off, taking effect as it is flipped (`<button role="switch" aria-checked>`), bound two-way to `checked`.
- `checked: RwSignal<bool>`
- `label: Signal<String>`
- `disabled: Signal<bool>`, optional
- `help: MaybeHelp`, optional
- `help_skip_tab: bool`, optional
- `help_reveal: Option<HelpReveal>`, optional
- `class: MaybeProp<String>`, optional

### `Textarea` (component)
A multi-line text field (`.form-textarea`), bound two-way to `value`.
- `value: RwSignal<String>`
- `rows: u32`, default `4`
- `placeholder: MaybeProp<String>`, optional
- `name: MaybeProp<String>`, optional
- `required: bool`, optional
- `readonly: Signal<bool>`, optional
- `disabled: Signal<bool>`, optional
- `autofocus: bool`, optional
- `class: MaybeProp<String>`, optional

## navigation (feature `navigation`)

### `nav_group_classes` (function)
`fn nav_group_classes(collapsible: bool, extra: &str) -> String`: The class list for a `.nav-group`.

### `NavGroup` (component)
Links that belong together inside a [`NavList`](crate::NavList), under a heading that names them.
- `label: Signal<String>`
- `collapsible: bool`, optional
- `start_expanded: bool`, optional
- `expanded: Option<RwSignal<bool>>`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `nav_link_classes` (function)
`fn nav_link_classes(extra: &str) -> String`: The class list for a `.nav-link`.

### `NavLink` (component)
One link in a [`NavList`](crate::NavList): a list item holding a router link (`.nav-link`), with an optional icon before the words.
- `href: String`
- `exact: bool`, optional
- `icon: Option<IconData>`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `nav_list_classes` (function)
`fn nav_list_classes(extra: &str) -> String`: The class list for a `.nav-list`.

### `NavList` (component)
A labelled `<nav>` landmark holding a list of [`NavLink`](crate::NavLink)s, so a screen reader hears the landmark's name and how many links it holds.
- `label: Signal<String>`
- `class: MaybeProp<String>`, optional
- `children: Children`

### `side_menu_classes` (function)
`fn side_menu_classes(extra: &str) -> String`: The class list for a side menu's sheet: `.side-menu`, then any extra classes.

### `SideMenu` (component)
Navigation that slides in from an edge: a [`Sheet`](crate::Sheet) holding a [`NavList`](crate::NavList) of the children, and an optional `footer` pinned to the bottom.
- `open: RwSignal<bool>`
- `title: Signal<String>`
- `nav_label: Signal<String>`, default `"Sections"`
- `side: SheetSide`, optional
- `footer: Option<ViewFn>`, optional
- `help: MaybeHelp`, optional
- `close_label: Signal<String>`, default `"Close"`
- `class: MaybeProp<String>`, optional
- `children: ChildrenFn`

### `SideMenuButton` (component)
The button that opens a [`SideMenu`](crate::SideMenu): an `IconButton` drawing `MENU`, named by `label`, with `aria-haspopup="dialog"` and `aria-expanded` following `open`.
- `open: RwSignal<bool>`
- `label: Signal<String>`, default `"Menu"`
- `class: MaybeProp<String>`, optional

## layout (feature `layout`)

### `app_bar_classes` (function)
`fn app_bar_classes(sticky: bool, extra: &str) -> String`: The class list for an `.app-bar`.

### `AppBar` (component)
The app's top band: what opens its menu, its name, and its actions (a user menu, a search).
- `title: Signal<String>`
- `href: MaybeProp<String>`, optional
- `start: Option<ViewFnOnce>`, optional
- `sticky: bool`, default `true`
- `class: MaybeProp<String>`, optional
- `children: Option<Children>`, optional

### `card_classes` (function)
`fn card_classes(clickable: bool, compact: bool, extra: &str) -> String`: The class list for a `.card`.

### `Card` (component)
A surface for one thing (a record, an item, a choice): an `<article>` with an optional heading, its own actions, and the body.
- `title: MaybeProp<String>`, optional
- `heading_level: HeadingLevel`, default `HeadingLevel::H3`
- `href: MaybeProp<String>`, optional
- `on_click: Option<UnsyncCallback<()>>`, optional
- `actions: Option<ViewFnOnce>`, optional
- `compact: bool`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `Facts` (component)
A record's facts, label beside value: a `<dl>` in two columns, labels muted, one under the other on a phone.
- `class: MaybeProp<String>`, optional
- `children: Children`

### `Fact` (component)
One line of [`Facts`]: its `label` (`<dt>`) and its value, the children (`<dd>`).
- `label: Signal<String>`
- `class: MaybeProp<String>`, optional
- `children: Children`

### `Gap` (enum)
The space between children, on the theme's spacing tokens. Variants: None, Xs, Sm, Md, Lg, Xl.

### `Align` (enum)
Where children sit across the flow: a `Stack`'s width, a `Cluster`'s height. Variants: Stretch, Start, Center, End.

### `Justify` (enum)
Where a `Cluster`'s children sit along its row. Variants: Start, Center, End, Between.

### `stack_classes` (function)
`fn stack_classes(gap: Gap, align: Align, extra: &str) -> String`: The class list for a `.stack`.

### `cluster_classes` (function)
`fn cluster_classes(gap: Gap, align: Align, justify: Justify, extra: &str) -> String`: The class list for a `.cluster`.

### `Stack` (component)
Children one under another, a token gap between: a form's fields, a column of panels, a card's lines.
- `gap: Gap`, optional
- `align: Align`, default `Align::Stretch`
- `class: MaybeProp<String>`, optional
- `children: Children`

### `Cluster` (component)
Children side by side, wrapping onto the next line when they don't fit, a token gap between: a row of buttons, tags, a label and its value.
- `gap: Gap`, default `Gap::Sm`
- `align: Align`, default `Align::Center`
- `justify: Justify`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `HeadingLevel` (enum)
Which heading element a titled component renders, so it sits at the right place in the page's outline. Variants: H1, H2, H3, H4.

### `PageWidth` (enum)
How wide a page's content may grow. Variants: Narrow, Medium, Wide, Full.

### `SidePosition` (enum)
Which side of the main column a `PageGrid`'s side column takes. Variants: Start, End.

### `page_grid_classes` (function)
`fn page_grid_classes(width: PageWidth, side: Option<SidePosition>, extra: &str) -> String`: The class list for a `.page-grid`.

### `PageGrid` (component)
A page: its content capped at a width and centred, with the page's padding, and an optional side column (filters, a summary, related links) beside the main one, which moves under it on a phone.
- `width: PageWidth`, optional
- `side: Option<ViewFnOnce>`, optional
- `side_position: SidePosition`, optional
- `landmark: bool`, default `true`
- `class: MaybeProp<String>`, optional
- `children: Children`

### `PanelTone` (enum)
What a panel holds, said in its colour. Variants: Default, Warning, Danger.

### `panel_classes` (function)
`fn panel_classes(collapsible: bool, compact: bool, flush: bool, tone: PanelTone, extra: &str) -> String`: The class list for a `.panel`.

### `Panel` (component)
A titled box: a header band with the title (a real heading) and the actions, then the body.
- `title: Signal<String>`
- `heading_level: HeadingLevel`, optional
- `actions: Option<ViewFnOnce>`, optional
- `collapsible: bool`, optional
- `start_expanded: bool`, optional
- `expanded: Option<RwSignal<bool>>`, optional
- `summary: MaybeProp<String>`, optional
- `compact: bool`, optional
- `flush: bool`, optional
- `tone: PanelTone`, optional
- `help: MaybeHelp`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

## data (feature `data`)

### `data_view_classes` (function)
`fn data_view_classes(layout: DataLayout, extra: &str) -> String`: The class list for a `.data-view`.

### `DataView` (component)
A record list rendered through one item template and laid out as the screen needs: a grid of cards, a list of rows, a sideways strip, or wrapping chips (`layout`).
- `items: Signal<Vec<T>>`
- `key_of: impl Fn(&T) -> String + Clone + Send + Sync + 'static`
- `item: impl Fn(T) -> IV + Clone + Send + Sync + 'static`
- `label: Signal<String>`
- `layout: Signal<DataLayout>`, default `DataLayout::default()`
- `loading: Signal<bool>`, optional
- `loading_label: Signal<String>`, default `"Loading…"`
- `error: MaybeProp<String>`, optional
- `on_retry: Option<UnsyncCallback<()>>`, optional
- `retry_label: Signal<String>`, default `"Retry"`
- `empty_label: Signal<String>`, default `"Nothing here yet."`
- `empty_view: Option<ViewFn>`, optional
- `on_more: Option<UnsyncCallback<()>>`, optional
- `has_more: Signal<bool>`, optional
- `loading_more: Signal<bool>`, optional
- `loading_more_label: Signal<String>`, default `"Loading more…"`
- `class: MaybeProp<String>`, optional

### `CardSize` (enum)
The narrowest a grid column may be: the grid fits as many as it can. Variants: Sm, Md, Lg.

### `DataLayout` (enum)
How a `DataView` lays out its items. Variants: Grid, List, Row, Wrap.

### `DataLayoutLabels` (struct)
The words of a [`DataLayoutSwitch`]: the group's name and each layout's. Fields: group, grid_sm, grid, grid_lg, list, row, wrap.

### `layout_icon` (function)
`fn layout_icon(layout: DataLayout) -> IconData`: The icon a layout's button draws.

### `DataLayoutSwitch` (component)
Lets the user pick a `DataView`'s layout: a labelled group of icon buttons, one per choice, the current one pressed (`aria-pressed`).
- `layout: RwSignal<DataLayout>`
- `choices: Vec<DataLayout>`, default `vec! [DataLayout::Grid(CardSize::Md), DataLayout::List]`
- `labels: DataLayoutLabels`, optional
- `class: MaybeProp<String>`, optional

### `PageItem` (enum)
One slot of a [`Pagination`]: a page number, or a gap standing for two or more pages not shown. Variants: Page, Gap.

### `page_window` (function)
`fn page_window(current: usize, count: usize) -> Vec<PageItem>`: Which page numbers show for `current` (1-based) of `count` pages.

### `PaginationLabels` (struct)
The words of a [`Pagination`]. Fields: previous, next, page, position.

### `Pagination` (component)
Moves through pages of anything (a `DataView`, a table, search results): Previous, the page numbers ([`page_window`]) and Next, in a `<nav aria-label>`.
- `page: RwSignal<usize>`
- `page_count: Signal<usize>`
- `label: Signal<String>`, default `"Pages"`
- `labels: PaginationLabels`, optional
- `class: MaybeProp<String>`, optional

## menus (feature `menus`)

### `ContextMenu` (component)
A menu of actions on an area (a row, a card), opened from the area itself rather than a trigger: the same panel, items and keyboard as [`crate::Menu`].
- `label: Signal<String>`
- `menu: ViewFnOnce`
- `header: Option<ViewFnOnce>`, optional
- `disabled: Signal<bool>`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `menu_item_classes` (function)
`fn menu_item_classes(danger: bool, current: bool, extra: &str) -> String`: The class list for a `.menu__item`.

### `MenuItem` (component)
One thing a [`crate::Menu`] offers: a `<button role="menuitem">`.
- `on_select: UnsyncCallback<()>`
- `icon: Option<IconData>`, optional
- `danger: bool`, optional
- `disabled: Signal<bool>`, optional
- `current: Signal<bool>`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `MenuLink` (component)
A [`crate::Menu`] item that goes somewhere: an `<a role="menuitem" href>`.
- `href: String`
- `icon: Option<IconData>`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `MenuDivider` (component)
A line between groups of items (`role="separator"`).

### `MenuTrigger` (enum)
What opens a [`Menu`]. Variants: Button, Icon, Custom, Chevron.

### `MenuAlign` (enum)
Which edge of the trigger a [`Menu`]'s panel lines up with. Variants: End, Start.

### `Menu` (component)
A list of actions behind a trigger, as ARIA's menu button has it.
- `label: Signal<String>`
- `trigger: MenuTrigger`
- `align: MenuAlign`, optional
- `header: Option<ViewFnOnce>`, optional
- `disabled: Signal<bool>`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`

### `SplitButton` (component)
A main action and, joined to it, a chevron that opens related actions: "Save ▾" with "Save as draft", "Save and close".
- `text: Signal<String>`
- `on_click: UnsyncCallback<()>`
- `menu_label: Signal<String>`
- `family: ButtonFamily`, optional
- `saving: Signal<bool>`, optional
- `disabled: Signal<bool>`, optional
- `class: MaybeProp<String>`, optional
- `children: Children`
