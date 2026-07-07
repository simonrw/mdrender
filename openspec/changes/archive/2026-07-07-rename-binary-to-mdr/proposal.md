## Why

The command-line executable should use the short, memorable name `mdr` instead of inheriting the package name `mdrender`. This makes the tool quicker to invoke and gives the project a clearer public CLI identity.

## What Changes

- Rename the installed binary from `mdrender` to `mdr`.
- Update CLI integration tests, documentation, and any local helper references that invoke the binary by name.
- Preserve the Rust package and library crate name unless implementation reveals a strong reason to change them.
- **BREAKING**: Users who run `mdrender` directly will need to invoke `mdr` instead.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `terminal-markdown-rendering`: The terminal Markdown renderer is invoked through the `mdr` executable.

## Impact

- Cargo manifest binary configuration.
- CLI tests that resolve and run the package binary.
- README and usage documentation.
- Any repository scripts or support tooling that refer to `mdrender` as the executable name.
