use mdrender::layout::Viewport;
use mdrender::render::{render_markdown, RenderOptions};

#[test]
fn unsupported_code_falls_back_to_plain_text() {
    let out = render_markdown(
        "```weird\nx = 1\n```",
        RenderOptions {
            viewport: Viewport { width: 20 },
            color: false,
        },
    );
    assert!(out.contains("x = 1"));
}

#[test]
fn supported_code_can_emit_ansi() {
    let out = render_markdown(
        "```rust\nfn main() {}\n```",
        RenderOptions {
            viewport: Viewport { width: 40 },
            color: true,
        },
    );
    assert!(out.contains("\u{1b}"));
}

#[test]
fn thematic_break_and_table_render() {
    let out = render_markdown(
        "| A | B |\n| - | - |\n| 1 | 2 |\n\n---",
        RenderOptions {
            viewport: Viewport { width: 40 },
            color: false,
        },
    );
    assert!(out.contains("A | B"));
    assert!(out.contains("------------------------"));
}
