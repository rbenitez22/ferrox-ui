# CSS classes

Every class each group's stylesheets define, for markup that needs the
same look without the component (each component also has a `*_classes`
function). Injected by `ferrox_ui::init()` only for the compiled groups.

## core

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

## overlay

| Sheet | Classes |
|---|---|
| `modal.css` | `.dialog-open` on `<body>`: no page scroll while a modal is open |
| `dialog.css` | `.dialog`, `.dialog--wide`, `.dialog::backdrop`, `.dialog__title-bar`, `.dialog__title`, `.dialog__help`, `.dialog__close`, `.dialog__body`, `.dialog__footer` |
| `sheet.css` | `.sheet`, `.sheet--{left,right,bottom}`, `.sheet::backdrop`, `.sheet__title-bar`, `.sheet__title`, `.sheet__help`, `.sheet__close`, `.sheet__body` |

These are injected only when the `overlay` feature is on.

## forms

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

## navigation

| Sheet | Classes |
|---|---|
| `nav.css` | `.nav-list`, `.nav-list__items`; `.nav-link`, `.nav-link__icon`, `.nav-link[aria-current='page']`; `.nav-group`, `.nav-group--collapsible`, `.nav-group__heading`, `.nav-group__toggle` (`[aria-expanded='true']`), `.nav-group__label`, `.nav-group__chevron`, `.nav-group__list` |
| `side-menu.css` | `.side-menu` (with `.sheet`), `.side-menu__content`, `.side-menu__footer` |

These are injected only when the `navigation` feature is on.

## layout

| Sheet | Classes |
|---|---|
| `page-grid.css` | `.page-grid`, `.page-grid--{narrow,medium,wide,full}`, `.page-grid--side-{end,start}`, `.page-grid__main`, `.page-grid__side` |
| `flow.css` | `.stack`, `.cluster`, `.stack--gap-{none,xs,sm,md,lg,xl}` (and `.cluster--gap-…`), `.stack--align-{stretch,start,center,end}` (and `.cluster--align-…`), `.cluster--justify-{start,center,end,between}` |
| `panel.css` | `.panel`, `.panel--{compact,flush,warning,danger,collapsible}`, `.panel__header` (the band), `.panel__heading` (the title and its `?`), `.panel__title`, `.panel__actions`, `.panel__body`; collapsible: `.panel__toggle` (`[aria-expanded='true']`), `.panel__title-text`, `.panel__summary`, `.panel__chevron`, `.panel__body[hidden]` |
| `card.css` | `.card`, `.card--clickable`, `.card--compact`, `.card__header`, `.card__title`, `.card__link` (the title as a link or button, its `::after` the stretched target), `.card__actions`, `.card__body` |

These are injected only when the `layout` feature is on.

## data

| Sheet | Classes |
|---|---|
| `data-view.css` | `.data-view`, `.data-view--{grid-sm,grid-md,grid-lg,list,row,wrap}`, `.data-view__list` (`[hidden]` while there are no items), `.data-view__item`, `.data-view__more` (the end marker), `.data-view__error`, `.data-view__empty` |
| `layout-switch.css` | `.data-layout-switch` |
| `pagination.css` | `.pagination`, `.pagination__list`, `.pagination__step` (Previous / Next), `.pagination__slot`, `.pagination__page` (`[aria-current='page']`), `.pagination__gap`, `.pagination__position` (the phone's `6 / 20`) |

These are injected only when the `data` feature is on.

## menus

| Sheet | Classes |
|---|---|
| `menu.css` | `.menu` (the wrapper), `.menu__trigger` (`[aria-expanded='true']`), `.menu__chevron`, `.menu__trigger--custom`, `.menu__panel` (the popover; `data-side`), `.menu__header`, `.menu__items` (`role="menu"`), `.menu__item` (`--danger`, `--current`, `[aria-disabled='true']`), `.menu__icon`, `.menu__label`, `.menu__check`, `.menu__divider`, `.menu__trigger--chevron` |
| `split-button.css` | `.split-button`, `.split-button__main`, `.split-button__menu` |

`menu.css` also holds `.context-menu` (the area's wrapper) and reads
`--menu-max-width`.

These are injected only when the `menus` feature is on.

A unit test (`css.rs`) fails if any shipped selector doesn't start with a
class.
