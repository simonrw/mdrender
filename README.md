# mdrender

`mdrender` is a pipe-friendly terminal Markdown renderer.

```sh
mdrender README.md
mdrender --render always --color never --width 80 README.md
```

In `--render auto` mode, non-TTY stdout receives the original Markdown source. Use `--render always` for deterministic rendered output in tests or pipelines.
