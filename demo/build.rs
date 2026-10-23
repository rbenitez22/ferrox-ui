//! The same setup an app has: embed the demo's own themes for `register`, and
//! write the stylesheet + fonts that index.html links.

fn main() {
    ferrox_theme_build::embed_themes("style/themes");
    ferrox_theme_build::write_css("style/ferrox", Some("style/themes"));
}
