## MODIFIED Requirements

### Requirement: Render Markdown from a file or stdin

The system SHALL provide a CLI named `mdr` that reads Markdown from a single file argument, or from stdin when no file argument is provided.

#### Scenario: Render a file

- **WHEN** the user runs `mdr README.md` with stdout attached to a terminal
- **THEN** the system reads `README.md`, renders its Markdown content to stdout, and exits successfully.

#### Scenario: Render stdin without file arguments

- **WHEN** Markdown content is piped into `mdr --render always`
- **THEN** the system reads stdin, renders the Markdown content to stdout, and exits successfully.

### Requirement: Preserve pipe-friendly raw passthrough

The system SHALL pass through the original Markdown source when stdout is not a terminal and render mode is `auto`.

#### Scenario: Output is redirected in auto mode

- **WHEN** the user runs `mdr README.md > out.md`
- **THEN** the system writes the raw contents of `README.md` to stdout without terminal styling, layout padding, or reflow.

#### Scenario: Output is piped in auto mode

- **WHEN** the user runs `mdr README.md | other-command`
- **THEN** the system writes the raw contents of `README.md` to stdout and exits after processing the input.

#### Scenario: Output is force-rendered for a pipe

- **WHEN** the user runs `mdr --render always --color never --width 80 README.md | other-command`
- **THEN** the system writes rendered plain text output to stdout using an 80-column render viewport.

### Requirement: Control render viewport width

The system SHALL render using the detected terminal width by default and SHALL allow the user to override the render viewport width with `--width <columns>`.

#### Scenario: Width override controls wrapping

- **WHEN** the user runs `mdr --render always --width 24 README.md`
- **THEN** prose is wrapped as if the render viewport is 24 columns.

#### Scenario: Width override is not clamped to terminal width

- **WHEN** the user runs `mdr --render always --width 120 README.md` inside a terminal that is narrower than 120 columns
- **THEN** the system renders using a 120-column viewport anyway.

#### Scenario: Width override enables deterministic tests

- **WHEN** the same Markdown input is rendered twice with the same `--render always --width` value and color disabled
- **THEN** the plain text layout is identical across both runs.

### Requirement: Control render and color modes

The system SHALL support `--render auto`, `--render always`, `--color auto`, `--color always`, and `--color never`.

#### Scenario: Disable color for rendered output

- **WHEN** the user runs `mdr --render always --color never README.md`
- **THEN** the system emits rendered output with no ANSI color or style escape sequences.

#### Scenario: Force color for rendered output

- **WHEN** the user runs `mdr --render always --color always README.md`
- **THEN** the system emits ANSI styling for supported Markdown elements even when stdout is not detected as a terminal.

### Requirement: Report input errors clearly

The system SHALL report file read errors to stderr and exit with a non-zero status when the requested input file cannot be read.

#### Scenario: Missing file

- **WHEN** the user runs `mdr missing.md`
- **THEN** the system writes a clear error message to stderr and exits with a non-zero status.
