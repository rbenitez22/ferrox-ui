//! The docs are current.
//!
//! - `docs/index.md` is generated from the source: every public component
//!   with its props, and every other public item, with the first line of
//!   its doc comment.
//! - Every Rust example in `docs/` is a compiled region of
//!   `tests/doc_examples.rs`, copied between `<!-- example: <name> -->` and
//!   `<!-- /example -->`. A Rust block without the markers, a marker naming
//!   no region, and a region no page shows all fail.
//!
//! - The agent kit's rendered blocks (`<!-- rendered: <name> -->`, in
//!   `agent-kit/core/AGENTS.md` and `agent-kit/README.md`) show `kit.tsv`,
//!   `check/rules.tsv` and `docs/choosing.md`.
//!
//! When the docs differ from what they're made from, the test fails; run it
//! with `FERROX_BLESS=1` to rewrite them instead.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use quote::ToTokens;
use syn::{Attribute, Expr, FnArg, Item, Lit, Meta, Pat, Visibility};

/// The source groups, in the order the index lists them.
const GROUPS: &[(&str, &str)] = &[
    ("core", "always on"),
    ("overlay", "feature `overlay`"),
    ("forms", "feature `forms`"),
    ("navigation", "feature `navigation`"),
    ("layout", "feature `layout`"),
    ("data", "feature `data`"),
    ("menus", "feature `menus`"),
];

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn the_index_matches_the_source() {
    check_or_bless(&crate_dir().join("docs/index.md"), &render_index());
}

#[test]
fn every_example_is_its_compiled_region() {
    let regions = regions(&fs::read_to_string(crate_dir().join("tests/doc_examples.rs")).unwrap());
    let mut shown = Vec::new();
    for page in doc_pages() {
        let text = fs::read_to_string(&page).unwrap();
        let (rendered, names) = render_examples(&text, &regions)
            .unwrap_or_else(|e| panic!("{}: {e}", page.display()));
        shown.extend(names);
        check_or_bless(&page, &rendered);
    }
    let unused: Vec<&String> = regions.keys().filter(|name| !shown.contains(name)).collect();
    assert!(unused.is_empty(), "regions no page shows: {unused:?}");
}

#[test]
fn the_kit_shows_its_sources() {
    let kit = crate_dir().join("agent-kit");
    let blocks = BTreeMap::from([
        ("tasks".to_string(), render_tasks(&kit)),
        ("rules".to_string(), render_rules(&kit)),
        ("choosing".to_string(), render_choosing()),
        ("kit".to_string(), render_kit_table(&kit)),
    ]);
    for page in [kit.join("core/AGENTS.md"), kit.join("README.md")] {
        let text = fs::read_to_string(&page).unwrap_or_else(|e| panic!("{}: {e}", page.display()));
        let rendered = render_blocks(&text, &blocks).unwrap_or_else(|e| panic!("{}: {e}", page.display()));
        check_or_bless(&page, &rendered);
    }
}

/// `kit.tsv`'s lines: kind, id, path, purpose.
fn kit_rows(kit: &Path) -> Vec<Vec<String>> {
    fs::read_to_string(kit.join("kit.tsv"))
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.split('\t').map(str::to_string).collect())
        .collect()
}

