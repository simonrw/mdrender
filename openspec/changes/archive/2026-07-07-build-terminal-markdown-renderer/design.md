## Context

`mdrender` starts as an empty repo-local project with an OpenSpec plan. The intended product is a pretty `cat` replacement for Markdown: it reads one Markdown file or stdin, renders styled output for interactive terminal viewing, and exits. The most important quality bar is readable typography in a terminal: prose must use a sensible line length, wrap on natural word boundaries, preserve Markdown structure, and avoid ugly mid-word breaks except when a word cannot otherwise fit.

The tool should feel natural in shell usage. When stdout is a TTY, it should render Markdown. When stdout is redirected or piped, it should pass through the raw Markdown source by default so downstream commands receive the original document rather than a terminal-specific presentation.

## Goals / Non-Goals

**Goals:**

- Provide a Rust CLI named `mdrender` that accepts one Markdown file or stdin.
- Render Markdown with visually distinct terminal styling for common document structure.
- Pass raw Markdown through by default when stdout is not a TTY.
- Apply a responsive terminal layout: no side padding on narrow terminals, readable content width with left and right padding on wide terminals.
- Wrap normal prose with a TeX-inspired optimal ragged layout that uses terminal cell measurement, natural breakpoints, and conservative hyphenation.
- Preserve indentation and continuation alignment for lists and blockquotes.
- Apply syntax highlighting to fenced code blocks when the language is supported.
- Support deterministic fixture tests by allowing width and color behavior to be controlled explicitly.
- Provide a render test DSL that can assert text, indentation, wrapping, and style spans without relying only on brittle ANSI snapshots.

**Non-Goals:**

- No interactive pager, alternate screen, scroll state, keyboard input, or `ratatui` event loop in the first version.
- No full Markdown editor or preview server.
- No attempt to perfectly reproduce browser Markdown layout.
- No streaming render-while-parse pipeline in the first version; revisit this in phase 2 if large-document performance becomes a problem.
- No terminal-specific rendering hacks that make raw passthrough or captured output surprising.

## Decisions

### Build a stdout renderer, not a TUI

Use a direct parse-layout-render pipeline that writes to stdout for TTY output. Avoid `ratatui` for the initial implementation because `ratatui` is optimized for stateful terminal applications, while this change is a stateless command that should compose with shell usage.

Alternatives considered:

- `ratatui`: useful later for an interactive viewer, but adds event loop and screen model complexity that does not serve a `cat` replacement.
- Shelling out to an existing renderer: faster initially, but prevents owning the wrapping, padding, and passthrough behavior that defines the tool.

### Pass raw Markdown through for non-TTY stdout

By default, `mdrender README.md | other-command` and `mdrender README.md > out.md` should emit the original Markdown source, not styled or reflowed terminal output. Rendering is a terminal presentation. Raw passthrough makes the command safe to place in pipelines without corrupting Markdown semantics.

The exception is an explicit force-render mode. Add `--render auto|always`, defaulting to `auto`. In `auto` mode the tool renders only for TTY stdout and passes raw Markdown through for non-TTY stdout. In `always` mode the tool renders even when stdout is redirected or piped. Keep `--color` only about ANSI styling so tests can use `--render always --color never --width <columns>`.

### Parse Markdown with `pulldown-cmark`

Use `pulldown-cmark` to turn Markdown into structured events. Enable CommonMark-compatible parsing plus practical extensions such as tables, task lists, strikethrough, and footnotes where supported by the parser.

Alternatives considered:

- Ad hoc line parsing: too fragile for nested inline formatting, links, lists, and code spans.
- A higher-level terminal Markdown crate: attractive for speed, but likely hides the wrapping and layout details this tool needs to control.

### Separate semantic parsing from terminal layout

Convert parser events into a small internal render model before writing ANSI output. The model should distinguish block-level structure from inline styled text runs.

```text
Markdown source
    |
    v
pulldown-cmark events
    |
    v
Block model + inline styled runs
    |
    v
Responsive viewport/content layout
    |
    v
Styled stdout or raw passthrough
```

This keeps wrapping testable without requiring terminal capture and makes it easier to add support for new Markdown constructs incrementally.

### Use responsive terminal layout with a readable measure

Treat terminal width as a viewport and compute a content column within it. The first implementation should use a small web-style breakpoint system:

- `viewport_width` comes from `--width <columns>` when provided, otherwise terminal detection.
- `--width` is authoritative and MUST NOT be clamped to the real terminal width. This keeps tests deterministic, even if the result looks garbled in a narrower real terminal.
- For `viewport_width < 104`, use the full viewport and no side padding.
- For `viewport_width >= 104`, cap prose content at `88` columns and center it with left and right padding.
- Padding is part of terminal presentation, not Markdown content.

This keeps 80-column terminals dense and useful while making very wide terminals feel more like a readable article instead of a single huge line.

### Measure width in terminal cells

