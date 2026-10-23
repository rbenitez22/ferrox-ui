//! Finds the icon files.

use std::path::{Path, PathBuf};
use std::{fs, io};

/// Every `*.svg` file directly in `dir`, sorted by icon name (the file
/// stem): `eye` before `eye-off`, though `eye-off.svg` sorts first as a path.
pub fn svg_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "svg") {
            files.push(path);
        }
    }
    files.sort_by(|a, b| a.file_stem().cmp(&b.file_stem()));
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_only_svg_files_sorted() {
        let dir = std::env::temp_dir().join(format!("ferrox-icons-discover-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        for name in ["plus.svg", "eye-off.svg", "close.svg", "eye.svg", "SOURCES", "notes.txt"] {
            fs::write(dir.join(name), "").unwrap();
        }
        let names: Vec<_> = svg_files(&dir).unwrap().iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        fs::remove_dir_all(&dir).unwrap();
        assert_eq!(names, ["close.svg", "eye.svg", "eye-off.svg", "plus.svg"]);
    }
}
