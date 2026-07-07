## Context

The package is currently named `mdrender`, and Cargo therefore exposes the binary as `mdrender` by default. The project wants the installed executable to be called `mdr` while keeping the existing package and library naming stable unless a code-level conflict appears during implementation.

The user-visible contract affected by this change is the command name in CLI invocation, documentation, and integration tests.

## Goals / Non-Goals

**Goals:**

- Make Cargo build and expose a binary named `mdr`.
- Update CLI tests to locate and invoke the `mdr` binary.
- Update user-facing documentation and any repository-local executable references.
- Keep the behavior, options, rendering output, and package metadata otherwise unchanged.

**Non-Goals:**

- Renaming the package, library crate, modules, or repository.
- Providing a compatibility executable named `mdrender`.
- Changing render behavior, CLI flags, output formatting, or dependencies.

## Decisions

- Add explicit Cargo binary metadata for `mdr` pointing at the existing `src/main.rs`. This changes the executable name without moving code or renaming the package.
- Update `assert_cmd::Command::cargo_bin` calls from `mdrender` to `mdr` so integration tests validate the installed command name.
- Replace documentation examples that show `mdrender` as the command with `mdr`, while only changing the project title if desired for user clarity.

## Risks / Trade-offs

- Existing users who call `mdrender` will see command-not-found after upgrading - the proposal marks this as a breaking CLI rename.
- Some references could be missed in docs or scripts - implementation should search the repository for `mdrender` and review each occurrence before finishing.
- Cargo package and binary names will differ - this is intentional, but release notes should make clear that the crate/package remains `mdrender` while the executable is `mdr`.