Use `unicode-width` for display-width calculations. Byte count and Rust string length are both wrong for terminal layout because emoji, CJK characters, combining marks, and ANSI escape sequences do not map cleanly to bytes or scalar values.

Alternatives considered:

- `str::len`: breaks for Unicode and would recreate the core class of visual bugs this tool is meant to avoid.
- Assuming ASCII input: simpler but too brittle for real Markdown documents.

### Wrap prose with an optimal ragged line breaker

Use a paragraph layout algorithm inspired by TeX's Knuth-Plass algorithm and Typst's optimized line breaking, but adapt it to terminal output:

- Terminal output is ragged-right, not justified, so we should minimize bad line breaks rather than stretch spaces.
- Candidate breakpoints come from whitespace, Unicode line break opportunities, punctuation-aware URL breaks, and optional hyphenation points.
- The cost function should penalize large leftover space, short final runts, hyphenation, consecutive hyphenated lines, splitting inline code, and breaking immediately after weak words.
- The algorithm should use dynamic programming over breakpoints for normal prose blocks.
- If a token cannot fit the content width, the algorithm may split it as a last resort.

Typst is useful prior art because its Rust layout engine exposes both simple first-fit and optimized Knuth-Plass-style modes. We should borrow the shape of the idea, not the whole layout engine. References for implementation research:

- https://en.wikipedia.org/wiki/Knuth%E2%80%93Plass_line-breaking_algorithm
- https://raw.githubusercontent.com/typst/typst/main/crates/typst-layout/src/inline/linebreak.rs
- https://raw.githubusercontent.com/typst/typst/main/crates/typst-layout/src/inline/mod.rs

### Highlight fenced code blocks with Tree-sitter where practical

Syntax highlighting is in scope for the first version. Use `tree-sitter-highlight` plus a curated set of language grammars for common README content, then gracefully fall back to plain styled code blocks when a language is unknown or highlighting fails.

The first supported language set should be small and useful: Rust, TOML, JSON, YAML, shell, JavaScript, TypeScript, and Markdown. The render model should store highlighted code as styled runs per source line so the terminal renderer can use the same styling path as prose.

Alternatives considered:

- `syntect`: mature and broad language coverage, but Tree-sitter gives parser-backed highlighting and a clearer path to structured code rendering later.
- No highlighting: simpler, but the user explicitly wants this in scope and code blocks are a major part of terminal Markdown reading.

### Use explicit CLI controls for deterministic behavior

The CLI should expose:

- `--width <columns>` to override terminal detection and define the render viewport.
- `--render auto|always` to control whether non-TTY stdout receives raw Markdown or rendered presentation.
- `--color auto|always|never` to make colored output predictable in tests and pipelines.
- A single optional file argument, with stdin used when no file is provided.

Terminal width detection should be a convenience, not a hidden dependency for the renderer core.

### Keep color styling separate from layout

Represent style semantically during layout, then decide whether to emit ANSI at the final rendering boundary. ANSI escape sequences must not contribute to wrapping width.

This also allows tests to inspect plain text output while separate tests verify style emission.

### Build a render assertion DSL

Golden snapshots are useful, but they become noisy when ANSI escapes, padding, and wrapping change. Add a small test DSL for rendering arbitrary Markdown into a structured output model and asserting properties such as:

```text
render(markdown, width: 40, color: never)
  .line(0).text("# Title")
  .line(2).starts_with("  - ")
  .line(3).indent(4)
  .no_word_splits()
  .span("important").style(strong)
```

The DSL should operate on structured rendered lines before ANSI emission where possible, with separate tests for the ANSI boundary.

### Defer streaming performance work

The first version should parse the whole input into a render model before layout. That is simpler, easier to test, and lets the line breaker see complete paragraphs. Rendering while parsing is attractive for very large files, but it complicates block-level layout, code highlighting, and optimal paragraph breaking. Treat streaming as phase 2 if benchmarks show real pain.

## Risks / Trade-offs

- Optimal ragged line breaking may be slower than greedy wrapping -> Keep it scoped to prose paragraphs, add benchmarks, and defer streaming until performance data says it matters.
- Hyphenation can look surprising in technical prose -> Use conservative penalties and prefer hyphenation only when it materially improves or saves a line.
- Tables can become unreadable in narrow terminals -> Start with a simple table strategy that preserves cells and degrades predictably, then improve after real examples.
- Wide-terminal padding can feel wasteful for code-heavy documents -> Apply readable measure primarily to prose and make code block behavior explicit in tests.
- Long URLs and long code spans may exceed width or require awkward splitting -> Define punctuation-aware URL breaks and a last-resort split behavior for tokens longer than available width.
- Styling can make snapshot tests noisy -> Support `--color never`, expose structured render assertions, and test ANSI emission separately.
- Tree-sitter language coverage can add dependency weight -> Start with a curated language set and plain fallback for unsupported fences.
