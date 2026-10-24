# Using ferrox-ui with any agent

Paste the paragraph below into your agent's instructions for this project
(whatever file or setting it reads at the start of every session), or at
the start of a conversation. Any agent that can read files and run
commands can then use ferrox-ui with nothing else. It's all your agent
needs; the rest is in the kit.

---

This project's UI is built with ferrox-ui, and its agent kit is in
`{{KIT}}/`. Before any UI work, read `{{KIT}}/core/AGENTS.md` and follow
it. For a job, follow its task file in `{{KIT}}/core/tasks/` (`setup`,
`screen`, `component`, `check`). Before saying UI work is done, run
`{{KIT}}/check/ferrox-check.sh` (or `ferrox-check.ps1`) on the app's folder
and fix what it reports.
