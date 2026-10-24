# Task: check the app for drift

1. Run `{{KIT}}/check/ferrox-check.sh <app folder>` (or `ferrox-check.ps1`).
   With no folder, it checks the current one. It lists each problem as
   `file:line: rule: what's wrong (fix: what to use)`, then a summary, and
   exits 0 when there's no drift.
2. Fix each finding as its `fix` says: replace a hand-made control with the
   component, an inline style or a written-out colour or size with a
   component prop, a layout component or a theme token; add a missing
   setup step (the `setup` task).
3. Only when something must stay as it is (an `<input type="file">`,
   which ferrox-ui doesn't have), mark that one line with a comment
   `ferrox-check: allow <rule id>` on it or just above it, and tell the
   user why.
4. Run the check again until it says `ferrox-check: no drift`, then build
   the app.
