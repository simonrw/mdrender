## Purpose

Specify how `mdrender` renders Markdown as visually appealing, pipe-friendly terminal output with word-aware wrapping.

## Requirements

### Requirement: Render Markdown from a file or stdin

The system SHALL provide a CLI that reads Markdown from a single file argument, or from stdin when no file argument is provided.

#### Scenario: Render a file

- **WHEN** the user runs `mdrender README.md` with stdout attached to a terminal
- **THEN** the system reads `README.md`, renders its Markdown content to stdout, and exits successfully.

#### Scenario: Render stdin without file arguments

- **WHEN** Markdown content is piped into `mdrender --render always`
- **THEN** the system reads stdin, renders the Markdown content to stdout, and exits successfully.

### Requirement: Preserve pipe-friendly raw passthrough

The system SHALL pass through the original Markdown source when stdout is not a terminal and render mode is `auto`.

#### Scenario: Output is redirected in auto mode

- **WHEN** the user runs `mdrender README.md > out.md`
- **THEN** the system writes the raw contents of `README.md` to stdout without terminal styling, layout padding, or reflow.

#### Scenario: Output is piped in auto mode

- **WHEN** the user runs `mdrender README.md | other-command`
- **THEN** the system writes the raw contents of `README.md` to stdout and exits after processing the input.

#### Scenario: Output is force-rendered for a pipe

- **WHEN** the user runs `mdrender --render always --color never --width 80 README.md | other-command`
- **THEN** the system writes rendered plain text output to stdout using an 80-column render viewport.

### Requirement: Control render viewport width

The system SHALL render using the detected terminal width by default and SHALL allow the user to override the render viewport width with `--width <columns>`.

#### Scenario: Width override controls wrapping

- **WHEN** the user runs `mdrender --render always --width 24 README.md`
- **THEN** prose is wrapped as if the render viewport is 24 columns.

#### Scenario: Width override is not clamped to terminal width

- **WHEN** the user runs `mdrender --render always --width 120 README.md` inside a terminal that is narrower than 120 columns
- **THEN** the system renders using a 120-column viewport anyway.

#### Scenario: Width override enables deterministic tests

- **WHEN** the same Markdown input is rendered twice with the same `--render always --width` value and color disabled
- **THEN** the plain text layout is identical across both runs.

### Requirement: Apply responsive readable layout

The system SHALL compute a content column from the render viewport and SHALL add side padding on wide terminals while using the full viewport on narrow terminals.

#### Scenario: Narrow viewport has no side padding

- **WHEN** content is rendered with a viewport width below 104 columns
- **THEN** the content column uses the full viewport width and no left or right presentation padding is added.

#### Scenario: Wide viewport uses readable measure

- **WHEN** content is rendered with a viewport width of at least 104 columns
- **THEN** prose is laid out in an 88-column content column centered with left and right presentation padding.

### Requirement: Control render and color modes

The system SHALL support `--render auto`, `--render always`, `--color auto`, `--color always`, and `--color never`.

#### Scenario: Disable color for rendered output

- **WHEN** the user runs `mdrender --render always --color never README.md`
- **THEN** the system emits rendered output with no ANSI color or style escape sequences.

#### Scenario: Force color for rendered output

- **WHEN** the user runs `mdrender --render always --color always README.md`
- **THEN** the system emits ANSI styling for supported Markdown elements even when stdout is not detected as a terminal.

### Requirement: Render common Markdown structure

The system SHALL render headings, paragraphs, emphasis, strong emphasis, inline code, links, lists, task list markers, blockquotes, thematic breaks, tables, and fenced or indented code blocks with readable terminal styling or spacing.

#### Scenario: Render inline formatting

- **WHEN** the input contains emphasis, strong emphasis, inline code, and links in a paragraph
- **THEN** the output preserves the text content and applies distinct terminal styling when color or style output is enabled.

#### Scenario: Render block structure

