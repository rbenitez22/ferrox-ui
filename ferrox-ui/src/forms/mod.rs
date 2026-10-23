//! The `forms` group (feature `forms`): a `Field` that wires a label and one
//! message to its control, and the controls.

mod field;
mod input;
mod textarea;

pub use field::{field_classes, Field};
pub use input::{Input, InputKind};
pub use textarea::Textarea;
