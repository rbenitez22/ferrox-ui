//! One area file → an [`Area`]. Line-based: the subset is small enough that
//! every construct is recognised by how its line starts.

use crate::error::HelpError;
use crate::model::{Area, Block, Related, Topic};

/// A line of the file, numbered from 1, after comments are blanked.
type Line = (usize, String);

/// What a run of lines turned out to be.
enum Item {
    Area(usize, String),
    Topic(usize, String),
    TooDeep(usize),
    Chunk(Vec<Line>),
}

/// Parses one area file. Every problem found is returned, not just the first.
pub fn parse_area(file: &str, text: &str) -> Result<Area, Vec<HelpError>> {
    let mut errors = Vec::new();
    let lines = blank_comments(file, text, &mut errors);
    let items = items(lines);
    let area = build_area(file, items, &mut errors);
    if errors.is_empty() { Ok(area) } else { Err(errors) }
}

/// Is `id` a valid topic id: `^[a-z][a-z0-9-]*$`.
pub fn is_valid_id(id: &str) -> bool {
    let mut chars = id.chars();
    matches!(chars.next(), Some('a'..='z'))
        && chars.all(|c| matches!(c, 'a'..='z' | '0'..='9' | '-'))
}

/// Replaces `<!-- … -->` comment lines with blank ones, keeping numbering.
fn blank_comments(file: &str, text: &str, errors: &mut Vec<HelpError>) -> Vec<Line> {
    let mut out = Vec::new();
    let mut open_at: Option<usize> = None;
    for (index, raw) in text.lines().enumerate() {
        let number = index + 1;
        let trimmed = raw.trim();
        if open_at.is_none() && trimmed.starts_with("<!--") {
            open_at = Some(number);
        }
        if open_at.is_some() {
            if trimmed.contains("-->") {
                if !trimmed.ends_with("-->") {
                    errors.push(HelpError::new(file, number, "text after a comment's closing -->; put comments on lines of their own"));
                }
                open_at = None;
            }
            out.push((number, String::new()));
        } else {
            out.push((number, raw.trim_end().to_string()));
        }
    }
    if let Some(start) = open_at {
        errors.push(HelpError::new(file, start, "comment is never closed with -->"));
    }
    out
}

/// Splits lines into headings and blank-separated chunks.
fn items(lines: Vec<Line>) -> Vec<Item> {
    let mut items = Vec::new();
    let mut chunk: Vec<Line> = Vec::new();
    let flush = |chunk: &mut Vec<Line>, items: &mut Vec<Item>| {
        if !chunk.is_empty() {
            items.push(Item::Chunk(std::mem::take(chunk)));
        }
    };
    for (number, text) in lines {
        if text.trim().is_empty() {
            flush(&mut chunk, &mut items);
        } else if text.starts_with("### ") || text.starts_with("####") {
            flush(&mut chunk, &mut items);
            items.push(Item::TooDeep(number));
        } else if let Some(rest) = text.strip_prefix("## ") {
            flush(&mut chunk, &mut items);
            items.push(Item::Topic(number, rest.trim().to_string()));
        } else if let Some(rest) = text.strip_prefix("# ") {
            flush(&mut chunk, &mut items);
            items.push(Item::Area(number, rest.trim().to_string()));
        } else {
            chunk.push((number, text));
        }
    }
    flush(&mut chunk, &mut items);
    items
}

/// Assembles the area from its items: heading, intro, then topics.
fn build_area(file: &str, items: Vec<Item>, errors: &mut Vec<HelpError>) -> Area {
    let mut area = Area { file: file.to_string(), title: String::new(), intro: None, topics: Vec::new() };
    let mut topic: Option<(Topic, Vec<Vec<Line>>)> = None;
    let mut intro_chunks: Vec<Vec<Line>> = Vec::new();

    for item in items {
        match item {
            Item::Area(number, title) => {
                if !area.title.is_empty() {
                    errors.push(HelpError::new(file, number, "a second # heading; each file has exactly one"));
                } else if topic.is_some() || !intro_chunks.is_empty() {
                    errors.push(HelpError::new(file, number, "the # heading must come first"));
                } else {
                    area.title = title;
                }
            }
            Item::Topic(number, heading) => {
                if let Some((done, chunks)) = topic.take() {
                    area.topics.push(finish_topic(file, done, chunks, errors));
                }
                topic = Some((start_topic(file, number, &heading, errors), Vec::new()));
            }
            Item::TooDeep(number) => {
                errors.push(HelpError::new(file, number, "### and deeper headings aren't supported; split the topic into two topics"));
            }
            Item::Chunk(lines) => match topic.as_mut() {
                Some((_, chunks)) => chunks.push(lines),
                None if area.title.is_empty() => {
                    errors.push(HelpError::new(file, lines[0].0, "text before the # heading"));
                }
                None => intro_chunks.push(lines),
            },
        }
    }
    if let Some((done, chunks)) = topic.take() {
        area.topics.push(finish_topic(file, done, chunks, errors));
    }
    if area.title.is_empty() {
        errors.push(HelpError::new(file, 0, "no # heading; each file starts with one"));
    }
    area.intro = intro(file, intro_chunks, errors);
    area
}

