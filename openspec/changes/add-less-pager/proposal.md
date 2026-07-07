## Why

Reading rendered Markdown in a terminal should not force users to scroll their terminal history back after viewing longer documents. `mdr` already renders comfortable terminal output, and paging that output makes it feel like a document reader while keeping shell pipelines predictable.

## What Changes

- Add pager-backed interactive viewing for rendered terminal output.
- Use `less -R` as the hard-coded pager command for the first version so ANSI styling is preserved and custom `$PAGER` commands do not accidentally re-render Markdown.
- Add an explicit pager control, `--pager auto|always|never`, defaulting to `auto`.
- Preserve the existing pipe-friendly behavior: non-TTY stdout in `--render auto` continues to receive raw Markdown and does not invoke a pager.
- Allow users to opt out with `--pager never` when they want direct rendered stdout in a terminal.

## Capabilities

### New Capabilities

### Modified Capabilities

- `terminal-markdown-rendering`: Rendered TTY output can be viewed through a less-backed pager while raw passthrough for redirected and piped output remains unchanged.

## Impact

- CLI argument parsing gains a pager mode option.
- The CLI output boundary changes from always writing rendered TTY output directly to stdout to optionally piping rendered output into `less -R`.
- Integration tests need coverage for pager mode selection without relying on an interactive terminal.
- Documentation examples should describe pager defaults and the direct-output opt-out.
