//! One copy of ferrox per build: ferrox-theme, ferrox-icons (which ferrox-ui
//! pulls in) and ferrox-theme-build each declare a `links` value, and Cargo
//! allows one package per value. An app whose dependencies bring a second
//! copy (another tag, or the same tag through another URL) fails to
//! resolve. Here the second copy is a stand-in crate declaring the same
//! value, beside the real one.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

/// A throwaway app depending on the real `krate` and, with `second`, on a
/// stand-in that claims the same `links` value; the output of resolving it.
fn resolve(krate: &str, second: bool) -> (bool, String) {
    let dir = std::env::temp_dir().join(format!("ferrox-one-copy-{krate}-{second}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    write(
        &dir.join("second/Cargo.toml"),
        &format!("[package]\nname = \"second-{krate}\"\nversion = \"0.1.0\"\nedition = \"2024\"\nlinks = \"{krate}\"\n"),
    );
    write(&dir.join("second/build.rs"), "fn main() {}\n");
    write(&dir.join("second/src/lib.rs"), "");
    let real = repo().join(krate);
    write(
        &dir.join("app/Cargo.toml"),
        &format!(
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\n\n[dependencies]\n\
             real = {{ package = \"{krate}\", path = \"{}\" }}\n{}",
            real.display(),
            if second { format!("second = {{ package = \"second-{krate}\", path = \"../second\" }}\n") } else { String::new() }
        ),
    );
    write(&dir.join("app/src/lib.rs"), "");
    let out = Command::new(env!("CARGO"))
        .args(["generate-lockfile", "--offline"])
        .current_dir(dir.join("app"))
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    (out.status.success(), String::from_utf8_lossy(&out.stderr).into_owned())
}

#[test]
fn a_second_copy_of_any_ferrox_crate_fails_to_resolve() {
    for krate in ["ferrox-theme", "ferrox-icons", "ferrox-theme-build"] {
        let (ok, err) = resolve(krate, false);
        assert!(ok, "{krate}: one copy should resolve: {err}");
        let (ok, err) = resolve(krate, true);
        assert!(!ok, "{krate}: a second copy resolved");
        assert!(err.contains(&format!("links to the native library `{krate}`")), "{krate}: {err}");
    }
}
