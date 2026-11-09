//! Generates `$OUT_DIR/icons.rs`: one `IconData` const per `icons/*.svg`,
//! after checking each file (`ferrox-icons-build`, the rules app icons follow
//! too), and the `ALL` table. A file that breaks a rule fails the build with
//! its name and the reason.

use ferrox_icons_build::{check_file, discover, emit, sources, stem};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=icons");

    let dir = Path::new("icons");
    let sources = sources::parse(&read(&dir.join("SOURCES")))
        .unwrap_or_else(|e| panic!("icons/SOURCES: {e}"));
    let files = discover::svg_files(dir)
        .unwrap_or_else(|e| panic!("cannot list {}: {e}", dir.display()));
    let stems: Vec<String> = files.iter().map(|f| file_stem(f)).collect();
    check_sources_match(&sources, &stems);

    let mut code: String = files.iter().map(|file| generate(file, &sources)).collect();
    code.push_str(&emit::all_table(&stems.iter().map(String::as_str).collect::<Vec<_>>()));
    let target = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by cargo")).join("icons.rs");
    fs::write(&target, code).unwrap_or_else(|e| panic!("cannot write {}: {e}", target.display()));
}

/// Every icon file has a line in SOURCES and every line has a file.
fn check_sources_match(sources: &BTreeMap<String, String>, stems: &[String]) {
    for stem in stems {
        assert!(sources.contains_key(stem), "icons/{stem}.svg has no line in icons/SOURCES");
    }
    for name in sources.keys() {
        assert!(stems.contains(name), "icons/SOURCES names {name}, but icons/{name}.svg does not exist");
    }
}

/// The generated const for one icon file, or a panic naming the file and the rule it breaks.
fn generate(file: &Path, sources: &BTreeMap<String, String>) -> String {
    let icon = check_file(file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
    emit::icon_const(&icon.name, &sources[&icon.name], &icon.body)
}

fn file_stem(file: &Path) -> String {
    stem(file).unwrap_or_else(|e| panic!("{}: {e}", file.display()))
}

fn read(file: &Path) -> String {
    fs::read_to_string(file).unwrap_or_else(|e| panic!("cannot read {}: {e}", file.display()))
}
