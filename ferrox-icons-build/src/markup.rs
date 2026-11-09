//! The icon's drawing as compact markup, for `inner_html`.

use roxmltree::Node;

/// The root's element children, serialised with whitespace and comments
/// dropped. Assumes the file passed `validate::check`.
pub fn body(root: Node) -> String {
    let mut out = String::new();
    for child in root.children().filter(Node::is_element) {
        write_element(child, &mut out);
    }
    out
}

fn write_element(node: Node, out: &mut String) {
    let tag = node.tag_name().name();
    out.push('<');
    out.push_str(tag);
    for attr in node.attributes() {
        out.push_str(&format!(" {}=\"{}\"", attr.name(), escape(attr.value())));
    }
    let children: Vec<Node> = node.children().filter(Node::is_element).collect();
    if children.is_empty() {
        out.push_str("/>");
        return;
    }
    out.push('>');
    for child in children {
        write_element(child, out);
    }
    out.push_str(&format!("</{tag}>"));
}

/// Escapes an attribute value for a double-quoted attribute.
fn escape(value: &str) -> String {
    value.replace('&', "&amp;").replace('"', "&quot;").replace('<', "&lt;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use roxmltree::Document;

    fn body_of(svg: &str) -> String {
        body(Document::parse(svg).unwrap().root_element())
    }

    #[test]
    fn compact_children_without_whitespace_or_comments() {
        let svg = "<svg viewBox=\"0 0 24 24\">\n  <!-- x -->\n  <path d=\"M18 6 6 18\" />\n  <path d=\"m6 6 12 12\" />\n</svg>";
        assert_eq!(body_of(svg), r#"<path d="M18 6 6 18"/><path d="m6 6 12 12"/>"#);
    }

    #[test]
    fn nested_groups() {
        let svg = r#"<svg><g transform="rotate(45)"><circle r="1"/></g></svg>"#;
        assert_eq!(body_of(svg), r#"<g transform="rotate(45)"><circle r="1"/></g>"#);
    }

    #[test]
    fn escapes_attribute_values() {
        assert_eq!(escape(r#"a&"<b"#), "a&amp;&quot;&lt;b");
    }
}
