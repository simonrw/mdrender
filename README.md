# mdrender

`mdr` is a pipe-friendly terminal Markdown renderer.

```sh
mdr README.md
mdr --pager never README.md
mdr --render always --color never --pager never --width 80 README.md
```

When stdout is a terminal, `mdr` renders Markdown through `less -R` by default so longer documents do not fill terminal scrollback. Use `--pager never` to write rendered output directly to stdout.

In `--render auto` mode, non-TTY stdout receives the original Markdown source and no pager is launched. Use `--render always --pager never` for deterministic rendered output in tests or pipelines.
