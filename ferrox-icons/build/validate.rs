//! The rules an icon file must follow. They keep every icon in one style (a
//! 24×24 grid, 2px round strokes) and keep its markup static and safe to set
//! with `inner_html`: drawing elements only, no ids, classes, styles, scripts,
//! links or event handlers, and colour only through `currentColor`.

use roxmltree::{Attribute, Document, Node};

/// Attributes the root `<svg>` must carry, with these exact values. `Icon`
/// renders them itself, so they are checked here and not copied.
const ROOT_REQUIRED: &[(&str, &str)] = &[
    ("viewBox", "0 0 24 24"),
    ("fill", "none"),
    ("stroke", "currentColor"),
    ("stroke-width", "2"),
    ("stroke-linecap", "round"),
    ("stroke-linejoin", "round"),
];

/// Root attributes that are allowed and ignored (`Icon` sizes the icon at 1em).
const ROOT_IGNORED: &[&str] = &["width", "height"];

/// The drawing elements an icon may use.
const ELEMENTS: &[&str] = &["path", "circle", "rect", "line", "polyline", "polygon", "ellipse", "g"];

/// Geometry and presentation attributes allowed on drawing elements.
const ATTRIBUTES: &[&str] = &[
    "d", "cx", "cy", "r", "rx", "ry", "x", "y", "width", "height",
    "x1", "y1", "x2", "y2", "points", "transform",
    "fill", "stroke", "stroke-width", "stroke-linecap", "stroke-linejoin",
    "fill-rule", "clip-rule", "opacity", "fill-opacity", "stroke-opacity",
];

/// The only values `fill` and `stroke` may take, so every icon follows the
/// text colour of the theme.
const PAINT_VALUES: &[&str] = &["none", "currentColor"];

/// Checks an icon file; the error names the first rule it breaks.
pub fn check(doc: &Document) -> Result<(), String> {
    let root = doc.root_element();
    check_root(root)?;
    root.children().try_for_each(check_node)
}

fn check_root(root: Node) -> Result<(), String> {
    let tag = root.tag_name().name();
    if tag != "svg" {
        return Err(format!("the root element is <{tag}>, not <svg>"));
    }
    for attr in root.attributes() {
        check_root_attribute(attr)?;
    }
    match ROOT_REQUIRED.iter().find(|(name, _)| root.attribute(*name).is_none()) {
        Some((name, value)) => Err(format!("<svg> lacks {name}=\"{value}\"")),
        None => Ok(()),
    }
}

fn check_root_attribute(attr: Attribute) -> Result<(), String> {
    let name = attr.name();
    if attr.namespace().is_none() && ROOT_IGNORED.contains(&name) {
        return Ok(());
    }
    match ROOT_REQUIRED.iter().find(|(required, _)| *required == name) {
        Some(_) if attr.namespace().is_some() => Err(format!("attribute {name} on <svg> has a namespace")),
        Some((_, value)) if attr.value() == *value => Ok(()),
        Some((_, value)) => Err(format!("<svg {name}=\"{}\">: must be \"{value}\"", attr.value())),
        None => Err(format!("attribute {name} is not allowed on <svg>")),
    }
}

/// Text that is only whitespace, comments and processing instructions are
/// allowed (the generated markup drops them); elements must be drawing
/// elements with allowed attributes, all the way down.
fn check_node(node: Node) -> Result<(), String> {
    if node.is_text() {
        let text = node.text().unwrap_or_default();
        return if text.trim().is_empty() { Ok(()) } else { Err(format!("text `{}` is not allowed", text.trim())) };
    }
    if !node.is_element() {
        return Ok(());
    }
    let tag = node.tag_name().name();
    if !ELEMENTS.contains(&tag) {
        return Err(format!("element <{tag}> is not allowed"));
    }
    for attr in node.attributes() {
        check_attribute(tag, attr)?;
    }
    node.children().try_for_each(check_node)
}

fn check_attribute(tag: &str, attr: Attribute) -> Result<(), String> {
    let name = attr.name();
    if attr.namespace().is_some() || !ATTRIBUTES.contains(&name) {
        return Err(format!("attribute {name} is not allowed on <{tag}>"));
    }
    if (name == "fill" || name == "stroke") && !PAINT_VALUES.contains(&attr.value()) {
        return Err(format!("<{tag} {name}=\"{}\">: only none or currentColor", attr.value()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOT: &str = r#"xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round""#;

    fn check_svg(svg: &str) -> Result<(), String> {
        check(&Document::parse(svg).unwrap())
    }

    fn icon(body: &str) -> String {
        format!("<svg {ROOT}>{body}</svg>")
    }

    #[test]
    fn accepts_a_lucide_icon() {
        let svg = format!("<svg\n  {ROOT}\n>\n  <!-- x -->\n  <path d=\"M18 6 6 18\" />\n  <g><circle cx=\"12\" cy=\"12\" r=\"1\" fill=\"currentColor\" /></g>\n</svg>\n");
        assert_eq!(check_svg(&svg), Ok(()));
    }

    #[test]
    fn rejects_a_root_that_is_not_svg() {
        assert!(check_svg("<g/>").unwrap_err().contains("not <svg>"));
    }

    #[test]
    fn rejects_another_grid_or_stroke() {
        let other_grid = icon("").replace("0 0 24 24", "0 0 16 16");
        assert!(check_svg(&other_grid).unwrap_err().contains("viewBox"));
        let thin = icon("").replace("stroke-width=\"2\"", "stroke-width=\"1.5\"");
        assert!(check_svg(&thin).unwrap_err().contains("stroke-width"));
    }

    #[test]
    fn rejects_a_missing_root_attribute() {
        let svg = icon("").replace(" stroke-linejoin=\"round\"", "");
        assert!(check_svg(&svg).unwrap_err().contains("lacks stroke-linejoin"));
    }

    #[test]
    fn rejects_other_root_attributes() {
        let svg = icon("").replace("<svg ", "<svg class=\"lucide\" ");
        assert!(check_svg(&svg).unwrap_err().contains("class"));
    }

    #[test]
    fn rejects_unsafe_or_unknown_elements() {
        for body in ["<script>alert(1)</script>", "<style>*{}</style>", "<use href=\"#a\"/>", "<image/>", "<foreignObject/>", "<text>A</text>", "<g><script/></g>"] {
            assert!(check_svg(&icon(body)).unwrap_err().contains("not allowed"), "{body}");
        }
    }

    #[test]
    fn rejects_ids_classes_styles_handlers_and_links() {
        for attr in ["id=\"a\"", "class=\"a\"", "style=\"color:red\"", "onclick=\"x()\"", "href=\"#a\"", "xmlns:xlink=\"http://www.w3.org/1999/xlink\" xlink:href=\"#a\""] {
            let body = format!("<path d=\"M0 0\" {attr}/>");
            assert!(check_svg(&icon(&body)).is_err(), "{attr}");
        }
    }

    #[test]
    fn rejects_colours_other_than_current_color() {
        for attr in ["fill=\"red\"", "stroke=\"#000\"", "fill=\"url(#g)\""] {
            let body = format!("<path d=\"M0 0\" {attr}/>");
            assert!(check_svg(&icon(&body)).unwrap_err().contains("currentColor"), "{attr}");
        }
    }

    #[test]
    fn rejects_text() {
        assert!(check_svg(&icon("A")).unwrap_err().contains("text"));
    }
}
