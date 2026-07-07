## ADDED Requirements

### Requirement: Validate changes in CI

The repository SHALL run automated CI checks for pull requests and default-branch pushes.

#### Scenario: Pull request validation runs

- **WHEN** a pull request targets the default branch
- **THEN** CI verifies formatting, linting, tests, and a release-mode build for the `mdr` binary.

#### Scenario: Default branch validation runs

- **WHEN** commits are pushed to the default branch
- **THEN** CI verifies formatting, linting, tests, and a release-mode build for the `mdr` binary.

### Requirement: Build release binaries from version tags

The repository SHALL build distributable `mdr` binaries when a version release tag is pushed.

#### Scenario: Version tag starts release build

- **WHEN** a maintainer pushes a release tag matching the documented version tag format
- **THEN** the release workflow builds `mdr` with `cargo build --locked --release --bin mdr` for every supported release target.

#### Scenario: Supported release targets are built

- **WHEN** the release workflow runs for a version tag
- **THEN** it produces binaries for `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, and `x86_64-pc-windows-msvc`.

### Requirement: Package release artifacts

The repository SHALL package each release binary as a platform-specific archive with a stable name.

#### Scenario: Unix artifacts use tar archives

- **WHEN** the release workflow packages Linux or macOS binaries for tag `v0.1.0`
- **THEN** it creates `.tar.gz` archives named with the project, tag, and target triple.

#### Scenario: Windows artifacts use zip archives

- **WHEN** the release workflow packages the Windows binary for tag `v0.1.0`
- **THEN** it creates a `.zip` archive named with the project, tag, and target triple.

### Requirement: Publish GitHub Releases

The repository SHALL publish packaged release artifacts to a GitHub Release for the triggering version tag.

#### Scenario: Release assets are uploaded

- **WHEN** all supported target archives are built successfully for a version tag
- **THEN** the release workflow creates or updates the GitHub Release for that tag and uploads every archive.

#### Scenario: Failed builds do not publish incomplete releases

- **WHEN** any supported target build or packaging step fails
- **THEN** the release workflow fails without publishing a successful GitHub Release containing a partial artifact set.

### Requirement: Provide release checksums

The repository SHALL publish SHA-256 checksums for release archives.

#### Scenario: Checksum manifest is uploaded

- **WHEN** the release workflow uploads release archives
- **THEN** it also uploads a `SHA256SUMS` file that includes every release archive.

### Requirement: Document the release process

The repository SHALL document how maintainers publish releases and which artifacts users can expect.

#### Scenario: Maintainer release instructions are available

- **WHEN** a maintainer needs to publish a new release
- **THEN** repository documentation states the version tag format and the high-level release workflow.

#### Scenario: Supported artifacts are documented

- **WHEN** a user inspects repository documentation
- **THEN** the documentation lists the supported release target triples and archive naming convention.
