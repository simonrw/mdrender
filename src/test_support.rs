use crate::layout::Viewport;
use crate::parser::parse_markdown;
use crate::render::{layout_document, RenderedDocument};

pub struct RenderAssert {
    doc: RenderedDocument,
}

pub fn render(markdown: &str, width: usize) -> RenderAssert {
    let doc = parse_markdown(markdown);
    RenderAssert {
        doc: layout_document(&doc, Viewport { width }),
    }
}

impl RenderAssert {
    pub fn line_text(&self, index: usize, expected: &str) -> &Self {
        assert_eq!(self.doc.lines[index].text(), expected);
        self
    }

    pub fn indent(&self, index: usize, expected: usize) -> &Self {
        assert_eq!(self.doc.lines[index].indent(), expected);
        self
    }

    pub fn no_word_splits(&self) -> &Self {
        for line in &self.doc.lines {
            let text = line.text();
            assert!(
                !text.ends_with(char::is_alphabetic),
                "line may split a word: {text}"
            );
        }
        self
    }

    pub fn span_has_strong(&self, text: &str) -> &Self {
        assert!(self
            .doc
            .lines
            .iter()
            .flat_map(|line| line.runs.iter())
            .any(|run| run.text.contains(text) && run.style.strong));
        self
    }
}
