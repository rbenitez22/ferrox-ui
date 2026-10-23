//! Generates `$OUT_DIR/icons.rs`: one `IconData` const per `icons/*.svg`,
//! after checking each file (`build/validate.rs`). A file that breaks a rule
//! fails the build with its name and the reason.

#[path = "build/mod.rs"]
mod icons;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=icons");
    println!("cargo:rerun-if-changed=build");

    let dir = Path::new("icons");
    let sources = icons::sources::parse(&read(&dir.join("SOURCES")))
        .unwrap_or_else(|e| panic!("icons/SOURCES: {e}"));
    let files = icons::discover::svg_files(dir)
        .unwrap_or_else(|e| panic!("cannot list {}: {e}", dir.display()));
    check_sources_match(&sources, &files);

    let code: String = files.iter().map(|file| generate(file, &sources)).collect();
    let target = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by cargo")).join("icons.rs");
    fs::write(&target, code).unwrap_or_else(|e| panic!("cannot write {}: {e}", target.display()));
}

/// Every icon file has a line in SOURCES and every line has a file.
fn check_sources_match(sources: &BTreeMap<String, String>, files: &[PathBuf]) {
    let stems: Vec<&str> = files.iter().map(|f| stem(f)).collect();
    for stem in &stems {
        assert!(sources.contains_key(*stem), "icons/{stem}.svg has no line in icons/SOURCES");
    }
    for name in sources.keys() {
        assert!(stems.contains(&name.as_str()), "icons/SOURCES names {name}, but icons/{name}.svg does not exist");
    }
}

/// The generated const for one icon file, or a panic naming the file and the rule it breaks.
fn generate(file: &Path, sources: &BTreeMap<String, String>) -> String {
    let stem = stem(file);
    let fail = |e: String| -> ! { panic!("{}: {e}", file.display()) };
    icons::naming::check_kebab(stem).unwrap_or_else(|e| fail(e));
    let text = read(file);
    let doc = roxmltree::Document::parse(&text).unwrap_or_else(|e| fail(e.to_string()));
    icons::validate::check(&doc).unwrap_or_else(|e| fail(e));
    icons::emit::icon_const(stem, &sources[stem], &icons::markup::body(doc.root_element()))
}

fn stem(file: &Path) -> &str {
    file.file_stem().and_then(|s| s.to_str()).unwrap_or_else(|| panic!("{}: file name is not UTF-8", file.display()))
}

fn read(file: &Path) -> String {
    fs::read_to_string(file).unwrap_or_else(|e| panic!("cannot read {}: {e}", file.display()))
}
