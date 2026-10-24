//! The agent kit's scripts (`agent-kit/check/`): the bash and the
//! PowerShell implementation of each give the same output, the one their
//! spec describes, on the same fixtures. The PowerShell half needs `pwsh`;
//! without it, it's skipped and says so.

use std::path::{Path, PathBuf};
use std::process::Command;

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn kit() -> PathBuf {
    crate_dir().join("agent-kit/check")
}

fn repo() -> PathBuf {
    crate_dir().parent().unwrap().to_path_buf()
}

/// The implementations to run: bash always, PowerShell when `pwsh` exists.
fn shells() -> Vec<&'static str> {
    let mut shells = vec!["bash"];
    if Command::new("pwsh").arg("-Version").output().is_ok_and(|o| o.status.success()) {
        shells.push("pwsh");
    } else {
        eprintln!("agent_kit: pwsh not found; the PowerShell scripts are NOT tested");
    }
    shells
}

/// Runs `script` (without its extension) with `shell` on `args`: stdout and
/// the exit code.
fn run(shell: &str, script: &str, args: &[&Path]) -> (String, i32) {
    let mut command = match shell {
        "bash" => {
            let mut c = Command::new("bash");
            c.arg(kit().join(format!("{script}.sh")));
            c
        }
        _ => {
            let mut c = Command::new("pwsh");
            c.args(["-NoProfile", "-File"]).arg(kit().join(format!("{script}.ps1")));
            c
        }
    };
    let output = command.args(args).output().unwrap();
    (String::from_utf8(output.stdout).unwrap(), output.status.code().unwrap_or(-1))
}

#[test]
fn check_finds_every_rule_broken_in_the_fixture() {
    let expected = std::fs::read_to_string(kit().join("tests/fixtures/drifty.expected")).unwrap();
    for shell in shells() {
        let (out, code) = run(shell, "ferrox-check", &[&kit().join("tests/fixtures/drifty")]);
        assert_eq!(out, expected, "{shell}: output differs from drifty.expected");
        assert_eq!(code, 1, "{shell}");
    }
}

#[test]
fn check_passes_a_clean_app_the_starter_and_the_recipes() {
    for folder in [kit().join("tests/fixtures/clean"), repo().join("starter"), repo().join("demo/src/recipes")] {
        for shell in shells() {
            let (out, code) = run(shell, "ferrox-check", &[&folder]);
            assert_eq!((out.as_str(), code), ("ferrox-check: no drift\n", 0), "{shell} on {}", folder.display());
        }
    }
}

#[test]
fn check_refuses_a_missing_folder() {
    for shell in shells() {
        let (_, code) = run(shell, "ferrox-check", &[Path::new("/no/such/folder")]);
        assert_eq!(code, 2, "{shell}");
    }
}

#[test]
fn docs_finds_the_docs_of_the_ferrox_ui_an_app_builds_against() {
    let docs = format!("{}\n", crate_dir().join("docs").display());
    for shell in shells() {
        let (out, code) = run(shell, "ferrox-docs", &[&repo().join("demo")]);
        assert_eq!((out.as_str(), code), (docs.as_str(), 0), "{shell}");
    }
}

#[test]
fn docs_says_so_when_ferrox_ui_is_not_a_dependency() {
    let app = std::env::temp_dir().join(format!("ferrox-docs-no-dep-{}", std::process::id()));
    std::fs::create_dir_all(app.join("src")).unwrap();
    std::fs::write(app.join("Cargo.toml"), "[package]\nname = \"plain\"\nversion = \"0.1.0\"\nedition = \"2024\"\n").unwrap();
    std::fs::write(app.join("src/main.rs"), "fn main() {}\n").unwrap();
    for shell in shells() {
        let (out, code) = run(shell, "ferrox-docs", &[&app]);
        assert_eq!((out.as_str(), code), ("", 1), "{shell}");
    }
    std::fs::remove_dir_all(&app).unwrap();
}