- **WHEN** the input contains headings, paragraphs, lists, blockquotes, thematic breaks, tables, and code blocks
- **THEN** the output uses spacing, indentation, prefixes, or styling to make each structure visually distinguishable.

### Requirement: Highlight fenced code blocks

The system SHALL apply syntax highlighting to fenced code blocks when the fence language is supported and color or style output is enabled.

#### Scenario: Supported fenced code language

- **WHEN** the input contains a fenced code block marked with a supported language
- **THEN** the system renders the code block with syntax-specific styled spans while preserving the source text.

#### Scenario: Unsupported fenced code language

- **WHEN** the input contains a fenced code block marked with an unsupported language
- **THEN** the system renders the code block as plain code without failing.

### Requirement: Wrap prose with optimal ragged line breaking

The system SHALL wrap normal prose using terminal display columns, natural break opportunities, and an optimal ragged layout algorithm that avoids poor local greedy breaks.

#### Scenario: Paragraph wraps at word boundaries

- **WHEN** a paragraph containing `This paragraph contains natural word breaks.` is rendered with a width that cannot fit the full sentence
- **THEN** each line breaks between words and no word is split across lines.

#### Scenario: Better ragged layout beats greedy wrapping

- **WHEN** a paragraph has multiple valid wrapping choices for the configured content width
- **THEN** the system chooses the line breaks with lower total raggedness cost rather than always filling each line greedily.

#### Scenario: Conservative hyphenation is used

- **WHEN** hyphenation produces a materially better layout or is needed to fit a word within the content width
- **THEN** the system uses a hyphenation point and avoids consecutive hyphenated lines when a comparable alternative exists.

#### Scenario: Long token uses last-resort splitting

- **WHEN** a single token is longer than the available content width and no useful hyphenation point exists
- **THEN** the system splits the token only as needed to make progress and keep output bounded by the configured content width.

#### Scenario: Unicode width is respected

- **WHEN** prose contains non-ASCII characters whose terminal display width differs from byte length
- **THEN** wrapping decisions use terminal display width rather than byte length.

### Requirement: Preserve wrapped indentation for nested structures

The system SHALL align wrapped continuation lines under the content start for lists and blockquotes.

#### Scenario: List item continuation aligns with item text

- **WHEN** a list item wraps across multiple lines
- **THEN** continuation lines align with the list item text rather than the list marker.

#### Scenario: Blockquote continuation preserves quote prefix

- **WHEN** a blockquote paragraph wraps across multiple lines
- **THEN** each wrapped line remains visually associated with the blockquote.

### Requirement: Preserve code block content

The system SHALL preserve code block text content and SHALL NOT reflow code blocks as prose.

#### Scenario: Code block lines are not word-wrapped as prose

- **WHEN** a fenced code block contains whitespace-sensitive lines
- **THEN** the system preserves the line structure instead of reflowing words into prose paragraphs.

#### Scenario: Inline code participates conservatively in wrapping

- **WHEN** inline code appears inside a paragraph
- **THEN** the system prefers to keep the inline code span intact and splits it only if it is longer than the available content width.

### Requirement: Provide structured render assertions for tests

The system SHALL provide test support for rendering arbitrary Markdown into structured lines that expose text, indentation, and semantic style spans before ANSI emission.

#### Scenario: Assert wrapped text without ANSI snapshots

- **WHEN** a test renders Markdown with a fixed viewport width and color disabled
- **THEN** the test can assert line text, indentation, absence of word splits, and semantic style spans through structured output.

#### Scenario: Assert ANSI emission separately

- **WHEN** a test renders structured lines with color enabled
- **THEN** the test can assert ANSI style emission independently from Markdown parsing and line wrapping.

### Requirement: Report input errors clearly

The system SHALL report file read errors to stderr and exit with a non-zero status when the requested input file cannot be read.

#### Scenario: Missing file

- **WHEN** the user runs `mdrender missing.md`
- **THEN** the system writes a clear error message to stderr and exits with a non-zero status.
