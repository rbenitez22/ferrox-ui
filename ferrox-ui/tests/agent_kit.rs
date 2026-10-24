//! The agent kit (`agent-kit/`): its scripts' bash and PowerShell
//! implementations give the same output, the one their spec describes, on
//! the same fixtures (the PowerShell half needs `pwsh`; without it, it's
//! skipped and says so); the installer puts the same files in place from
//! both; the agent-neutral core names no particular agent; `kit.tsv` and
//! `VERSION` match what's there.

use std::path::{Path, PathBuf};
use std::process::Command;

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn kit() -> PathBuf {
    crate_dir().join("agent-kit/check")
}

fn kit_root() -> PathBuf {
    crate_dir().join("agent-kit")
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
    run_in(&kit(), shell, script, args.iter().map(|a| a.as_os_str()).collect())
}

/// Runs `<dir>/<script>.sh` or `.ps1` with `shell`.
fn run_in(dir: &Path, shell: &str, script: &str, args: Vec<&std::ffi::OsStr>) -> (String, i32) {
    let mut command = match shell {
        "bash" => {
            let mut c = Command::new("bash");
            c.arg(dir.join(format!("{script}.sh")));
            c
        }
        _ => {
            let mut c = Command::new("pwsh");
            c.args(["-NoProfile", "-File"]).arg(dir.join(format!("{script}.ps1")));
            c
        }
    };
    let output = command.args(args).output().unwrap();
    (String::from_utf8(output.stdout).unwrap(), output.status.code().unwrap_or(-1))
}

/// A fresh, empty folder for one test.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ferrox-kit-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Every file under `dir`, relative, sorted, with its bytes.
fn tree(dir: &Path) -> Vec<(String, Vec<u8>)> {
    fn walk(base: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                out.push((path.strip_prefix(base).unwrap().display().to_string(), std::fs::read(&path).unwrap()));
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, dir, &mut out);
    out.sort();
    out
}

fn install(shell: &str, args: &[&str], app: &Path) -> (String, i32) {
    let mut all: Vec<&std::ffi::OsStr> = args.iter().map(std::ffi::OsStr::new).collect();
    all.push(app.as_os_str());
    run_in(&kit_root().join("install"), shell, "install", all)
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

#[test]
fn install_puts_the_same_kit_in_place_from_either_shell() {
    let mut installed = Vec::new();
    for shell in shells() {
        let app = scratch(&format!("install-{shell}"));
        let (out, code) = install(shell, &["--generic"], &app);
        assert_eq!(code, 0, "{shell}: {out}");
        assert!(out.ends_with("ferrox-kit: installed 0.1.0 (generic) into .ferrox\n"), "{shell}: {out}");
        let files = tree(&app);
        assert!(files.iter().any(|(p, _)| p == ".ferrox/core/AGENTS.md"));
        assert!(!files.iter().any(|(p, _)| p.contains("/tests/")), "{shell}: tests copied");
        for (path, bytes) in &files {
            if path.ends_with(".md") {
                assert!(!String::from_utf8_lossy(bytes).contains("{{KIT}}"), "{shell}: {path} keeps a placeholder");
            }
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let script = app.join(".ferrox/check/ferrox-check.sh");
            assert!(std::fs::metadata(&script).unwrap().permissions().mode() & 0o111 != 0, "{shell}: not executable");
        }
        installed.push((out, files));
        std::fs::remove_dir_all(&app).unwrap();
    }
    if let [(out_a, tree_a), (out_b, tree_b)] = installed.as_slice() {
        assert_eq!(out_a, out_b, "the installers print differently");
        assert!(tree_a == tree_b, "the installers write different files");
    }
}

#[test]
fn install_dry_run_writes_nothing_and_an_old_kit_is_kept() {
    for shell in shells() {
        let app = scratch(&format!("again-{shell}"));
        let (out, code) = install(shell, &["--generic", "--dry-run"], &app);
        assert_eq!(code, 0);
        assert!(out.starts_with("would write .ferrox/core/AGENTS.md\n"), "{shell}: {out}");
        assert!(out.ends_with("ferrox-kit: dry run, nothing written\n"));
        assert!(tree(&app).is_empty(), "{shell}: a dry run wrote files");

        install(shell, &["--generic"], &app);
        std::fs::create_dir_all(app.join(".ferrox.bak")).unwrap();
        let (out, _) = install(shell, &["--generic"], &app);
        assert!(out.starts_with("backed up .ferrox to .ferrox.bak.2\n"), "{shell}: {out}");
        assert!(app.join(".ferrox.bak.2/core/AGENTS.md").exists());
        assert!(app.join(".ferrox/core/AGENTS.md").exists());
        std::fs::remove_dir_all(&app).unwrap();
    }
}

#[test]
fn install_refuses_what_it_does_not_understand() {
    let app = scratch("usage");
    for shell in shells() {
        for args in [vec![], vec!["--claude"], vec!["--generic", "--nope"]] {
            let (_, code) = install(shell, &args, &app);
            assert_eq!(code, 2, "{shell} {args:?}");
        }
        let (_, code) = install(shell, &["--generic"], Path::new("/no/such/app"));
        assert_eq!(code, 2, "{shell}");
    }
    std::fs::remove_dir_all(&app).unwrap();
}

/// What only one agent understands: its tools, its files, its syntax.
const AGENT_SPECIFIC: &[&str] = &[
    "Claude", "Copilot", "Cursor", "Codex", "Gemini", "SKILL.md", "CLAUDE.md", ".claude", ".github", "Skill tool", "Bash tool",
    "subagent_type", "slash command", "$ARGUMENTS", "#file:", "@workspace", "frontmatter",
];

#[test]
fn the_core_names_no_particular_agent() {
    let mut pages = vec![kit_root().join("BOOTSTRAP.md")];
    let mut dirs = vec![kit_root().join("core")];
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() { dirs.push(path) } else { pages.push(path) }
        }
    }
    for page in pages {
        let text = std::fs::read_to_string(&page).unwrap();
        for term in AGENT_SPECIFIC {
            assert!(!text.contains(term), "{} names {term:?}: the core is for any agent", page.display());
        }
    }
}

#[test]
fn kit_tsv_lists_files_that_exist_and_version_is_the_crates() {
    let text = std::fs::read_to_string(kit_root().join("kit.tsv")).unwrap();
    for line in text.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let fields: Vec<&str> = line.split('\t').collect();
        assert_eq!(fields.len(), 4, "kit.tsv: {line}");
        assert!(["entry", "task", "role"].contains(&fields[0]), "kit.tsv: {line}");
        assert!(kit_root().join(fields[2]).is_file(), "kit.tsv names a missing file: {}", fields[2]);
    }
    let version = std::fs::read_to_string(kit_root().join("VERSION")).unwrap();
    assert_eq!(version.trim(), env!("CARGO_PKG_VERSION"), "agent-kit/VERSION differs from the crate's version");
}
