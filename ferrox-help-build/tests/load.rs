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
    assert!(code.contains("::ferrox_ui::HelpArea::new(\"First\", [ONE.clone()]).intro(\"An intro.\"),"));
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

#[test]
fn json_round_trips() {
    let help = Help::try_load(OK).unwrap().guide_prefix("/help#");
    let doc: serde_json::Value = serde_json::from_str(&help.render_json()).unwrap();
    let areas = doc["areas"].as_array().unwrap();
    assert_eq!(areas.len(), 2);
    assert_eq!(areas[0]["intro"], "An intro.");
    assert_eq!(areas[1]["intro"], serde_json::Value::Null);
    let one = &areas[0]["topics"][0];
    assert_eq!(one["id"], "one");
    assert_eq!(one["blocks"][0]["paragraph"], "The first topic.");
    assert_eq!(one["blocks"][1]["list"][1], "b");
    assert_eq!(one["related"][0], "two");
    assert_eq!(one["guide"], "/help#one");
    assert_eq!(areas[1]["topics"][0]["blocks"][1]["table"]["rows"][0][1], "2");
}

#[test]
fn json_of_a_real_doc_set_parses() {
    // Rusty Skillet's help, when it sits beside this repo.
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../rusty-skillet/docs/help");
    let Ok(help) = Help::try_load(dir) else { return };
    let doc: serde_json::Value = serde_json::from_str(&help.render_json()).unwrap();
    let count: usize = doc["areas"].as_array().unwrap().iter().map(|a| a["topics"].as_array().unwrap().len()).sum();
    assert!(count > 0);
}
