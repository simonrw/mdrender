## 1. Binary Configuration

- [x] 1.1 Add explicit Cargo binary metadata so `src/main.rs` builds an executable named `mdr`.
- [x] 1.2 Confirm the package and library crate remain named `mdrender`.

## 2. References and Tests

- [x] 2.1 Update CLI integration tests to invoke `mdr` with `Command::cargo_bin`.
- [x] 2.2 Search repository references to `mdrender` and update user-facing command examples to `mdr` where they refer to the executable.
- [x] 2.3 Keep non-executable references to the package or project name unchanged unless they would confuse users.

## 3. Verification

- [x] 3.1 Run the CLI test suite and confirm the `mdr` binary is built and invoked successfully.
- [x] 3.2 Run OpenSpec validation for `rename-binary-to-mdr`.
- [x] 3.3 Note the command rename as a breaking CLI change in the implementation summary.
