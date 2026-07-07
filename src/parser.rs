use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

use crate::model::{Block, Document, Inline, InlineStyle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BlockKind {
    Heading(u32),
    Paragraph,
    ListItem,
    BlockQuote,
}

#[derive(Default)]
struct InlineState {
    emphasis: bool,
    strong: bool,
    code: bool,
    strike: bool,
    link: Option<String>,
}

impl InlineState {
    fn style(&self) -> InlineStyle {
        InlineStyle {
            emphasis: self.emphasis,
            strong: self.strong,
            code: self.code,
            strike: self.strike,
            link: self.link.clone(),
        }
    }
}

pub fn parse_markdown(source: &str) -> Document {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(source, options);
    let mut doc = Document::default();
    let mut current_kind: Option<BlockKind> = None;
    let mut current_inlines = Vec::new();
    let mut style = InlineState::default();
    let mut list_stack: Vec<Option<u64>> = Vec::new();
    let mut current_code: Option<(Option<String>, Vec<String>)> = None;
    let mut table_rows: Option<Vec<Vec<Vec<Inline>>>> = None;
    let mut current_table_row: Option<Vec<Vec<Inline>>> = None;
    let mut current_table_cell: Option<Vec<Inline>> = None;
    let mut task_checked: Option<bool> = None;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    current_kind = Some(BlockKind::Heading(level as u32));
                    current_inlines.clear();
                }
                Tag::Paragraph => {
                    if current_kind.is_none() {
                        current_kind = Some(BlockKind::Paragraph);
                        current_inlines.clear();
                    }
                }
                Tag::Item => {
                    let marker = list_stack
                        .last()
                        .and_then(|start| *start)
                        .map(|n| format!("{n}."))
                        .unwrap_or_else(|| "-".to_string());
                    current_kind = Some(BlockKind::ListItem);
                    current_inlines = vec![Inline::plain(format!("{marker} "))];
                    task_checked = None;
                }
                Tag::BlockQuote(_) => {
                    current_kind = Some(BlockKind::BlockQuote);
                    current_inlines.clear();
                }
                Tag::List(start) => list_stack.push(start),
                Tag::Emphasis => style.emphasis = true,
                Tag::Strong => style.strong = true,
                Tag::Strikethrough => style.strike = true,
                Tag::Link { dest_url, .. } => style.link = Some(dest_url.to_string()),
                Tag::CodeBlock(kind) => {
                    let language = match kind {
                        CodeBlockKind::Fenced(info) => {
                            info.split_whitespace().next().map(str::to_string)
                        }
                        CodeBlockKind::Indented => None,
                    };
                    current_code = Some((language, Vec::new()));
                }
                Tag::Table(_) => table_rows = Some(Vec::new()),
                Tag::TableHead | Tag::TableRow => current_table_row = Some(Vec::new()),
                Tag::TableCell => current_table_cell = Some(Vec::new()),
                _ => {}
            },
            Event::End(end) => match end {
                TagEnd::Heading(_) => {
                    push_inline_block(&mut doc, current_kind.take(), &mut current_inlines)
                }
                TagEnd::Paragraph => {
                    if matches!(current_kind, Some(BlockKind::Paragraph)) {
                        push_inline_block(&mut doc, current_kind.take(), &mut current_inlines);
                    }
                }
                TagEnd::Item => {
                    if let Some(checked) = task_checked {
                        current_inlines
                            .insert(1, Inline::plain(if checked { "[x] " } else { "[ ] " }));
                    }
                    push_inline_block(&mut doc, current_kind.take(), &mut current_inlines);
                }
                TagEnd::BlockQuote(_) => {
                    push_inline_block(&mut doc, current_kind.take(), &mut current_inlines)
                }
                TagEnd::List(_) => {
                    list_stack.pop();
                }
                TagEnd::Emphasis => style.emphasis = false,
                TagEnd::Strong => style.strong = false,
                TagEnd::Strikethrough => style.strike = false,
                TagEnd::Link => style.link = None,
                TagEnd::CodeBlock => {
                    if let Some((language, lines)) = current_code.take() {
                        doc.blocks.push(Block::CodeBlock { language, lines });
                    }
                }
                TagEnd::Table => {
                    if let Some(rows) = table_rows.take() {
                        doc.blocks.push(Block::Table { rows });
                    }
                }
                TagEnd::TableHead | TagEnd::TableRow => {
                    if let (Some(rows), Some(row)) = (&mut table_rows, current_table_row.take()) {
                        rows.push(row);
                    }
                }
                TagEnd::TableCell => {
                    if let (Some(row), Some(cell)) =
                        (&mut current_table_row, current_table_cell.take())
                    {
                        row.push(cell);
                    }
                }
                _ => {}
            },
            Event::Text(text) => {
                if let Some((_, lines)) = current_code.as_mut() {
                    lines.extend(text.lines().map(str::to_string));
                } else if let Some(cell) = current_table_cell.as_mut() {
                    cell.push(Inline {
                        text: text.to_string(),
                        style: style.style(),
                    });
                } else {
                    current_inlines.push(Inline {
                        text: text.to_string(),
                        style: style.style(),
                    });
                }
            }
            Event::Code(text) => {
                let previous = style.code;
                style.code = true;
                current_inlines.push(Inline {
                    text: text.to_string(),
                    style: style.style(),
                });
                style.code = previous;
            }
            Event::SoftBreak | Event::HardBreak => current_inlines.push(Inline::plain(" ")),
            Event::Rule => doc.blocks.push(Block::ThematicBreak),
            Event::TaskListMarker(checked) => task_checked = Some(checked),
            Event::Html(text) | Event::InlineHtml(text) => {
                current_inlines.push(Inline::plain(text.to_string()))
            }
            _ => {}
        }
    }

    doc
}

fn push_inline_block(doc: &mut Document, kind: Option<BlockKind>, inlines: &mut Vec<Inline>) {
    let content = std::mem::take(inlines);
    match kind {
        Some(BlockKind::Heading(level)) => doc.blocks.push(Block::Heading { level, content }),
        Some(BlockKind::Paragraph) => doc.blocks.push(Block::Paragraph(content)),
        Some(BlockKind::ListItem) => doc.blocks.push(Block::ListItem {
            marker: "-".to_string(),
            checked: None,
            content,
        }),
        Some(BlockKind::BlockQuote) => doc.blocks.push(Block::BlockQuote(content)),
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_common_blocks() {
        let doc = parse_markdown("# Title\n\nA *soft* **loud** `code` [link](https://e.test)\n\n- [x] done\n\n> quote\n\n---\n\n```rust\nfn main() {}\n```");
        assert!(matches!(doc.blocks[0], Block::Heading { .. }));
        assert!(matches!(doc.blocks[1], Block::Paragraph(_)));
        assert!(matches!(doc.blocks[2], Block::ListItem { .. }));
        assert!(matches!(doc.blocks[3], Block::BlockQuote(_)));
        assert!(matches!(doc.blocks[4], Block::ThematicBreak));
        assert!(matches!(doc.blocks[5], Block::CodeBlock { .. }));
    }
}
