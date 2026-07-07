## Context

`mdr` is a Rust CLI with a committed `Cargo.lock`, an explicit binary target named `mdr`, and no existing `.github/workflows` directory. Releases currently require maintainers or users to build locally, which makes installation less convenient and makes release artifacts inconsistent.

This change introduces GitHub Actions as the repository release boundary. CI should protect normal development, while a separate tag-triggered workflow should produce installable archives and attach them to a GitHub Release.

## Goals / Non-Goals

**Goals:**

- Run a CI workflow for pull requests and default-branch pushes that verifies formatting, linting, tests, and release-mode compilation.
- Publish GitHub Releases from version tags using `GITHUB_TOKEN`.
- Build archives containing the `mdr` executable for Linux x86_64, macOS x86_64, macOS arm64, and Windows x86_64.
- Publish checksum metadata so users can verify downloaded artifacts.
- Document the maintainer-facing release process and artifact naming convention.

**Non-Goals:**

- Do not publish packages to crates.io, Homebrew, npm, winget, apt, or other package registries.
- Do not add Linux arm64 or musl binaries in the initial release matrix.
- Do not introduce Docker, custom cross-compilation images, or a release orchestration service unless implementation proves the native GitHub Actions runners are insufficient.
- Do not change the crate name, binary name, CLI behavior, or rendering output.

## Decisions

### Split CI and release workflows

Add a CI workflow for normal validation and a release workflow for publishing artifacts. CI runs on pull requests and pushes to the default branch. Releases run only for version tags matching `v*`.

Alternatives considered:

- Single workflow with conditional jobs: fewer files, but release permissions and branching logic become harder to scan.
- Release-only workflow: simpler, but it leaves pull requests without automated validation.

### Use native runner builds for the initial target matrix

Build Linux x86_64 on `ubuntu-latest`, macOS x86_64 and macOS arm64 on `macos-latest` with the relevant Rust targets installed, and Windows x86_64 on `windows-latest`. Each build invokes `cargo build --locked --release --bin mdr`.

Alternatives considered:

- Use `cross` or Docker for all targets: useful for broader platform coverage, but it adds tooling and operational complexity before the project needs it.
- Only publish the host platform for each runner: simpler, but it would omit macOS arm64, which is a common installation target.

### Package artifacts as platform-specific archives

Name release archives with the project, version tag, and target triple, for example `mdr-v0.1.0-x86_64-unknown-linux-gnu.tar.gz`, `mdr-v0.1.0-aarch64-apple-darwin.tar.gz`, and `mdr-v0.1.0-x86_64-pc-windows-msvc.zip`. Unix archives contain `mdr`; Windows archives contain `mdr.exe`.

Alternatives considered:

- Upload raw binaries: easier to produce, but archives leave room for future files such as license text or README snippets without changing the distribution shape.
- Use generic OS names instead of target triples: friendlier at a glance, but target triples are precise and match Rust build output.

### Publish checksums next to release archives

Generate a `SHA256SUMS` file covering every uploaded archive and attach it to the same GitHub Release. The release workflow should fail if any expected archive or checksum generation step fails.

Alternatives considered:

- Per-artifact checksum files: convenient for single downloads, but a single checksum manifest is easier to inspect and update in one release job.
- No checksums: simpler, but users lose a basic verification path.

### Keep release automation declarative unless sharing becomes necessary

Prefer workflow shell steps for the initial implementation. Add an `xtask` release helper only if packaging logic becomes duplicated or awkward across operating systems.

Alternatives considered:

- Implement packaging through `xtask` immediately: more testable, but it increases Rust code surface for a small amount of workflow glue.

## Risks / Trade-offs

- GitHub runner images can change over time, which may affect installed tooling or target support. Mitigation: pin action major versions, use `rustup target add` explicitly, and keep workflow commands minimal.
- macOS cross-building for `aarch64-apple-darwin` depends on Apple targets available from the hosted runner. Mitigation: verify with the release build task before relying on the workflow.
- Release publishing can create incomplete releases if upload steps are not wired carefully. Mitigation: build all archives first, then create or update the GitHub Release only after required artifacts exist.
- Tag naming mistakes could publish unintended releases. Mitigation: restrict the workflow trigger to version-like `v*.*.*` tags if implementation prefers a stricter pattern.
- Windows packaging differs from Unix packaging. Mitigation: use target-specific packaging commands and assert final artifact names in CI steps.

## Migration Plan

1. Add CI and release workflows.
2. Add release documentation.
3. Validate CI locally where possible with `cargo fmt`, `cargo clippy`, `cargo test`, and release builds.
4. Verify OpenSpec validation for this change.
5. After merge, publish the first release by pushing a version tag such as `v0.1.0`.

Rollback is removing or disabling the workflows. Existing source builds and local Cargo workflows continue to work independently of GitHub Releases.

## Open Questions

- Should the release trigger use broad `v*` tags or strict semantic version tags such as `v*.*.*`?
- Should release archives include `README.md` and `LICENSE` if a license file is added later?
