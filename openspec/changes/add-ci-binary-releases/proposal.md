## Why

The project currently has no GitHub Actions release path, so users need to build `mdr` from source and maintainers have to create release artifacts manually. Adding CI-built binaries and GitHub Releases makes tagged versions easier to install, verify, and distribute.

## What Changes

- Add GitHub Actions CI for pull requests and default-branch pushes that checks formatting, linting, tests, and release builds.
- Add a tag-driven release workflow that builds `mdr` binaries for supported platforms, packages them with checksums, and publishes them to a GitHub Release.
- Document the supported release targets and the tag format maintainers use to publish releases.
- Keep the existing package name `mdrender` and executable name `mdr` unchanged.

## Capabilities

### New Capabilities

- `release-automation`: CI and GitHub Release automation for building, packaging, and publishing `mdr` binaries.

### Modified Capabilities

None.

## Impact

- New `.github/workflows/` configuration for CI and releases.
- Potential release helper scripts or `xtask` commands if packaging logic needs to be shared locally and in CI.
- README or maintainer documentation describing release tags, target triples, artifact names, and checksum files.
- Repository settings expectations for GitHub Actions permissions and release publishing with `GITHUB_TOKEN`.
