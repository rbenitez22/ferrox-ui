//! Checks across files: ids unique everywhere, related ids that exist.

use std::collections::HashMap;

use crate::error::HelpError;
use crate::model::Area;

/// Ids the generated module already uses for its own items.
pub const RESERVED_IDS: &[&str] = &["topics"];

/// Every cross-file problem in `areas`.
pub fn check_areas(areas: &[Area]) -> Vec<HelpError> {
    let mut errors = Vec::new();
    let mut seen: HashMap<&str, (&str, usize)> = HashMap::new();
    for area in areas {
        for topic in &area.topics {
            if topic.id.is_empty() {
                continue; // already reported by the parser
            }
            if RESERVED_IDS.contains(&topic.id.as_str()) {
                errors.push(HelpError::new(&area.file, topic.line, format!("`{}` is reserved; choose another id", topic.id)));
            }
            if let Some((file, line)) = seen.insert(&topic.id, (&area.file, topic.line)) {
                errors.push(HelpError::new(&area.file, topic.line, format!(
                    "duplicate id `{}` (first at {file}:{line}); ids are unique across all files", topic.id
                )));
            }
        }
    }
    for area in areas {
        for topic in &area.topics {
            for related in &topic.related {
                if related.id == topic.id {
                    errors.push(HelpError::new(&area.file, related.line, format!("`{}` lists itself in Related:", topic.id)));
                } else if !seen.contains_key(related.id.as_str()) {
                    errors.push(HelpError::new(&area.file, related.line, format!("Related: #{} names no topic", related.id)));
                }
            }
        }
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_area;

    fn area(file: &str, text: &str) -> Area {
        parse_area(file, text).unwrap()
    }

    fn messages(areas: &[Area]) -> Vec<String> {
        check_areas(areas).iter().map(ToString::to_string).collect()
    }

    #[test]
    fn clean() {
        let a = area("a.md", "# A\n## One {#one}\nx\n\nRelated: #two\n");
        let b = area("b.md", "# B\n## Two {#two}\ny\n\nRelated: #one\n");
        assert!(messages(&[a, b]).is_empty());
    }

    #[test]
    fn duplicate_across_files() {
        let a = area("a.md", "# A\n## One {#one}\nx\n");
        let b = area("b.md", "# B\n\n## Again {#one}\ny\n");
        assert_eq!(messages(&[a, b]), ["b.md:3: duplicate id `one` (first at a.md:2); ids are unique across all files"]);
    }

    #[test]
    fn related_problems() {
        let a = area("a.md", "# A\n## One {#one}\nx\n\nRelated: #one, #missing\n");
        assert_eq!(messages(&[a]), [
            "a.md:5: `one` lists itself in Related:",
            "a.md:5: Related: #missing names no topic",
        ]);
    }

    #[test]
    fn reserved() {
        let a = area("a.md", "# A\n## All {#topics}\nx\n");
        assert_eq!(messages(&[a]), ["a.md:2: `topics` is reserved; choose another id"]);
    }
}
