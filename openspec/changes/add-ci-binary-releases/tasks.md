## 1. CI Workflow

- [x] 1.1 Confirm the repository default branch name for workflow `push` triggers.
- [x] 1.2 Create `.github/workflows/ci.yml` with pull request and default-branch push triggers.
- [x] 1.3 Add CI steps for Rust checkout, toolchain setup, `cargo fmt --check`, `cargo clippy --locked --all-targets --all-features -- -D warnings`, `cargo test --locked`, and `cargo build --locked --release --bin mdr`.
- [x] 1.4 Verify the CI workflow uses the committed lockfile and does not depend on local target directory assumptions.

## 2. Release Workflow

- [x] 2.1 Create `.github/workflows/release.yml` with a documented version tag trigger.
- [x] 2.2 Add a release build matrix for `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, and `x86_64-pc-windows-msvc`.
- [x] 2.3 Install required Rust targets in each matrix job and build `mdr` with `cargo build --locked --release --bin mdr`.
- [x] 2.4 Package Unix binaries as `.tar.gz` archives and the Windows binary as a `.zip` archive using names that include `mdr`, the tag, and the target triple.
- [x] 2.5 Upload packaged archives as workflow artifacts for a publish job.

## 3. Release Publishing

- [x] 3.1 Add a publish job that waits for all release build matrix jobs to finish successfully.
- [x] 3.2 Generate a `SHA256SUMS` file covering every release archive.
- [x] 3.3 Create or update the GitHub Release for the triggering tag using `GITHUB_TOKEN`.
- [x] 3.4 Upload every archive and `SHA256SUMS` to the GitHub Release.
- [x] 3.5 Ensure failed build, package, checksum, or upload steps fail the workflow rather than reporting a successful partial release.

## 4. Documentation

- [x] 4.1 Document the release tag format maintainers use to publish releases.
- [x] 4.2 Document supported target triples and archive naming conventions.
- [x] 4.3 Document that releases publish binaries only and do not publish package-registry artifacts.

## 5. Verification

- [x] 5.1 Run `cargo fmt --check`.
- [x] 5.2 Run `cargo clippy --locked --all-targets --all-features -- -D warnings`.
- [x] 5.3 Run `cargo test --locked`.
- [x] 5.4 Run `cargo build --locked --release --bin mdr`.
- [x] 5.5 Run OpenSpec validation for `add-ci-binary-releases`.
