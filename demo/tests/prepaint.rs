//! index.html must carry the theme's pre-paint script verbatim.

#[test]
fn index_html_has_the_current_prepaint_script() {
    let index = include_str!("../index.html");
    assert!(
        index.contains(ferrox_theme::PREPAINT_SCRIPT),
        "demo/index.html's <script> differs from ferrox_theme::PREPAINT_SCRIPT; paste the current one"
    );
}
