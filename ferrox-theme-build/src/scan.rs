//! Small CSS scanners: enough to check the theme contract, not a CSS parser.

/// Comment bodies, in order.
pub(crate) fn comments(css: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        let after = &rest[start + 2..];
        let end = after.find("*/").unwrap_or(after.len());
        found.push(&after[..end]);
        rest = after.get(end + 2..).unwrap_or("");
    }
    found
}

/// The CSS with comments removed.
pub(crate) fn strip_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        rest = after.find("*/").map_or("", |end| &after[end + 2..]);
    }
    out.push_str(rest);
    out
}

/// The text before every `{` that opens a style rule (at-rule preludes such as
/// `@media …` are skipped, but the rules nested inside them are included).
pub(crate) fn rule_selectors(css: &str) -> Vec<String> {
    css.split('{')
        .take(css.matches('{').count())
        .map(|chunk| {
            let prelude = chunk.rsplit(['}', ';']).next().unwrap_or(chunk);
            prelude.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .filter(|prelude| !prelude.starts_with('@'))
        .collect()
}

/// Every custom-property name declared (`--name:`) anywhere in the CSS.
pub(crate) fn declared_tokens(css: &str) -> Vec<&str> {
    css.split(['{', ';', '}'])
        .filter_map(|decl| {
            let (name, _) = decl.split_once(':')?;
            let name = name.trim();
            name.starts_with("--").then_some(name)
        })
        .collect()
}

/// The text following `marker` in the first comment that has one.
pub(crate) fn comment_value(css: &str, marker: &str) -> Option<String> {
    comments(css).into_iter().find_map(|body| {
        let after = &body[body.find(marker)? + marker.len()..];
        let value = after.lines().next().unwrap_or("").trim();
        (!value.is_empty()).then(|| value.to_string())
    })
}
