use mdrender::layout::{display_width, Viewport};
use mdrender::parser::parse_markdown;
use mdrender::render::layout_document;
use mdrender::test_support::render;

#[test]
fn narrow_viewport_has_no_padding() {
    let doc = layout_document(&parse_markdown("hello"), Viewport { width: 80 });
    assert_eq!(doc.lines[0].left_padding, 0);
}

#[test]
fn width_larger_than_terminal_is_used() {
    let doc = layout_document(&parse_markdown("hello"), Viewport { width: 120 });
    assert_eq!(doc.lines[0].left_padding, 16);
}

#[test]
fn list_continuation_aligns_with_text() {
    let doc = layout_document(
        &parse_markdown("- alpha beta gamma delta"),
        Viewport { width: 14 },
    );
    assert!(doc.lines[1].text().starts_with("  "));
}

#[test]
fn blockquote_continuation_keeps_prefix() {
    let doc = layout_document(
        &parse_markdown("> alpha beta gamma delta"),
        Viewport { width: 14 },
    );
    assert!(doc.lines.iter().all(|line| line.text().starts_with("> ")));
}

#[test]
fn unicode_width_is_respected() {
    assert_eq!(display_width("界"), 2);
    render("alpha 界界 beta", 9)
        .line_text(0, "alpha")
        .line_text(1, "界界 beta");
}

#[test]
fn structured_assertions_can_see_styles() {
    render("hello **strong** text", 80).span_has_strong("strong");
}
