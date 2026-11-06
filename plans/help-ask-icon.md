# Ask as a chat icon in the help header, and a header that stays

Status: approved 2026-11-06 (open questions as recommended). Piece 1 done: `message-circle` (`MESSAGE_CIRCLE`) in ferrox-icons; 14 icon e2e pass. Piece 2 done: the header (title and Ask icon) stays while `.help-popover__scroll` scrolls; the text button is gone. 10 new e2e tests, and the 144 existing help, guide and smoke tests still pass in Chromium and Firefox. Piece 3 done: each `HelpGuide` topic has the icon at the end of a `.help-guide__topic-header` row; 4 new e2e tests, 158 help, guide and smoke tests pass. Next: piece 4.

The idea comes from testing Rusty Skillet's assistant (rusty-skillet `plans/help-assistant.md`).

- **"Ask about this" is a large text button at the bottom** of the `?` popover and of each `HelpGuide` topic.
- **It is often out of reach.** In a long topic it sits below the fold. On a phone, the popover's height cap means scrolling to find it.
- **The app bar already asks with a small chat icon.** webapp-lib's `AskButton` draws one, inline, until ferrox-icons has it.

## Goal

- **Ask becomes a chat icon in the popover's header**, at the end of the title row. The header stays in place while the topic scrolls under it, so Ask is visible the moment the popover opens, on any screen.
- **One way to ask.** The text button at the bottom goes away. "Related" and "Full guide" stay at the end of the text, where they belong.
- **`HelpGuide` matches.** Each topic's Ask becomes the same icon, beside the topic's title.
- **ferrox-icons gains `message-circle`.** The popover, the guide and webapp-lib's `AskButton` all use it.

## Out of scope

- The `?` button itself, and placement (`anchor.rs`). The popover keeps its width and its height cap (`min(60vh, 28rem)`).
- Topics without an assistant: no `HelpAssistant` means no icon, and the header is just the title, as today.

## Design

### The popover

```
┌───────────────────────────────┐
│ The batch screen          [💬] │  header: title + Ask (fixed)
├───────────────────────────────┤
│ The batch screen is where…    │
│ - …                           │  scroll region: blocks,
│ Related                       │  related, Full guide
│ Working through the steps     │
│ Full guide                    │
└───────────────────────────────┘
```

- **Layout.** `.help-popover` becomes a flex column with `overflow: hidden`. `.help-popover__header` (new) holds the title and the icon. `.help-popover__scroll` (new) holds the body, the related list and the guide link, and takes `overflow-y: auto` and `min-height: 0`. Only the scroll region scrolls.
- **The icon** is an `.icon-btn` (the core classes, at the `?`'s size) with `message-circle`.
  - Its accessible name is `ask_label`, which still defaults to `"Ask about this"`, so existing tests that find the button by role and name keep working.
  - Its tooltip (`title`) names the topic: `"Ask about this: {title}"`.
  - It still closes the popover (`popovertargetaction="hide"`) and sends the same `HelpRequest`.
- **Focus** still goes to the popover on open (so a screen reader reads it) and to the title on a related swap. The icon is the first stop after that in tab order.
- **`.help-popover__actions`** now holds only "Full guide", inside the scroll region. It is hidden when there's no guide link.
- **CSS classes.**
  - New: `help-popover__header`, `help-popover__scroll`, `help-popover__ask` (now an icon button).
  - Unchanged: the rest.
  - The `help-popover__ask` change is called out in the CHANGELOG, because a theme that styled it as a text button needs a look.

### `HelpGuide`

- Each topic's header row has the title, then the same icon (`.help-guide__ask`, now an icon button) at the end.
- Its accessible name is `ask_label`, and its tooltip names the topic.
- The `HelpRequest` (component `"guide"`) is unchanged.

### ferrox-icons

- Add `message-circle.svg`, copied unchanged from Lucide at the pinned commit, and a `SOURCES` line: `message-circle message-circle`.
- The generated `MESSAGE_CIRCLE` and `ALL` follow from the build. Its tests (each icon draws, `ALL` complete) cover it.

## Pieces (each reviewed before the next)

1. **ferrox-icons: `message-circle`.** The icon, its `SOURCES` line, and the demo's `/icons` page, which lists `ALL` without any change.
2. **The popover: header with the Ask icon, and a scrolling region.**
   - CSS and markup in `help_button.rs` and `core/help.css`.
   - e2e tests in Chromium and Firefox:
     - with an assistant, the icon sits in the header beside the title;
     - without one, there's no icon;
     - a long topic scrolls under a header that stays, with the icon still in view;
     - a phone-width viewport shows the icon without scrolling;
     - the icon's name and tooltip;
     - asking still sends the topic shown, including after a related swap;
     - focus still goes to the popover on open, and Tab reaches the icon.
   - Update the existing Ask tests to the new layout.
3. **`HelpGuide`: the icon beside each topic's title.** e2e tests for it.
4. **Docs and release.**
   - `docs/help.md` (Ask, the layout, the classes), `docs/index.md` regenerated, the CHANGELOG.
   - The workspace version becomes **0.2.1**: additive, but with the class change noted. Tag `v0.2.1` after `scripts/release-check.sh`.
5. **Downstream, after the tag:**
   - webapp-lib `v1.0.0-alpha.11`: ferrox `v0.2.1`, and `AskButton` uses `MESSAGE_CIRCLE` instead of its inline SVG.
   - product-db-client `v0.4.5`.
   - Rusty Skillet bumps both, then `railway up` with the owner's OK.

## Open questions

1. **Keep a text "Ask about this" anywhere?** **Recommendation: no.** The icon's name and tooltip carry the words for anyone who needs them, and one way to ask is less noise.
2. **The icon in `HelpGuide`: beside each title, or once per page?** **Recommendation: beside each title.** A guide's Ask is about a topic, and the topic travels with the request.
