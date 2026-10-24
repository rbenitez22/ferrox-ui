//! Moving through a menu's items from the keyboard, as pure functions.

/// The item to focus after an arrow, Home or End, from item `at` of `len`.
/// ↑ / ↓ wrap at the ends; from no item, ↓ goes to the first and ↑ to the
/// last. Any other key moves nothing.
pub(crate) fn move_to(at: Option<usize>, len: usize, key: &str) -> Option<usize> {
    if len == 0 {
        return None;
    }
    match key {
        "ArrowDown" => Some(at.map_or(0, |i| (i + 1) % len)),
        "ArrowUp" => Some(at.map_or(len - 1, |i| (i + len - 1) % len)),
        "Home" => Some(0),
        "End" => Some(len - 1),
        _ => None,
    }
}

/// The next item after `at` whose text starts with `ch` (case-insensitive),
/// wrapping round; `None` when no item does.
pub(crate) fn type_ahead(labels: &[String], at: Option<usize>, ch: char) -> Option<usize> {
    let len = labels.len();
    let wanted: String = ch.to_lowercase().collect();
    let start = at.map_or(0, |i| i + 1);
    (0..len)
        .map(|offset| (start + offset) % len)
        .find(|&i| labels[i].trim().to_lowercase().starts_with(&wanted))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_wrap_and_home_end_jump() {
        assert_eq!(move_to(None, 3, "ArrowDown"), Some(0));
        assert_eq!(move_to(None, 3, "ArrowUp"), Some(2));
        assert_eq!(move_to(Some(2), 3, "ArrowDown"), Some(0));
        assert_eq!(move_to(Some(0), 3, "ArrowUp"), Some(2));
        assert_eq!(move_to(Some(1), 3, "Home"), Some(0));
        assert_eq!(move_to(Some(1), 3, "End"), Some(2));
        assert_eq!(move_to(Some(1), 3, "a"), None);
        assert_eq!(move_to(None, 0, "ArrowDown"), None);
    }

    #[test]
    fn a_letter_finds_the_next_item_starting_with_it() {
        let labels = ["Edit", "Duplicate", "Delete", "Archive"].map(String::from);
        assert_eq!(type_ahead(&labels, None, 'd'), Some(1));
        assert_eq!(type_ahead(&labels, Some(1), 'D'), Some(2));
        assert_eq!(type_ahead(&labels, Some(2), 'd'), Some(1));
        assert_eq!(type_ahead(&labels, Some(0), 'x'), None);
    }
}
