//! Build-time support for ferrox-ui help. An app writes its help topics in
//! a small Markdown subset; this crate validates them and generates one
//! `HelpTopic` static per topic, so a `help=` prop names a constant and a
//! wrong topic is a compile error.
//!
//! ```rust,ignore
//! // build.rs
//! fn main() {
//!     ferrox_help_build::Help::load("../docs/help")
//!         .guide_prefix("/help#")
//!         .write_rust("help_topics.rs");
//! }
//! ```
//!
//! ```rust,ignore
//! // src/help.rs
//! include!(concat!(env!("OUT_DIR"), "/help_topics.rs"));
//!
//! // the app root: related links in popovers, and the full guide
//! ferrox_ui::provide_help_topics(crate::help::TOPICS.clone());
//!
//! // a view
//! <Page title="Recipes" help=crate::help::RECIPE_LIST.clone()>
//! ```
//!
//! # The help directory
//!
//! - `ORDER` lists the area files, one per line, in reading order.
//! - Every other `.md` file must be listed, except `README.md` (notes for
//!   writers).
//! - Each area file has one `# Area` heading, an optional intro paragraph,
//!   then topics: `## Title {#topic-id}`, each holding paragraphs, `- ` lists
//!   (two-space continuation lines), pipe tables, and an optional last line
//!   `Related: #id, #id`. `<!-- -->` comment lines are dropped.
//! - Ids are `^[a-z][a-z0-9-]*$`, unique across all files. The static is the
//!   id upper-snake-cased: `recipe-search` → `RECIPE_SEARCH`. `TOPICS` holds
//!   them all, so `topics` is not a valid id.
//! - The first block of every topic is a paragraph that stands on its own.
//! - Text is plain: `**`, backticks, `[..](..)` and HTML tags are rejected.
//!
//! An API's build script can write the same topics as JSON, for an
//! assistant: `Help::load(dir).guide_prefix("/help#").write_json("help.json")`.
//!
//! Any problem fails the build, with every problem listed as `file:line`.
//!
//! Std only, so an app's build script never compiles ferrox-ui itself.

mod check;
mod error;
mod json;
mod model;
mod parse;
mod rust;

use std::fs;
use std::path::{Path, PathBuf};

pub use check::{check_areas, RESERVED_IDS};
pub use error::HelpError;
pub use json::render_json;
pub use model::{Area, Block, Related, Topic};
pub use parse::{is_valid_id, parse_area};
pub use rust::{render_rust, static_name, RustOptions};

/// The file listing the area files in reading order.
pub const ORDER_FILE: &str = "ORDER";
/// Notes for writers, allowed in the directory without being listed.
pub const WRITERS_NOTES: &str = "README.md";

/// A loaded, validated help directory.
#[derive(Clone, Debug)]
pub struct Help {
    dir: PathBuf,
    files: Vec<PathBuf>,
    areas: Vec<Area>,
    guide_prefix: Option<String>,
    crate_path: String,
}

impl Help {
    /// Loads and validates `dir`, for a build script: tells cargo to rerun
    /// when any help file changes, and panics listing every problem.
    pub fn load(dir: impl AsRef<Path>) -> Self {
        let dir = dir.as_ref();
        println!("cargo:rerun-if-changed={}", dir.display());
        let help = Self::try_load(dir).unwrap_or_else(|errors| {
            let lines: Vec<String> = errors.iter().map(|e| format!("  {e}")).collect();
            panic!("invalid help files in {}:\n{}", dir.display(), lines.join("\n"));
        });
        for file in &help.files {
            println!("cargo:rerun-if-changed={}", file.display());
        }
        help
    }

