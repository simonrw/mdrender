## Why

Reading Markdown in a terminal should be pleasant without requiring an interactive pager or browser. Existing plain output often loses Markdown structure, while naive wrapping can split words in distracting places and make prose harder to scan.

## What Changes

- Add a Rust CLI that renders a Markdown file or stdin as styled terminal output.
- Support pipe-friendly usage where terminal output is rendered for TTYs, while redirected or piped output passes through the raw Markdown source by default.
- Preserve readable document structure for headings, paragraphs, lists, blockquotes, inline formatting, links, code spans, and code blocks.
- Highlight fenced code blocks when the language is known, with graceful plain-code fallback.
- Wrap prose using a readable content measure, natural word breaks, Unicode-aware display widths, and optional side padding on wide terminals.
- Explore a TeX-inspired line-breaking strategy that improves ragged right edges and allows conservative hyphenation when required.
- Provide explicit controls for output width, render mode, and color behavior.
- Keep the first version non-interactive: no alternate screen, scroll state, keyboard input, or TUI event loop.

## Capabilities

### New Capabilities

- `terminal-markdown-rendering`: Rendering Markdown as visually appealing, pipe-friendly terminal output with word-aware wrapping.

### Modified Capabilities

- None.

## Impact

- Adds a Rust project structure for the `mdrender` CLI.
- Introduces Markdown parsing, syntax highlighting, terminal width detection, terminal cell measurement, word-aware wrapping, CLI argument parsing, and styled stdout output dependencies.
- Establishes rendering behavior that must remain testable through deterministic width-based fixtures and a render assertion DSL.
