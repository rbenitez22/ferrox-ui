//! Icon names: the file stem is kebab-case, the const is SCREAMING_SNAKE_CASE.
//! Upper-case consts cannot be Rust keywords, and kebab-case stems cannot
//! clash once converted (`_` is not allowed in a stem).

/// Checks that `stem` is kebab-case: words of lowercase letters and digits,
/// joined by single hyphens, starting with a letter.
pub fn check_kebab(stem: &str) -> Result<(), String> {
    let starts_with_letter = stem.starts_with(|c: char| c.is_ascii_lowercase());
    let words_ok = stem.split('-').all(|w| !w.is_empty() && w.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    if starts_with_letter && words_ok {
        Ok(())
    } else {
        Err(format!("file name `{stem}` is not kebab-case (e.g. `chevron-down`)"))
    }
}

/// `chevron-down` → `CHEVRON_DOWN`.
pub fn const_name(stem: &str) -> String {
    stem.to_ascii_uppercase().replace('-', "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_kebab_case() {
        for stem in ["close", "chevron-down", "trash-2"] {
            assert_eq!(check_kebab(stem), Ok(()), "{stem}");
        }
    }

    #[test]
    fn rejects_anything_else() {
        for stem in ["", "Close", "chevron_down", "chevron--down", "-close", "close-", "2fa", "lock open"] {
            assert!(check_kebab(stem).is_err(), "{stem:?}");
        }
    }

    #[test]
    fn const_names() {
        assert_eq!(const_name("close"), "CLOSE");
        assert_eq!(const_name("chevron-down"), "CHEVRON_DOWN");
        assert_eq!(const_name("trash-2"), "TRASH_2");
    }
}
