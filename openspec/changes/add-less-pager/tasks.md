## 1. CLI Surface

- [x] 1.1 Add a `PagerMode` enum with `auto`, `always`, and `never` values.
- [x] 1.2 Add a `--pager <mode>` CLI option defaulting to `auto`.
- [x] 1.3 Update CLI help and README examples to describe default pager behavior and `--pager never`.

## 2. Pager Output Boundary

- [x] 2.1 Add a small output sink decision that chooses raw stdout, rendered stdout, or rendered pager output from render mode, pager mode, and stdout TTY state.
- [x] 2.2 Implement a pager writer that spawns `less -R`, writes rendered output to the child stdin, waits for completion, and reports spawn, write, and exit failures clearly.
- [x] 2.3 Keep `$PAGER` ignored so custom Markdown-aware pagers such as `bat` cannot re-render already-rendered output.
- [x] 2.4 Preserve current non-TTY `--render auto` behavior so redirected and piped stdout receive raw Markdown and never launch a pager.

## 3. Tests

- [x] 3.1 Add unit coverage for output sink selection across `--render auto|always`, `--pager auto|always|never`, and TTY/non-TTY stdout.
- [x] 3.2 Add integration or seam-level tests proving `--pager never` still writes rendered output directly to stdout.
- [x] 3.3 Add tests proving raw passthrough does not invoke pager behavior for non-TTY stdout.
- [x] 3.4 Add tests for pager launch failure returning a clear non-zero CLI error.
- [x] 3.5 Add a test or injectable command assertion proving the pager command is exactly `less -R` and does not use `$PAGER`.

## 4. Verification

- [x] 4.1 Run the full Rust test suite.
- [x] 4.2 Run OpenSpec validation for `add-less-pager`.
- [x] 4.3 Manually smoke-test `mdr README.md`, `mdr --pager never README.md`, and `mdr README.md | other-command` in a terminal when an interactive terminal is available.
