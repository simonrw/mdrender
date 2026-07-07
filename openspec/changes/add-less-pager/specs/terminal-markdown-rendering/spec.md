## ADDED Requirements

### Requirement: Control pager behavior

The system SHALL support `--pager auto`, `--pager always`, and `--pager never` to control whether rendered terminal output is shown through a pager.

#### Scenario: Auto pager uses less for interactive rendered output

- **WHEN** the user runs `mdr README.md` with stdout attached to a terminal
- **THEN** the system renders the Markdown and sends the rendered output to `less -R`.

#### Scenario: Disable pager for direct terminal output

- **WHEN** the user runs `mdr --pager never README.md` with stdout attached to a terminal
- **THEN** the system renders the Markdown and writes the rendered output directly to stdout.

#### Scenario: Pager is not used for raw passthrough

- **WHEN** the user runs `mdr README.md | other-command` in `--render auto` mode
- **THEN** the system writes the raw contents of `README.md` to stdout without launching a pager.

#### Scenario: Forced pager reports launch failure

- **WHEN** the user runs `mdr --pager always README.md` and `less` cannot be launched
- **THEN** the system writes a clear pager error to stderr and exits with a non-zero status.

### Requirement: Use a fixed less pager command

The system SHALL use `less -R` as the pager command and SHALL NOT read `$PAGER` for pager selection.

#### Scenario: PAGER is ignored

- **WHEN** the user runs `PAGER=bat mdr README.md` with stdout attached to a terminal
- **THEN** the system renders the Markdown and sends the rendered output to `less -R` rather than `bat`.

## MODIFIED Requirements

### Requirement: Render Markdown from a file or stdin

The system SHALL provide a CLI named `mdr` that reads Markdown from a single file argument, or from stdin when no file argument is provided.

#### Scenario: Render a file

- **WHEN** the user runs `mdr README.md` with stdout attached to a terminal
- **THEN** the system reads `README.md`, renders its Markdown content, and displays the rendered output through the configured pager behavior.

#### Scenario: Render stdin without file arguments

- **WHEN** Markdown content is piped into `mdr --render always`
- **THEN** the system reads stdin, renders the Markdown content, and exits successfully.
