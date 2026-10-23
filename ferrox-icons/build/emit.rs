//! The generated Rust source for one icon.

use super::naming::const_name;

/// `/// doc` + `pub const NAME: IconData = …;` for the icon in `<stem>.svg`,
/// drawn from Lucide's `<lucide>.svg`, with `body` as its markup.
pub fn icon_const(stem: &str, lucide: &str, body: &str) -> String {
    format!(
        "/// `{stem}.svg`: Lucide `{lucide}` (ISC).\npub const {}: IconData = IconData {{ name: {stem:?}, body: {body:?} }};\n",
        const_name(stem),
    )
}

/// `pub const ALL: &[IconData]`: every icon, in the order of `stems`.
pub fn all_table(stems: &[&str]) -> String {
    let names: Vec<String> = stems.iter().map(|s| const_name(s)).collect();
    format!(
        "/// Every icon, sorted by name: for galleries and tests. It keeps every\n\
         /// icon in the build, so apps name the icons they use instead.\n\
         pub const ALL: &[IconData] = &[{}];\n",
        names.join(", "),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_table_lists_consts_in_order() {
        assert!(all_table(&["chevron-down", "close"]).ends_with("pub const ALL: &[IconData] = &[CHEVRON_DOWN, CLOSE];\n"));
    }

    #[test]
    fn doc_comment_and_const() {
        assert_eq!(
            icon_const("chevron-down", "chevron-down", r#"<path d="m6 9 6 6 6-6"/>"#),
            "/// `chevron-down.svg`: Lucide `chevron-down` (ISC).\n\
             pub const CHEVRON_DOWN: IconData = IconData { name: \"chevron-down\", body: \"<path d=\\\"m6 9 6 6 6-6\\\"/>\" };\n",
        );
    }
}
