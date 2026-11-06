# Help topics in Markdown: the format

The contract between an app's help files and `ferrox-help-build`. The
format is a deliberately small subset of Markdown: every construct maps to
one `HelpBlock`, and the build rejects anything else, so a mistake fails
loudly instead of rendering oddly. Every problem is reported as
`file:line`, all at once.

How the files become code is in [help.md](help.md#topics-from-markdown-ferrox-help-build)
and [the repo README](../../README.md#your-help-topics).

## The directory

- `ORDER` lists the area files, one per line, in the order a guide shows
  them. Inside a file, topics are read in document order.
- Every other `.md` file in the directory must be listed in `ORDER`, except
  `README.md`, which is left for notes to writers (conventions, who checks
  what).

## Area heading

Each file starts with exactly one level-1 heading, the area title shown as
a group in the guide:

```markdown
# Recipes
```

An optional single paragraph may follow it, before the first topic. It
introduces the area in the guide and is not part of any topic.

## Topics

A level-2 heading with an id starts a topic. The topic runs until the next
level-2 heading or the end of the file.

```markdown
## Searching recipes {#recipe-search}
```

Id rules:

- Lowercase letters, digits and hyphens only, starting with a letter:
  `^[a-z][a-z0-9-]*$`.
- Unique across **all** files, not just one.
- `topics` is reserved: the generated registry is `TOPICS`.
- The generated static is the id upper-snake-cased: `recipe-search` →
  `RECIPE_SEARCH`.
- Stable: an id is a public key. `help=` props, guide links (`/help#id`),
  an assistant's tools and saved conversations all use it. Rename the title
  freely, but never the id. To retire a topic, delete it and fix every
  reference; the build finds them.

No level-3 or deeper headings. If a topic needs sub-headings, it is two
topics.

## Blocks

Inside a topic, blocks are separated by blank lines.

**Paragraph.** One or more lines of plain text, joined with single spaces.

**List.** Lines that start with `- `. A line indented by two spaces
continues the item above it. Lists are one level deep, with no numbering.
If the order matters, say so in the text ("Do these in order:").

```markdown
- Title: matches anywhere in the name, ignoring case.
- Tags: a recipe matches if it has any of the tags you picked.
```

**Table.** A pipe table. The first row holds the column names and the
second row is the separator. Every row has the same number of cells.

```markdown
| You have | Use instead of 1 clove |
|---|---|
| Dried minced garlic | ½ teaspoon |
```

**Related.** An optional last line of a topic, naming other topics by id:

```markdown
Related: #make-it, #servings-and-scaling
```

The ids must exist. A topic can't list itself.

**Comments.** `<!-- ... -->` on lines of their own are notes for writers
and are dropped.

## The first paragraph

The first block of every topic must be a paragraph that stands on its own,
in one to three sentences. The popover shows it first, a guide search can
list it, and an assistant may quote it alone. Put the detail after it.

## Text

Text is plain. `HelpBlock` has no inline styling, so:

- no `**bold**`, `_italics_`, backtick code, `[links](...)` or HTML tags;
  the build rejects `**`, `` ` ``, `](` and `<tag>`-shaped text;
- quote UI labels exactly as the screen shows them, in double quotes:
  tap "Make It";
- use the real characters the UI uses: ½ ⅓ ¼, ✓, ▶, —.

## For an assistant

The JSON output (`write_json`) gives each topic as its id, title, blocks,
related ids and guide link. Because the first paragraph stands alone, a
summary can start from it.
