//! The docs are current. `docs/index.md` is generated from the source: every
//! public component with its props, and every other public item, with the
//! first line of its doc comment. This test fails when the committed index
//! differs from what the source says; run it with `FERROX_BLESS=1` to
//! rewrite the file instead.

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
