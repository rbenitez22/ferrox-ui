//! Areas → JSON, so an app's API can serve the same topics to an assistant.
//! Written by hand: the crate stays std only.
//!
//! ```json
//! { "areas": [ { "title": "…", "intro": "…" | null, "topics": [
//!     { "id": "…", "title": "…",
//!       "blocks": [ {"paragraph": "…"}, {"list": ["…"]},
//!                   {"table": {"columns": ["…"], "rows": [["…"]]}} ],
//!       "related": ["…"], "guide": "/help#…" | null } ] } ] }
//! ```

use crate::model::{Area, Block, Topic};

/// The whole document, compact, one topic per line for readable diffs.
pub fn render_json(areas: &[Area], guide_prefix: Option<&str>) -> String {
    let areas: Vec<String> = areas.iter().map(|a| area(a, guide_prefix)).collect();
    format!("{{\"areas\":[\n{}\n]}}\n", areas.join(",\n"))
}

fn area(area: &Area, guide_prefix: Option<&str>) -> String {
    let topics: Vec<String> = area.topics.iter().map(|t| topic(t, guide_prefix)).collect();
    format!(
        "{{\"title\":{},\"intro\":{},\"topics\":[\n{}\n]}}",
        string(&area.title),
        optional(area.intro.as_deref()),
        topics.join(",\n")
    )
}

fn topic(topic: &Topic, guide_prefix: Option<&str>) -> String {
    let blocks: Vec<String> = topic.blocks.iter().map(block).collect();
    let related: Vec<&str> = topic.related.iter().map(|r| r.id.as_str()).collect();
    let guide = guide_prefix.map(|prefix| format!("{prefix}{}", topic.id));
    format!(
        "{{\"id\":{},\"title\":{},\"blocks\":[{}],\"related\":{},\"guide\":{}}}",
        string(&topic.id),
        string(&topic.title),
        blocks.join(","),
        strings(&related),
        optional(guide.as_deref())
    )
}

fn block(block: &Block) -> String {
    match block {
        Block::Paragraph(text) => format!("{{\"paragraph\":{}}}", string(text)),
        Block::List(items) => format!("{{\"list\":{}}}", strings(items)),
        Block::Table { columns, rows } => {
            let rows: Vec<String> = rows.iter().map(|r| strings(r)).collect();
            format!("{{\"table\":{{\"columns\":{},\"rows\":[{}]}}}}", strings(columns), rows.join(","))
        }
    }
}

fn optional(text: Option<&str>) -> String {
    text.map_or_else(|| "null".to_string(), string)
}

fn strings<S: AsRef<str>>(items: &[S]) -> String {
    let items: Vec<String> = items.iter().map(|s| string(s.as_ref())).collect();
    format!("[{}]", items.join(","))
}

/// A JSON string literal: quotes, backslashes and control characters escaped.
fn string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_area;

    #[test]
    fn escapes() {
        assert_eq!(string("a \"b\" \\ ½\t\u{1}"), "\"a \\\"b\\\" \\\\ ½\\t\\u0001\"");
    }

    #[test]
    fn renders_an_area() {
        let area = parse_area("a.md", "# A\n## T {#t}\nOne.\n\n- x\n\n| K | V |\n|---|---|\n| 1 | 2 |\n\nRelated: #u\n").unwrap();
        let json = render_json(&[area], Some("/help#"));
        assert_eq!(json, concat!(
            "{\"areas\":[\n",
            "{\"title\":\"A\",\"intro\":null,\"topics\":[\n",
            "{\"id\":\"t\",\"title\":\"T\",\"blocks\":[{\"paragraph\":\"One.\"},{\"list\":[\"x\"]},",
            "{\"table\":{\"columns\":[\"K\",\"V\"],\"rows\":[[\"1\",\"2\"]]}}],\"related\":[\"u\"],\"guide\":\"/help#t\"}\n",
            "]}\n",
            "]}\n",
        ));
    }
}
