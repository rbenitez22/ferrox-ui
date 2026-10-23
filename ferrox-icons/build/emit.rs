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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doc_comment_and_const() {
        assert_eq!(
            icon_const("chevron-down", "chevron-down", r#"<path d="m6 9 6 6 6-6"/>"#),
            "/// `chevron-down.svg`: Lucide `chevron-down` (ISC).\n\
             pub const CHEVRON_DOWN: IconData = IconData { name: \"chevron-down\", body: \"<path d=\\\"m6 9 6 6 6-6\\\"/>\" };\n",
        );
    }
}
