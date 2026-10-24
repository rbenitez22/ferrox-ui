# Task: set up ferrox-ui in an app

Add ferrox-ui to a Leptos CSR app built with Trunk, or start a new app.
The setup lives in one place, the starter app, beside the docs: with DOCS
the folder `{{KIT}}/check/ferrox-docs` prints, the starter is
`DOCS/../../starter`, and its `README.md` names every file and its job.

## A new app

1. Ask the user for the app's name if they haven't given it.
2. Copy the starter folder to where the app goes (everything but `target/`
   and `dist/`).
3. In the copy: set `name` in `Cargo.toml` and the `<title>` in
   `index.html`. Keep the ferrox dependencies exactly as the starter pins
   them (one git reference for all four crates).
4. Build: `trunk build` in the app's folder (`trunk serve` to look at it).
   Replace the starter's page with the app's own screens (the `screen`
   task).

## An existing app

Take each part from the starter's file of the same name, as its README
lists them:

1. `Cargo.toml`: the four ferrox dependencies, all with the same git
   reference, and the feature groups the app needs.
2. `build.rs`: the `ferrox_theme_build::write_css` call.
3. `index.html`: the `<link data-trunk>` lines at the top of `<head>`, in
   the starter's order (the pre-paint script first), and
   `class="ferrox-app"` on `<body>`.
4. `Trunk.toml`: the `pre_build` hook and the `[watch]` ignore.
5. `src/main.rs`: `ferrox_theme::init()`, then `ferrox_ui::init()`, before
   mounting.
6. `.gitignore`: `style/ferrox/`.

Then build, and run the `check` task: its `require` rules confirm the setup.

## Done when

The app builds, `ferrox-check` reports no drift, and the page has the
theme's background and font.
