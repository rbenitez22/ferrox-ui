//! `icons/SOURCES`: which Lucide icon each of ours is.

use std::collections::BTreeMap;

/// Parses SOURCES: one `<our name> <Lucide name>` pair per line; blank lines
/// and lines starting with `#` are skipped. Returns our name → Lucide name.
pub fn parse(text: &str) -> Result<BTreeMap<String, String>, String> {
    let mut sources = BTreeMap::new();
    for (index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        let [ours, lucide] = fields[..] else {
            return Err(format!("line {}: expected `<our name> <Lucide name>`, got `{line}`", index + 1));
        };
        if sources.insert(ours.to_string(), lucide.to_string()).is_some() {
            return Err(format!("line {}: `{ours}` is listed twice", index + 1));
        }
    }
    Ok(sources)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_comments_and_blank_lines() {
        let sources = parse("# header\n\nclose x\n  delete   trash-2  \n").unwrap();
        assert_eq!(sources.len(), 2);
        assert_eq!(sources["close"], "x");
        assert_eq!(sources["delete"], "trash-2");
    }

    #[test]
    fn rejects_malformed_lines() {
        assert!(parse("close\n").unwrap_err().contains("line 1"));
        assert!(parse("close x extra\n").is_err());
    }

    #[test]
    fn rejects_duplicates() {
        assert!(parse("close x\nclose x-circle\n").unwrap_err().contains("listed twice"));
    }
}
