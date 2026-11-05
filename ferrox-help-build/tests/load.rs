use ferrox_help_build::{Block, Help};

const OK: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/ok");
const BAD: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/bad");

#[test]
fn loads_areas_in_order_and_skips_the_writers_readme() {
    let help = Help::try_load(OK).unwrap();
    let titles: Vec<_> = help.areas().iter().map(|a| a.title.as_str()).collect();
    assert_eq!(titles, ["First", "Second"]);
    assert_eq!(help.areas()[0].intro.as_deref(), Some("An intro."));
    assert_eq!(help.areas()[1].topics[0].blocks[1], Block::Table {
        columns: vec!["K".into(), "V".into()],
        rows: vec![vec!["1".into(), "2".into()]],
    });
}

#[test]
fn renders_statics_for_every_topic() {
    let code = Help::try_load(OK).unwrap().guide_prefix("/help#").render_rust();
    assert!(code.contains("pub static ONE: ::std::sync::LazyLock<::ferrox_ui::HelpTopic>"));
    assert!(code.contains(".related([\"two\"])"));
    assert!(code.contains(".guide(\"/help#two\")"));
}

#[test]
fn reports_every_problem() {
    let errors: Vec<String> = Help::try_load(BAD).unwrap_err().iter().map(ToString::to_string).collect();
    assert_eq!(errors, [
        "stray.md: not listed in ORDER",
        "first.md:3: topic `one` must start with a paragraph that stands on its own",
    ]);
}

#[test]
fn missing_order() {
    let errors = Help::try_load(env!("CARGO_MANIFEST_DIR")).unwrap_err();
    assert!(errors[0].to_string().starts_with("ORDER: cannot read"));
}