fn render_tasks(kit: &Path) -> String {
    kit_rows(kit)
        .iter()
        .filter(|row| row[0] == "task")
        .map(|row| format!("- `{}`: {} (`{{{{KIT}}}}/{}`)", row[1], row[3], row[2]))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_kit_table(kit: &Path) -> String {
    let mut out = vec!["| Kind | Piece | File | What it's for |".to_string(), "|---|---|---|---|".to_string()];
    for row in kit_rows(kit) {
        out.push(format!("| {} | `{}` | [`{}`]({}) | {} |", row[0], row[1], row[2], row[2], row[3]));
    }
    out.join("\n")
}

fn render_rules(kit: &Path) -> String {
    fs::read_to_string(kit.join("check/rules.tsv"))
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            format!("- `{}`: {}. Instead: {}.", f[0], f[5], f[6])
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// `docs/choosing.md`'s tables, one level down (its `## group`s become `###`).
fn render_choosing() -> String {
    let text = fs::read_to_string(crate_dir().join("docs/choosing.md")).unwrap();
    let from = text.find("\n## ").map(|i| i + 1).unwrap_or(0);
    text[from..].lines().map(|l| if l.starts_with("## ") { format!("#{l}") } else { l.to_string() }).collect::<Vec<_>>().join("\n").trim().to_string()
}

/// Fills each `<!-- rendered: <name> -->` … `<!-- /rendered -->` block,
/// indented as its opening marker is.
fn render_blocks(text: &str, blocks: &BTreeMap<String, String>) -> Result<String, String> {
    const OPEN: &str = "<!-- rendered: ";
    const CLOSE: &str = "<!-- /rendered -->";
    let mut out = String::new();
    let mut rest = text;
    while let Some(at) = rest.find(OPEN) {
        let line_start = rest[..at].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let indent = &rest[line_start..at];
        out.push_str(&rest[..at]);
        let after = &rest[at + OPEN.len()..];
        let name_end = after.find(" -->").ok_or("an unclosed rendered marker")?;
        let name = after[..name_end].trim();
        let close = after.find(CLOSE).ok_or(format!("block {name} has no {CLOSE}"))?;
        let content = blocks.get(name).ok_or(format!("no source for the rendered block {name}"))?;
        let body: Vec<String> = content.lines().map(|l| if l.is_empty() { String::new() } else { format!("{indent}{l}") }).collect();
        out.push_str(&format!("{OPEN}{name} -->\n{}\n{indent}{CLOSE}", body.join("\n")));
        rest = &after[close + CLOSE.len()..];
    }
    out.push_str(rest);
    Ok(out)
}

/// Every `docs/*.md` page, sorted.
fn doc_pages() -> Vec<PathBuf> {
    let mut pages: Vec<PathBuf> = fs::read_dir(crate_dir().join("docs"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "md"))
        .collect();
    pages.sort();
    pages
}

/// `// region: <name>` … `// endregion` blocks, dedented.
fn regions(source: &str) -> BTreeMap<String, String> {
    let mut found = BTreeMap::new();
    let mut open: Option<(String, Vec<&str>)> = None;
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(name) = trimmed.strip_prefix("// region: ") {
            open = Some((name.trim().to_string(), Vec::new()));
        } else if trimmed == "// endregion" {
            let (name, lines) = open.take().expect("an endregion without a region");
            assert!(found.insert(name.clone(), dedent(&lines)).is_none(), "region {name} twice");
        } else if let Some((_, lines)) = open.as_mut() {
            lines.push(line);
        }
    }
    assert!(open.is_none(), "a region without an endregion");
    found
}

fn dedent(lines: &[&str]) -> String {
    let indent = lines.iter().filter(|l| !l.trim().is_empty()).map(|l| l.len() - l.trim_start().len()).min().unwrap_or(0);
    let body: Vec<&str> = lines.iter().map(|l| if l.len() >= indent { &l[indent..] } else { l.trim_start() }).collect();
    body.join("\n").trim_matches('\n').to_string()
}

/// The page with each example block replaced by its region, and the names
/// of the regions it shows.
fn render_examples(text: &str, regions: &BTreeMap<String, String>) -> Result<(String, Vec<String>), String> {
    const OPEN: &str = "<!-- example: ";
    let mut out = String::new();
    let mut names = Vec::new();
    let mut rest = text;
    while let Some(at) = rest.find(OPEN) {
        out.push_str(&rest[..at]);
        let after = &rest[at + OPEN.len()..];
        let name_end = after.find(" -->").ok_or("an unclosed example marker")?;
        let name = after[..name_end].trim().to_string();
        let close = after.find("<!-- /example -->").ok_or(format!("example {name} has no <!-- /example -->"))?;
        let code = regions.get(&name).ok_or(format!("example {name} names no region in tests/doc_examples.rs"))?;
        out.push_str(&format!("{OPEN}{name} -->\n```rust\n{code}\n```\n<!-- /example -->"));
        names.push(name);
        rest = &after[close + "<!-- /example -->".len()..];
    }
    out.push_str(rest);
    // Outside the markers, no Rust block: every example is compiled.
    let mut outside = out.clone();
    while let (Some(a), Some(b)) = (outside.find(OPEN), outside.find("<!-- /example -->")) {
        outside.replace_range(a..b + "<!-- /example -->".len(), "");
    }
    if outside.contains("```rust") {
        return Err("a ```rust block outside <!-- example --> markers: make it a region of tests/doc_examples.rs".into());
    }
    Ok((out, names))
}

/// Compares `path` with `wanted`, or writes it under `FERROX_BLESS=1`.
fn check_or_bless(path: &Path, wanted: &str) {
    if std::env::var_os("FERROX_BLESS").is_some() {
        fs::write(path, wanted).unwrap();
        return;
    }
    let have = fs::read_to_string(path).unwrap_or_default();
    assert!(
        have == wanted,
        "{} is stale: run `FERROX_BLESS=1 cargo test -p ferrox-ui --all-features --test docs_current`",
        path.display()
    );
}

// ---------------------------------------------------------------------------
// The index
// ---------------------------------------------------------------------------

fn render_index() -> String {
    let mut out = String::from(
        "# ferrox-ui: index\n\n\
         Every public component (with its props) and every other public item, by\n\
         group, with the first line of its documentation. Generated from the source\n\
         by `tests/docs_current.rs`: don't edit it; a test fails when it's stale.\n\
         Each component's page (`docs/<name>.md`) has its example and behaviour.\n",
    );
    for (group, feature) in GROUPS {
        out.push_str(&format!("\n## {group} ({feature})\n"));
        for file in source_files(&crate_dir().join("src").join(group)) {
            let text = fs::read_to_string(&file).unwrap();
            let parsed = syn::parse_file(&text).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
            for item in &parsed.items {
                if let Some(entry) = render_item(item) {
                    out.push('\n');
                    out.push_str(&entry);
                }
            }
        }
    }
    out
}

/// The `.rs` files of a group, sorted, `mod.rs` left out (it only re-exports).
fn source_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "rs") && !p.ends_with("mod.rs"))
        .collect();
    files.sort();
    files
}

