//! The inline `<script>` that sets `data-theme` before the WASM bundle loads.

/// Paste into `index.html`'s `<head>`, inside `<script>…</script>`, before any
/// stylesheet. It reads [`super::STORAGE_KEY`] (or the legacy webapp-lib key)
/// and sets `data-theme` on `<html>`. With the stylesheet linked in
/// `index.html` (see `ferrox-theme-build`), the first paint is already in the
/// user's theme; waiting for WASM would flash the default theme on every load.
///
/// It never needs to change when themes are added: the stored value carries
/// the resolved light and dark ids (see `persist`).
pub const PREPAINT_SCRIPT: &str = r#"(function () {
  try {
    var parts = (localStorage.getItem('ferrox.theme') ||
      localStorage.getItem('webapp-lib.theme') || '').split(':');
    if (parts.length < 4) return;
    var mode = parts[1];
    var dark = mode === 'dark' || (mode === 'system' && window.matchMedia &&
      window.matchMedia('(prefers-color-scheme: dark)').matches);
    var theme = dark ? parts[3] : parts[2];
    if (theme) document.documentElement.setAttribute('data-theme', theme);
  } catch (e) {
    /* A blocked localStorage must not stop the app booting. */
  }
})();"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::STORAGE_KEY;

    #[test]
    fn reads_the_same_keys_as_persist() {
        assert!(PREPAINT_SCRIPT.contains(&format!("'{STORAGE_KEY}'")));
        assert!(PREPAINT_SCRIPT.contains(&format!("'{}'", crate::persist::LEGACY_STORAGE_KEY)));
    }
}
