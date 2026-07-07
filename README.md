# mdrender

`mdr` is a pipe-friendly terminal Markdown renderer.

```sh
mdr README.md
mdr --pager never README.md
mdr --render always --color never --pager never --width 80 README.md
```

When stdout is a terminal, `mdr` renders Markdown through `less -R` by default so longer documents do not fill terminal scrollback. Use `--pager never` to write rendered output directly to stdout.

In `--render auto` mode, non-TTY stdout receives the original Markdown source and no pager is launched. Use `--render always --pager never` for deterministic rendered output in tests or pipelines.

## Releases

Maintainers publish binary releases by pushing a semantic version tag such as `v0.1.0`. The release workflow builds `mdr` with the committed lockfile, packages the supported binaries, creates or updates the matching GitHub Release, and uploads a `SHA256SUMS` checksum manifest.

Release archives are named `mdr-<tag>-<target>.<extension>`. The initial supported targets are:

- `x86_64-unknown-linux-gnu` as `.tar.gz`
- `x86_64-apple-darwin` as `.tar.gz`
- `aarch64-apple-darwin` as `.tar.gz`
- `x86_64-pc-windows-msvc` as `.zip`

GitHub Releases contain binary archives only. The workflow does not publish package-registry artifacts such as crates.io, Homebrew, npm, winget, apt, or similar packages.
