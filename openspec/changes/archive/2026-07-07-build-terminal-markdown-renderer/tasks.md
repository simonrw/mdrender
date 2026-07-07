## 1. Rust Project Setup

- [x] 1.1 Create the Rust crate structure for the `mdrender` binary.
- [x] 1.2 Add CLI, Markdown parsing, terminal width, Unicode width, Unicode line breaking, hyphenation, Tree-sitter highlighting, ANSI styling, and test assertion dependencies.
- [x] 1.3 Add baseline commands for formatting, linting, tests, and benchmarks using the repo's chosen Rust toolchain.

## 2. CLI Input and Output Modes

- [x] 2.1 Implement CLI parsing for an optional file argument, `--width <columns>`, `--render auto|always`, and `--color auto|always|never`.
- [x] 2.2 Implement input loading from a single file or stdin when no file is provided.
- [x] 2.3 Implement render mode selection so TTY stdout renders by default and non-TTY stdout passes raw Markdown through in `--render auto`.
- [x] 2.4 Implement forced rendering for non-TTY stdout with `--render always`.
- [x] 2.5 Report unreadable input files to stderr and return a non-zero exit status.
- [x] 2.6 Add CLI tests for file input, stdin input, raw passthrough, forced rendering, width override, color modes, and missing-file errors.

## 3. Markdown Parsing and Render Model

- [x] 3.1 Build a parser module around `pulldown-cmark` with the selected Markdown extension flags.
- [x] 3.2 Define an internal render model for block elements, inline styled runs, links, list metadata, blockquotes, code blocks, tables, thematic breaks, and syntax-highlighted code spans.
- [x] 3.3 Convert parser events into the render model while preserving text content and semantic style information.
- [x] 3.4 Add parser/model tests for headings, paragraphs, emphasis, strong emphasis, inline code, links, lists, task lists, blockquotes, tables, thematic breaks, and code blocks.

## 4. Responsive Terminal Layout

- [x] 4.1 Implement terminal cell width measurement using Unicode display width rather than byte length.
- [x] 4.2 Implement viewport resolution from detected terminal width or authoritative `--width` override without clamping to the real terminal.
- [x] 4.3 Implement content column calculation with no side padding below 104 columns and centered 88-column prose content at 104 columns or wider.
- [x] 4.4 Preserve list and blockquote continuation alignment within the computed content column.
- [x] 4.5 Add deterministic layout tests for narrow viewport behavior, wide viewport padding, and width overrides larger than the real terminal.

## 5. Optimal Prose Wrapping

- [x] 5.1 Implement a breakpoint model for whitespace, Unicode line break opportunities, punctuation-aware URL breaks, optional hyphenation points, mandatory breaks, and last-resort token splits.
- [x] 5.2 Implement a TeX-inspired dynamic programming line breaker that minimizes raggedness cost for prose paragraphs.
- [x] 5.3 Add penalties for hyphenation, consecutive hyphenated lines, short final runts, splitting inline code, and weak-word line endings.
- [x] 5.4 Preserve style spans while wrapping inline runs across output lines.
- [x] 5.5 Preserve code block line structure without prose reflow.
- [x] 5.6 Add deterministic layout fixture tests for optimal-vs-greedy wrapping, natural word breaks, conservative hyphenation, long tokens, Unicode width, inline code, list continuation, blockquote continuation, and code blocks.

## 6. Syntax Highlighting and Terminal Rendering

- [x] 6.1 Implement Tree-sitter highlighting for a curated first set of fenced code languages.
- [x] 6.2 Implement plain code fallback for unsupported languages or highlighter failures.
- [x] 6.3 Implement plain text rendering from laid-out lines.
- [x] 6.4 Implement ANSI styling emission for supported Markdown and syntax highlighting semantics.
- [x] 6.5 Ensure ANSI escape sequences are applied only at the final output boundary and do not affect layout width.
- [x] 6.6 Add rendering tests for color-disabled output, forced-color output, syntax-highlighted code, unsupported code fallback, and structure styling.

## 7. Test DSL and End-to-End Verification

- [x] 7.1 Build a render assertion DSL for arbitrary Markdown input, fixed viewport widths, line text assertions, indentation assertions, no-word-split assertions, and semantic style span assertions.
- [x] 7.2 Add ANSI-boundary tests that verify escape emission separately from Markdown parsing and line wrapping.
- [x] 7.3 Add integration tests that run `mdrender --render always` against fixture Markdown files at fixed widths.
- [x] 7.4 Add snapshot or golden tests for representative README-style documents with `--render always --color never`.
- [x] 7.5 Verify redirected and piped output paths pass raw Markdown through by default and do not require terminal input or alternate screen behavior.
- [x] 7.6 Add simple benchmarks for large Markdown documents and record streaming rendering as phase 2 work only if benchmarks show a problem.
- [x] 7.7 Run format, lint, and test commands and resolve any failures.