/// The area's optional intro: at most one paragraph.
fn intro(file: &str, chunks: Vec<Vec<Line>>, errors: &mut Vec<HelpError>) -> Option<String> {
    let mut chunks = chunks.into_iter();
    let first = chunks.next()?;
    for extra in chunks {
        errors.push(HelpError::new(file, extra[0].0, "an area intro is a single paragraph"));
    }
    match block(file, &first, errors) {
        Some(Block::Paragraph(text)) => Some(text),
        _ => {
            errors.push(HelpError::new(file, first[0].0, "an area intro must be a paragraph"));
            None
        }
    }
}

/// Reads `Title {#id}` from a topic heading.
fn start_topic(file: &str, number: usize, heading: &str, errors: &mut Vec<HelpError>) -> Topic {
    let parsed = heading
        .strip_suffix('}')
        .and_then(|h| h.rsplit_once(" {#"))
        .map(|(title, id)| (title.trim().to_string(), id.to_string()));
    let (title, id) = parsed.unwrap_or_else(|| {
        errors.push(HelpError::new(file, number, "a topic heading is `## Title {#topic-id}`"));
        (heading.to_string(), String::new())
    });
    if !id.is_empty() && !is_valid_id(&id) {
        errors.push(HelpError::new(file, number, format!("invalid id `{id}`: use lowercase letters, digits and hyphens, starting with a letter")));
    }
    if title.is_empty() {
        errors.push(HelpError::new(file, number, "a topic needs a title before {#id}"));
    }
    check_inline(file, number, &title, errors);
    Topic { id, title, blocks: Vec::new(), related: Vec::new(), line: number }
}

/// Turns a topic's chunks into blocks and its `Related:` line.
fn finish_topic(file: &str, mut topic: Topic, chunks: Vec<Vec<Line>>, errors: &mut Vec<HelpError>) -> Topic {
    let count = chunks.len();
    let mut failed = false;
    for (index, chunk) in chunks.iter().enumerate() {
        let (number, first) = &chunk[0];
        if let Some(rest) = first.strip_prefix("Related:") {
            if index + 1 != count {
                errors.push(HelpError::new(file, *number, "Related: must be the topic's last line"));
            }
            if chunk.len() > 1 {
                errors.push(HelpError::new(file, chunk[1].0, "Related: is a single line"));
            }
            topic.related = related(file, *number, rest, errors);
        } else if let Some(block) = block(file, chunk, errors) {
            topic.blocks.push(block);
        } else {
            failed = true;
        }
    }
    if failed {
        return topic; // the broken block is reported; its position is unknown
    }
    match topic.blocks.first() {
        None => errors.push(HelpError::new(file, topic.line, format!("topic `{}` has no content", topic.id))),
        Some(Block::Paragraph(_)) => {}
        Some(_) => errors.push(HelpError::new(file, topic.line, format!(
            "topic `{}` must start with a paragraph that stands on its own", topic.id
        ))),
    }
    topic
}

/// Reads `#a, #b` after `Related:`.
fn related(file: &str, number: usize, rest: &str, errors: &mut Vec<HelpError>) -> Vec<Related> {
    let mut out = Vec::new();
    for part in rest.split(',') {
        let part = part.trim();
        match part.strip_prefix('#') {
            Some(id) if is_valid_id(id) => out.push(Related { id: id.to_string(), line: number }),
            _ => errors.push(HelpError::new(file, number, format!("`{part}` in Related: is not a #topic-id"))),
        }
    }
    out
}

/// One blank-separated chunk → a paragraph, a list or a table.
fn block(file: &str, chunk: &[Line], errors: &mut Vec<HelpError>) -> Option<Block> {
    for (number, text) in chunk {
        check_inline(file, *number, text, errors);
    }
    let first = &chunk[0].1;
    if first.starts_with("- ") {
        list(file, chunk, errors).map(Block::List)
    } else if first.starts_with('|') {
        table(file, chunk, errors)
    } else {
        paragraph(file, chunk, errors).map(Block::Paragraph)
    }
}

