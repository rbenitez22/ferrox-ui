//! Page-unique element ids, so two instances of a component never share one
//! (a dialog's title, a field's control and its message).

use std::cell::Cell;

thread_local! {
    /// The last number handed out by `unique_id`.
    static LAST_ID: Cell<usize> = const { Cell::new(0) };
}

/// A page-unique id, `{prefix}-{n}`.
pub(crate) fn unique_id(prefix: &str) -> String {
    let n = LAST_ID.with(|last| {
        last.set(last.get() + 1);
        last.get()
    });
    format!("{prefix}-{n}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_ids_differ() {
        let a = unique_id("ferrox-dialog-title");
        let b = unique_id("ferrox-dialog-title");
        assert_ne!(a, b);
        assert!(a.starts_with("ferrox-dialog-title-"));
    }
}
