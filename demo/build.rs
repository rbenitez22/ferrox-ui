//! The same setup an app has: embed the demo's own themes for `register`, and
//! write the stylesheet + fonts that index.html links, and generate the help
//! topics from `help/*.md`.

fn main() {
    ferrox_theme_build::embed_themes("style/themes");
    ferrox_theme_build::write_css("style/ferrox", Some("style/themes"));
    ferrox_help_build::Help::load("help").guide_prefix("/help-guide#").write_rust("help_topics.rs");
}
