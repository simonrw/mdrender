use crate::ansi::paint_inline;
use crate::highlight::{code_inline, highlight_code};
use crate::layout::{display_width, Viewport};
use crate::model::{Block, Document, Inline};
use crate::parser::parse_markdown;
use crate::wrap::{wrap_inlines, StyledLine};

#[derive(Clone, Copy, Debug)]
pub struct RenderOptions {
    pub viewport: Viewport,
    pub color: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderedDocument {
    pub lines: Vec<RenderedLine>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderedLine {
    pub left_padding: usize,
    pub runs: Vec<Inline>,
}

impl RenderedLine {
    pub fn text(&self) -> String {
        format!(
            "{}{}",
            " ".repeat(self.left_padding),
            self.runs
                .iter()
                .map(|run| run.text.as_str())
                .collect::<String>()
        )
    }

    pub fn indent(&self) -> usize {
        self.text().chars().take_while(|ch| *ch == ' ').count()
    }
}

pub fn render_markdown(source: &str, options: RenderOptions) -> String {
    let doc = parse_markdown(source);
    let rendered = layout_document(&doc, options.viewport);
    emit(&rendered, options.color)
}

pub fn layout_document(doc: &Document, viewport: Viewport) -> RenderedDocument {
    let area = viewport.content_area();
    let mut lines = Vec::new();

    for (index, block) in doc.blocks.iter().enumerate() {
        if index > 0 {
            lines.push(RenderedLine {
                left_padding: 0,
                runs: Vec::new(),
            });
        }
        match block {
            Block::Heading { level, content } => {
                let prefix = "#".repeat(*level as usize) + " ";
                push_wrapped(
                    &mut lines,
                    area.left_padding,
                    area.width,
                    &prefix,
                    "",
                    content,
                );
            }
            Block::Paragraph(content) => {
                push_wrapped(&mut lines, area.left_padding, area.width, "", "", content)
            }
            Block::ListItem { content, .. } => {
                let text = content
                    .iter()
                    .map(|run| run.text.as_str())
                    .collect::<String>();
                let marker_len = if text.starts_with("- ") {
                    2
                } else {
                    text.find(' ').map(|n| n + 1).unwrap_or(2)
                };
                let (prefix, rest) = text.split_at(marker_len.min(text.len()));
                push_wrapped(
                    &mut lines,
                    area.left_padding,
                    area.width,
                    prefix,
                    &" ".repeat(display_width(prefix)),
                    &[Inline::plain(rest)],
                );
            }
            Block::BlockQuote(content) => push_wrapped(
                &mut lines,
                area.left_padding,
                area.width,
                "> ",
                "> ",
                content,
            ),
            Block::CodeBlock {
                language,
                lines: code_lines,
            } => {
                for line in code_lines {
                    let runs = highlight_code(language.as_deref(), line)
                        .into_iter()
                        .map(|(text, style)| code_inline(text, style))
                        .collect();
                    lines.push(RenderedLine {
                        left_padding: area.left_padding,
                        runs,
                    });
                }
            }
            Block::Table { rows } => {
                for row in rows {
                    let row_text = row
                        .iter()
                        .map(|cell| cell.iter().map(|run| run.text.as_str()).collect::<String>())
                        .collect::<Vec<_>>()
                        .join(" | ");
                    lines.push(RenderedLine {
                        left_padding: area.left_padding,
                        runs: vec![Inline::plain(row_text)],
                    });
                }
            }
            Block::ThematicBreak => lines.push(RenderedLine {
                left_padding: area.left_padding,
                runs: vec![Inline::plain("-".repeat(area.width.min(24)))],
            }),
        }
    }

    RenderedDocument { lines }
}

fn push_wrapped(
    lines: &mut Vec<RenderedLine>,
    left_padding: usize,
    width: usize,
    first_prefix: &str,
    continuation_prefix: &str,
    content: &[Inline],
) {
    let first_width = width.saturating_sub(display_width(first_prefix)).max(1);
    let wrapped = wrap_inlines(content, first_width);
    for (index, line) in wrapped.into_iter().enumerate() {
        let prefix = if index == 0 {
            first_prefix
        } else {
            continuation_prefix
        };
        let mut runs = Vec::new();
        if !prefix.is_empty() {
            runs.push(Inline::plain(prefix));
        }
        runs.extend(line.runs);
        lines.push(RenderedLine { left_padding, runs });
    }
}

pub fn emit(doc: &RenderedDocument, color: bool) -> String {
    let mut out = String::new();
    for line in &doc.lines {
        out.push_str(&" ".repeat(line.left_padding));
        for run in &line.runs {
            out.push_str(&paint_inline(run, color));
        }
        out.push('\n');
    }
    out
}

#[allow(dead_code)]
fn _keep_styled_line_used(_: StyledLine) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_wide_padding() {
        let doc = parse_markdown("hello world");
        let rendered = layout_document(&doc, Viewport { width: 120 });
        assert_eq!(rendered.lines[0].left_padding, 16);
    }

    #[test]
    fn preserves_code_lines() {
        let out = render_markdown(
            "```rust\nfn main() {\n}\n```",
            RenderOptions {
                viewport: Viewport { width: 10 },
                color: false,
            },
        );
        assert!(out.contains("fn main() {"));
    }
}
