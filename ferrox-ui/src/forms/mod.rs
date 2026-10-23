//! The `forms` group (feature `forms`): a `Field` that wires a label and one
//! message to its control, and the controls.

mod checkbox;
mod field;
mod input;
mod password;
mod select;
mod strength;
mod switch;
mod textarea;

pub use checkbox::{checkbox_classes, Checkbox};
pub use field::{field_classes, Field};
pub use input::{Input, InputKind};
pub use password::PasswordInput;
pub use select::{Select, SelectOption};
pub use strength::{password_entropy, password_strength, strength_level, PasswordStrength, StrengthLabels, StrengthLevel};
pub use switch::{switch_classes, Switch};
pub use textarea::Textarea;
