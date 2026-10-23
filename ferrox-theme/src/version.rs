//! Keeping `ferrox-theme-build` and `ferrox-theme` on the same version.
//!
//! An app's `build.rs` uses `ferrox-theme-build` while its code uses
//! ferrox-theme (directly or through a re-export). Both must come from the same
//! ferrox-ui tag. The code the build crate generates calls
//! [`check_build_version`] in a `const`, so a mismatch fails the build with
//! this message instead of a confusing error inside generated code.

/// This crate's version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Fails compilation (when used in a `const`) unless `build_version` is
/// [`VERSION`]. Called by the code `ferrox-theme-build` generates.
///
/// ```compile_fail
/// const _: () = ferrox_theme::check_build_version("0.0.0-other");
/// ```
///
/// ```
/// const _: () = ferrox_theme::check_build_version(ferrox_theme::VERSION);
/// ```
pub const fn check_build_version(build_version: &str) {
    if !same(build_version.as_bytes(), VERSION.as_bytes()) {
        panic!(
            "ferrox-theme-build and ferrox-theme are different versions: depend on both from the same ferrox-ui tag"
        );
    }
}

const fn same(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_version_passes() {
        check_build_version(VERSION);
    }

    #[test]
    #[should_panic(expected = "different versions")]
    fn other_version_fails() {
        check_build_version("0.0.0-other");
    }
}