/// Lines joined with single spaces. A list or table line inside is a mistake.
fn paragraph(file: &str, chunk: &[Line], errors: &mut Vec<HelpError>) -> Option<String> {
    let mut ok = true;
    for (number, text) in &chunk[1..] {
        if text.starts_with("- ") || text.starts_with('|') {
            errors.push(HelpError::new(file, *number, "a list or table must be separated from the paragraph above by a blank line"));
            ok = false;
        }
    }
    ok.then(|| chunk.iter().map(|(_, t)| t.trim()).collect::<Vec<_>>().join(" "))
}

/// `- ` items; lines indented by two spaces continue the item above.
fn list(file: &str, chunk: &[Line], errors: &mut Vec<HelpError>) -> Option<Vec<String>> {
    let mut items: Vec<String> = Vec::new();
    let mut ok = true;
    for (number, text) in chunk {
        if let Some(item) = text.strip_prefix("- ") {
            items.push(item.trim().to_string());
        } else if text.starts_with("  ") && !text.trim_start().starts_with("- ") {
            let last = items.last_mut().expect("a list chunk starts with an item");
            last.push(' ');
            last.push_str(text.trim());
        } else if text.trim_start().starts_with("- ") {
            errors.push(HelpError::new(file, *number, "lists are one level deep"));
            ok = false;
        } else {
            errors.push(HelpError::new(file, *number, "a list line starts with `- `, or two spaces to continue the item above"));
            ok = false;
        }
    }
    ok.then_some(items)
}

/// A pipe table: header row, separator row, then at least one row.
fn table(file: &str, chunk: &[Line], errors: &mut Vec<HelpError>) -> Option<Block> {
    let start = chunk[0].0;
    let mut rows: Vec<(usize, Vec<String>)> = Vec::new();
    for (number, text) in chunk {
        match cells(text) {
            Some(cells) => rows.push((*number, cells)),
            None => {
                errors.push(HelpError::new(file, *number, "a table line starts and ends with `|`"));
                return None;
            }
        }
    }
    if rows.len() < 3 {
        errors.push(HelpError::new(file, start, "a table needs a header row, a separator row and at least one row"));
        return None;
    }
    let (_, columns) = rows.remove(0);
    let (sep_line, separator) = rows.remove(0);
    let is_separator = |c: &String| {
        let core = c.trim_start_matches(':').trim_end_matches(':');
        !core.is_empty() && core.chars().all(|ch| ch == '-')
    };
    if separator.len() != columns.len() || !separator.iter().all(is_separator) {
        errors.push(HelpError::new(file, sep_line, "the second table line is the separator: `|---|---|`, one cell per column"));
        return None;
    }
    let mut ok = true;
    for (number, row) in &rows {
        if row.len() != columns.len() {
            errors.push(HelpError::new(file, *number, format!("this row has {} cells; the header has {}", row.len(), columns.len())));
            ok = false;
        }
    }
    ok.then(|| Block::Table { columns, rows: rows.into_iter().map(|(_, r)| r).collect() })
}

/// `| a | b |` → `["a", "b"]`; `None` if the line isn't framed by pipes.
fn cells(line: &str) -> Option<Vec<String>> {
    let inner = line.trim().strip_prefix('|')?.strip_suffix('|')?;
    Some(inner.split('|').map(|c| c.trim().to_string()).collect())
}

/// Text is plain: reject the inline Markdown and HTML a writer might reach for.
fn check_inline(file: &str, number: usize, text: &str, errors: &mut Vec<HelpError>) {
    let found = if text.contains("**") {
        Some("`**`")
    } else if text.contains('`') {
        Some("a backtick")
    } else if text.contains("](") {
        Some("a Markdown link")
    } else if looks_like_tag(text) {
        Some("an HTML tag")
    } else {
        None
    };
    if let Some(what) = found {
        errors.push(HelpError::new(file, number, format!("{what}: help text is plain, with no inline formatting")));
    }
}

