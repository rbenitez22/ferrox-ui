# `Switch`

An on/off that **is** the act: a filter over what the screen shows, or a
preference written straight back. A field that a dialog's Save sends stays a
`Checkbox`. It's a `<button role="switch" aria-checked>` with a track, a knob
and the words inside, so the words name it and are part of the target.

<!-- example: switch -->
```rust
use ferrox_ui::Switch;

view! { <Switch checked=show_archived label="Show archived accounts" /> }
```
<!-- /example -->

Props: `checked` (required, `RwSignal<bool>`), `label` (required), `disabled`
(off and unusable: still loading, or not this user's to change), `help` and
`help_skip_tab` (a `?` beside the switch, not inside it), `help_reveal`
(see [`HelpReveal`](help.md#helpreveal-only-while-in-use)), `class`. A click or
Space flips it, and the knob slides across. `switch_classes(extra)` returns
the class list.
