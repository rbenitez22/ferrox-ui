# `PasswordInput`

A password field (`.form-input` inside `.password-input`) with a show/hide
button at its right end. The button is a real `<button>` with one label
("Show password"); `aria-pressed` says whether the password is showing, and
the eye is crossed out while it is.

<!-- example: password-input -->
```rust
use ferrox_ui::{Field, PasswordInput};

view! {
    <Field label="Password"><PasswordInput value=password /></Field>

    <Field label="New password" hint="At least four words, or a long mix">
        <PasswordInput value=new_password autocomplete="new-password" strength=true min_bits=70.0 />
    </Field>
}
```
<!-- /example -->

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

<!-- example: password-strength -->
```rust
use ferrox_ui::{password_strength, StrengthLevel};

let strength = password_strength(&new_password.get(), 80.0); // PasswordStrength { bits, level }
if strength.level != StrengthLevel::Strong { /* say so under the field */ }
```
<!-- /example -->

- `password_entropy(&str) -> f64`: length × log2 of the size of the
  character sets used. It counts characters, not bytes ("é" is one).
- `strength_level(bits, min_bits) -> StrengthLevel`: `VeryWeak` under 25
  bits, `Weak` under 40, `Fair` under `min_bits`, then `Strong`.
  `StrengthLevel::class()` is its meter class.
- `password_strength(&str, min_bits) -> PasswordStrength`: both.

The new-password and confirm pair stays in webapp-lib, built from two
`PasswordInput`s.