/// `<` followed by a letter, `/` or `!`.
fn looks_like_tag(text: &str) -> bool {
    text.as_bytes()
        .windows(2)
        .any(|w| w[0] == b'<' && (w[1].is_ascii_alphabetic() || w[1] == b'/' || w[1] == b'!'))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(text: &str) -> Area {
        parse_area("a.md", text).unwrap_or_else(|e| panic!("{e:?}"))
    }

    fn errs(text: &str) -> Vec<String> {
        parse_area("a.md", text).expect_err("should fail").iter().map(ToString::to_string).collect()
    }

    #[test]
    fn parses_every_block_kind() {
        let area = ok("# Area\n\nIntro\nline.\n\n## One {#one}\n\nFirst\nparagraph.\n\n- a\n  more a\n- b\n\n| X | Y |\n|---|:-:|\n| 1 | |\n\nRelated: #two, #three\n");
        assert_eq!(area.title, "Area");
        assert_eq!(area.intro.as_deref(), Some("Intro line."));
        let topic = &area.topics[0];
        assert_eq!((topic.id.as_str(), topic.title.as_str(), topic.line), ("one", "One", 6));
        assert_eq!(topic.blocks, vec![
            Block::Paragraph("First paragraph.".into()),
            Block::List(vec!["a more a".into(), "b".into()]),
            Block::Table { columns: vec!["X".into(), "Y".into()], rows: vec![vec!["1".into(), "".into()]] },
        ]);
        let ids: Vec<_> = topic.related.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["two", "three"]);
    }

    #[test]
    fn comments_are_dropped_and_lines_keep_their_numbers() {
        let area = ok("# A\n<!-- one\nline -->\n## T {#t}\n<!-- note -->\nText.\n");
        assert_eq!(area.topics[0].line, 4);
        assert_eq!(area.topics[0].blocks, vec![Block::Paragraph("Text.".into())]);
    }

    #[test]
    fn ids() {
        assert!(is_valid_id("recipe-search2"));
        assert!(!is_valid_id("Recipe"));
        assert!(!is_valid_id("2a"));
        assert!(!is_valid_id("a_b"));
        assert!(!is_valid_id(""));
    }

    #[test]
    fn heading_problems() {
        assert_eq!(errs("## T {#t}\n\nx\n"), ["a.md: no # heading; each file starts with one"]);
        assert_eq!(errs("# A\n# B\n"), ["a.md:2: a second # heading; each file has exactly one"]);
        assert_eq!(errs("# A\n## T\n\nx\n"), ["a.md:2: a topic heading is `## Title {#topic-id}`"]);
        assert_eq!(errs("# A\n## T {#Bad}\n\nx\n")[0], "a.md:2: invalid id `Bad`: use lowercase letters, digits and hyphens, starting with a letter");
        assert_eq!(errs("# A\n## T {#t}\n\nx\n\n### Sub\n"), ["a.md:6: ### and deeper headings aren't supported; split the topic into two topics"]);
        assert_eq!(errs("Hi\n# A\n"), ["a.md:1: text before the # heading"]);
    }

    #[test]
    fn topic_problems() {
        assert_eq!(errs("# A\n## T {#t}\n"), ["a.md:2: topic `t` has no content"]);
        assert_eq!(errs("# A\n## T {#t}\n- a\n"), ["a.md:2: topic `t` must start with a paragraph that stands on its own"]);
        assert_eq!(errs("# A\n## T {#t}\nx\n\nRelated: #a\n\ny\n"), ["a.md:5: Related: must be the topic's last line"]);
        assert_eq!(errs("# A\n## T {#t}\nx\n\nRelated: a\n"), ["a.md:5: `a` in Related: is not a #topic-id"]);
    }

    #[test]
    fn block_problems() {
        assert_eq!(errs("# A\n## T {#t}\nx\n- y\n"), ["a.md:4: a list or table must be separated from the paragraph above by a blank line"]);
        assert_eq!(errs("# A\n## T {#t}\nx\n\n- a\n  - b\n"), ["a.md:6: lists are one level deep"]);
        assert_eq!(errs("# A\n## T {#t}\nx\n\n- a\nb\n"), ["a.md:6: a list line starts with `- `, or two spaces to continue the item above"]);
        assert_eq!(errs("# A\n## T {#t}\nx\n\n| a | b |\n|---|---|\n| 1 |\n"), ["a.md:7: this row has 1 cells; the header has 2"]);
        assert_eq!(errs("# A\n## T {#t}\nx\n\n| a | b |\n| 1 | 2 |\n| 3 | 4 |\n"), ["a.md:6: the second table line is the separator: `|---|---|`, one cell per column"]);
        assert_eq!(errs("# A\n## T {#t}\nx\n\n| a |\n|---|\n"), ["a.md:5: a table needs a header row, a separator row and at least one row"]);
    }

    #[test]
    fn inline_markup_is_rejected() {
        for (text, what) in [("**bold**", "`**`"), ("a `b`", "a backtick"), ("[x](y)", "a Markdown link"), ("<b>x</b>", "an HTML tag"), ("x <!-- y -->", "an HTML tag")] {
            let got = errs(&format!("# A\n## T {{#t}}\n{text}\n"));
            assert_eq!(got, [format!("a.md:3: {what}: help text is plain, with no inline formatting")]);
        }
        ok("# A\n## T {#t}\n1 < 2 and 3 > 2, ½ cup — ✓ \"Make It\".\n");
    }

    #[test]
    fn unclosed_comment() {
        assert_eq!(errs("# A\n<!-- open\n## T {#t}\nx\n"), ["a.md:2: comment is never closed with -->"]);
    }
}