    /// Loads and validates `dir`, returning every problem found.
    pub fn try_load(dir: impl AsRef<Path>) -> Result<Self, Vec<HelpError>> {
        let dir = dir.as_ref().to_path_buf();
        let names = read_order(&dir).map_err(|e| vec![e])?;
        let mut errors = unlisted_files(&dir, &names);
        let mut areas = Vec::new();
        let mut files = vec![dir.join(ORDER_FILE)];
        for name in &names {
            let path = dir.join(name);
            match fs::read_to_string(&path) {
                Ok(text) => match parse_area(name, &text) {
                    Ok(area) => areas.push(area),
                    Err(found) => errors.extend(found),
                },
                Err(e) => errors.push(HelpError::new(ORDER_FILE, 0, format!("cannot read {name}: {e}"))),
            }
            files.push(path);
        }
        if errors.is_empty() {
            errors = check_areas(&areas);
        }
        if errors.is_empty() {
            Ok(Self { dir, files, areas, guide_prefix: None, crate_path: "::ferrox_ui".into() })
        } else {
            Err(errors)
        }
    }

    /// Gives every topic `.guide(prefix + id)`: the "Full guide" link.
    pub fn guide_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.guide_prefix = Some(prefix.into());
        self
    }

    /// The path generated code uses for ferrox-ui; `::ferrox_ui` by default.
    pub fn crate_path(mut self, path: impl Into<String>) -> Self {
        self.crate_path = path.into();
        self
    }

    /// The areas, in reading order.
    pub fn areas(&self) -> &[Area] {
        &self.areas
    }

    /// The generated Rust source.
    pub fn render_rust(&self) -> String {
        let source = self.dir.display().to_string();
        render_rust(&self.areas, &RustOptions {
            crate_path: &self.crate_path,
            guide_prefix: self.guide_prefix.as_deref(),
            source: &source,
        })
    }

    /// The same topics as JSON, for an API to serve to an assistant.
    pub fn render_json(&self) -> String {
        render_json(&self.areas, self.guide_prefix.as_deref())
    }

    /// Writes the generated Rust into `$OUT_DIR/<file_name>` and returns its path.
    pub fn write_rust(&self, file_name: &str) -> PathBuf {
        write_out(file_name, &self.render_rust())
    }

    /// Writes the JSON into `$OUT_DIR/<file_name>` and returns its path. An
    /// API embeds it with `include_str!(concat!(env!("OUT_DIR"), "/help.json"))`.
    pub fn write_json(&self, file_name: &str) -> PathBuf {
        write_out(file_name, &self.render_json())
    }
}

/// Writes `contents` into `$OUT_DIR/<file_name>`.
fn write_out(file_name: &str, contents: &str) -> PathBuf {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is set by cargo for build scripts");
    let target = Path::new(&out_dir).join(file_name);
    fs::write(&target, contents).unwrap_or_else(|e| panic!("cannot write {}: {e}", target.display()));
    target
}

/// The area file names from `ORDER`, checked for shape and repeats.
fn read_order(dir: &Path) -> Result<Vec<String>, HelpError> {
    let path = dir.join(ORDER_FILE);
    let text = fs::read_to_string(&path)
        .map_err(|e| HelpError::new(ORDER_FILE, 0, format!("cannot read {}: {e}", path.display())))?;
    let mut names: Vec<String> = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let name = line.trim();
        if name.is_empty() {
            continue;
        }
        if !name.ends_with(".md") || name.contains('/') || name == WRITERS_NOTES {
            return Err(HelpError::new(ORDER_FILE, index + 1, format!("`{name}` is not an area file: a .md file in this directory, not {WRITERS_NOTES}")));
        }
        if names.iter().any(|n| n == name) {
            return Err(HelpError::new(ORDER_FILE, index + 1, format!("`{name}` is listed twice")));
        }
        names.push(name.to_string());
    }
    if names.is_empty() {
        return Err(HelpError::new(ORDER_FILE, 0, "lists no area files"));
    }
    Ok(names)
}

/// `.md` files in `dir` that `ORDER` doesn't list.
fn unlisted_files(dir: &Path, listed: &[String]) -> Vec<HelpError> {
    let Ok(entries) = fs::read_dir(dir) else { return Vec::new() };
    let mut found: Vec<String> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".md") && name != WRITERS_NOTES && !listed.contains(name))
        .collect();
    found.sort();
    found
        .into_iter()
        .map(|name| HelpError::new(&name, 0, format!("not listed in {ORDER_FILE}")))
        .collect()
}
