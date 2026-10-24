# Task: find and use a component

DOCS is the folder `{{KIT}}/check/ferrox-docs` prints.

1. **Pick it** from the table in `AGENTS.md` (the same as
   `DOCS/choosing.md`). If two seem to fit, the table's wording decides;
   if none does, say so to the user rather than hand-making one.
2. **Read its page**, `DOCS/<component>.md`: its example (compiled, so it's
   right), its props, its behaviour. The page of its group
   (`DOCS/<group>.md`) has what its siblings share; the feature group must
   be on in the app's `Cargo.toml`.
3. **Check a prop you're unsure of** in `DOCS/index.md`: every component's
   props, with exact types and defaults.
4. **Follow the conventions** (`DOCS/conventions.md`): every callback is
   written `move || …`; every word is a prop (`label`, `title`, `*_label`);
   an overlay's `open` is an `RwSignal<bool>` that it closes itself; extra
   classes go in `class`, never `style`.
5. **Build.** A compile error about a prop means the docs say otherwise:
   read the page again rather than trying variations.
