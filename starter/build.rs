//! Writes the theme's stylesheet, fonts and pre-paint script into
//! style/ferrox/, which index.html links. Generated: keep it out of git.

fn main() {
    ferrox_theme_build::write_css("style/ferrox", None);
}
