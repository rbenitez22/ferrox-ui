# The measuring prompt

The reference task from `plans/agent-first.md` ("9. Measure it"), word for
word, so every run is comparable. Install the kit with `--generic` into an
empty folder, then give a fresh agent (no skills, no memory) the text
below the line, with `{{APP}}` replaced by that folder's absolute path and
`{{REPO}}` by the ferrox-ui checkout's. Its first paragraph is
`BOOTSTRAP.md`'s, as installed.

---

This project's UI is built with ferrox-ui, and its agent kit is in `.ferrox/`. Before any UI work, read `.ferrox/core/AGENTS.md` and follow it. For a job, follow its task file in `.ferrox/core/tasks/` (`setup`, `screen`, `component`, `check`). Before saying UI work is done, run `.ferrox/check/ferrox-check.sh` (or `ferrox-check.ps1`) on the app's folder and fix what it reports.

The app's folder is {{APP}} (the `.ferrox/` above is inside it). It is otherwise empty. ferrox-ui is checked out at {{REPO}}; depend on it by path.

Task: make this a new Leptos CSR app with a recipes list page: add a recipe in a dialog; row actions: edit, delete with confirm; empty, loading and error states; in-memory data with a 500 ms simulated load. Done when `trunk build` passes in the app's folder. Don't run `trunk serve`. Don't change anything outside the app's folder.

When finished, report: which files you read (paths, and for long ones roughly how much), every build round and any compile error, the drift check's final output, and anywhere the docs or kit left you guessing or were wrong.
