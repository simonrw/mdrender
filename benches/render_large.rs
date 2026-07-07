use mdrender::layout::Viewport;
use mdrender::render::{render_markdown, RenderOptions};
use std::time::Instant;

fn main() {
    let paragraph = "This is a representative paragraph with natural word breaks, inline code, Unicode like 界, and a long https://example.com/path/to/resource URL.\n\n";
    let input = paragraph.repeat(1_000);
    let start = Instant::now();
    let output = render_markdown(
        &input,
        RenderOptions {
            viewport: Viewport { width: 88 },
            color: false,
        },
    );
    assert!(!output.is_empty());
    eprintln!("render_large: {:?}", start.elapsed());
}
