//! Small shared traits for records, and their derives (feature `derive`, on
//! by default). Std only, so a crate shared by a server and a WASM app can
//! use them.
//!
//! ```rust
//! use ferrox_traits::{HasId, HasName};
//!
//! #[derive(HasId, HasName)]
//! struct Store { id: String, name: String }
//!
//! #[derive(HasId, HasName)]
//! #[has_name(field = "display_name")]
//! struct User { id: String, display_name: String }
//!
//! let user = User { id: "u1".into(), display_name: "Ada".into() };
//! assert_eq!((user.get_id(), user.get_name()), ("u1".to_string(), "Ada".to_string()));
//! # let _ = Store { id: String::new(), name: String::new() };
//! ```
//!
//! Importing a derive imports its trait (same name), which the generated
//! impl needs in scope.

/// A record identified by a string id. Derive: the field `id`, or
/// `#[has_id(field = "…")]`.
pub trait HasId {
    fn get_id(&self) -> String;
}

/// A record with a display name. Derive: the field `name`, or
/// `#[has_name(field = "…")]`.
pub trait HasName {
    fn get_name(&self) -> String;
}

/// A record that may belong to a parent, referenced by id. Derive: the field
/// `parent_id` (an `Option<String>`), or `#[has_parent_id(field = "…")]`.
pub trait HasParentId {
    fn get_parent_id(&self) -> Option<String>;
}

#[cfg(feature = "derive")]
pub use ferrox_macros::{HasId, HasName, HasParentId};

#[cfg(all(test, feature = "derive"))]
mod tests {
    use super::*;

    #[derive(HasId, HasName, HasParentId)]
    struct Store {
        id: String,
        name: String,
        parent_id: Option<String>,
    }

    #[derive(HasId, HasName)]
    #[has_id(field = "email")]
    #[has_name(field = "display_name")]
    struct Invite {
        email: String,
        display_name: String,
    }

    #[test]
    fn derives_read_the_default_fields() {
        let s = Store { id: "s1".into(), name: "Pantry".into(), parent_id: Some("s0".into()) };
        assert_eq!(s.get_id(), "s1");
        assert_eq!(s.get_name(), "Pantry");
        assert_eq!(s.get_parent_id(), Some("s0".to_string()));
    }

    #[test]
    fn attributes_pick_other_fields() {
        let i = Invite { email: "a@b.c".into(), display_name: "Ada".into() };
        assert_eq!(i.get_id(), "a@b.c");
        assert_eq!(i.get_name(), "Ada");
    }

    #[test]
    fn traits_are_usable_without_the_derive() {
        struct Plain;
        impl HasId for Plain {
            fn get_id(&self) -> String { "p".into() }
        }
        assert_eq!(Plain.get_id(), "p");
    }
}