fn render_item(item: &Item) -> Option<String> {
    match item {
        Item::Fn(f) if is_public(&f.vis) && has_attr(&f.attrs, "component") => {
            let name = f.sig.ident.to_string();
            let mut out = format!("### `{name}` (component)\n{}\n", summary(&f.attrs));
            for input in &f.sig.inputs {
                if let FnArg::Typed(arg) = input {
                    let Pat::Ident(ident) = &*arg.pat else { continue };
                    out.push_str(&format!("- `{}: {}`{}\n", ident.ident, tidy(&arg.ty.to_token_stream().to_string()), prop_note(&arg.attrs)));
                }
            }
            Some(out)
        }
        Item::Fn(f) if is_public(&f.vis) => {
            let sig = tidy(&f.sig.to_token_stream().to_string());
            Some(format!("### `{}` (function)\n`{sig}`: {}\n", f.sig.ident, summary(&f.attrs)))
        }
        Item::Enum(e) if is_public(&e.vis) => {
            let variants: Vec<String> = e.variants.iter().map(|v| v.ident.to_string()).collect();
            Some(format!("### `{}` (enum)\n{} Variants: {}.\n", e.ident, summary(&e.attrs), variants.join(", ")))
        }
        Item::Struct(s) if is_public(&s.vis) => {
            let fields: Vec<String> = s.fields.iter().filter(|f| is_public(&f.vis)).filter_map(|f| f.ident.as_ref().map(|i| i.to_string())).collect();
            let fields = if fields.is_empty() { String::new() } else { format!(" Fields: {}.", fields.join(", ")) };
            Some(format!("### `{}` (struct)\n{}{fields}\n", s.ident, summary(&s.attrs)))
        }
        Item::Const(c) if is_public(&c.vis) => Some(format!("### `{}` (const)\n{}\n", c.ident, summary(&c.attrs))),
        _ => None,
    }
}

fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

fn has_attr(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|a| a.path().is_ident(name))
}

/// The first sentence of an item's doc comment, on one line.
fn summary(attrs: &[Attribute]) -> String {
    let mut text = String::new();
    for attr in attrs {
        let Meta::NameValue(nv) = &attr.meta else { continue };
        if !nv.path.is_ident("doc") {
            continue;
        }
        let Expr::Lit(lit) = &nv.value else { continue };
        let Lit::Str(s) = &lit.lit else { continue };
        let line = s.value();
        if line.trim().is_empty() {
            if !text.is_empty() {
                break;
            }
            continue;
        }
        text.push_str(line.trim());
        text.push(' ');
    }
    let text = text.trim();
    let end = text.find(". ").map(|i| i + 1).unwrap_or(text.len());
    let first = text[..end].trim();
    if first.is_empty() { "(undocumented)".to_string() } else { first.to_string() }
}

/// A prop's `#[prop(…)]` options, as a note: optional, its default, `into`.
fn prop_note(attrs: &[Attribute]) -> String {
    let Some(prop) = attrs.iter().find(|a| a.path().is_ident("prop")) else {
        return String::new();
    };
    let opts = tidy(&prop.meta.to_token_stream().to_string());
    let mut notes = Vec::new();
    if let Some(at) = opts.find("default =") {
        // The rest of `prop(…)`, without the attribute's own closing paren.
        let rest = &opts[at + "default =".len()..];
        let default = rest.strip_suffix(')').unwrap_or(rest).trim();
        let default = default.trim_end_matches(".into()").trim_end_matches(".to_string()");
        notes.push(format!("default `{default}`"));
    } else if opts.contains("optional") {
        notes.push("optional".to_string());
    }
    if notes.is_empty() { String::new() } else { format!(", {}", notes.join(", ")) }
}

/// Token-stream spacing, made readable: `Signal < String >` → `Signal<String>`.
fn tidy(tokens: &str) -> String {
    let mut s = tokens.to_string();
    for (from, to) in [
        (" < ", "<"), ("< ", "<"), (" >", ">"), (" ,", ","), ("& ", "&"), (" :: ", "::"), (":: ", "::"), (" ::", "::"),
        ("( ", "("), (" )", ")"), (" !", "!"), ("[ ", "["), (" ]", "]"), (" .", "."), (". ", "."), ("' static", "'static"),
        ("- >", "->"), ("= >", "=>"), (" (", "("), (" : ", ": "), (" ;", ";"),
    ] {
        while s.contains(from) {
            s = s.replace(from, to);
        }
    }
    s
}
