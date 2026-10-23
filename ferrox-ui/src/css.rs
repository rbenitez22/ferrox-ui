//! Component CSS, injected as `<style>` elements by [`crate::init`].
//!
//! Only the groups compiled in are injected. Theme CSS (tokens, themes, fonts)
//! is not here: it is the stylesheet the app links in `index.html`
//! (ferrox-theme-build), so it applies before the WASM loads.

/// `(element id, css)` for `core`, in order: the focus ring before the
/// buttons, so a filled button's own outline colour wins.
const CORE: &[(&str, &str)] = &[
    ("ferrox-ui-a11y", include_str!("../style/core/a11y.css")),
    ("ferrox-ui-focus", include_str!("../style/core/focus.css")),
    ("ferrox-ui-buttons", include_str!("../style/core/buttons.css")),
    ("ferrox-ui-icon", include_str!("../style/core/icon.css")),
    ("ferrox-ui-toolbar", include_str!("../style/core/toolbar.css")),
    ("ferrox-ui-spinner", include_str!("../style/core/spinner.css")),
    ("ferrox-ui-alert", include_str!("../style/core/alert.css")),
];

/// Every stylesheet of the compiled groups, in injection order.
pub(crate) fn sheets() -> Vec<(&'static str, &'static str)> {
    CORE.to_vec()
}

/// Adds a `<style id="{id}">` to `<head>`. Does nothing if that id exists, so
/// calling [`crate::init`] twice injects nothing twice.
pub(crate) fn inject(id: &str, css: &str) {
    let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };
    if document.get_element_by_id(id).is_some() {
        return;
    }
    let Ok(style) = document.create_element("style") else { return };
    style.set_id(id);
    style.set_text_content(Some(css));
    if let Some(head) = document.head() {
        let _ = head.append_child(&style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The class-only rule: no selector may start with an element name or a
    /// bare pseudo-class (`button {`, `:focus-visible {`). Checked on every
    /// shipped sheet so a new rule cannot slip it.
    #[test]
    fn every_selector_starts_with_a_class() {
        for (id, css) in sheets() {
            let mut rest = strip_comments(css);
            while let Some(brace) = rest.find('{') {
                let prelude = rest[..brace].rsplit(['}', ';']).next().unwrap_or("").trim().to_string();
                let close = matching_brace(&rest, brace);
                if prelude.starts_with("@keyframes") {
                    rest = rest[close + 1..].to_string();
                    continue;
                }
                if !prelude.starts_with('@') {
                    for selector in prelude.split(',').map(str::trim) {
                        assert!(selector.starts_with('.'), "{id}: selector `{selector}` does not start with a class");
                    }
                    rest = rest[close + 1..].to_string();
                } else {
                    rest = rest[brace + 1..].to_string();
                }
            }
        }
    }

    fn strip_comments(css: &str) -> String {
        let mut out = String::new();
        let mut rest = css;
        while let Some(start) = rest.find("/*") {
            out.push_str(&rest[..start]);
            rest = rest[start + 2..].split_once("*/").map_or("", |(_, after)| after);
        }
        out.push_str(rest);
        out
    }

    fn matching_brace(css: &str, open: usize) -> usize {
        let mut depth = 0;
        for (i, c) in css[open..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return open + i;
                    }
                }
                _ => {}
            }
        }
        css.len() - 1
    }
}
