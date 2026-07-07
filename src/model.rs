#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block {
    Heading {
        level: u32,
        content: Vec<Inline>,
    },
    Paragraph(Vec<Inline>),
    ListItem {
        marker: String,
        checked: Option<bool>,
        content: Vec<Inline>,
    },
    BlockQuote(Vec<Inline>),
    CodeBlock {
        language: Option<String>,
        lines: Vec<String>,
    },
    Table {
        rows: Vec<Vec<Vec<Inline>>>,
    },
    ThematicBreak,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inline {
    pub text: String,
    pub style: InlineStyle,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InlineStyle {
    pub emphasis: bool,
    pub strong: bool,
    pub code: bool,
    pub link: Option<String>,
    pub strike: bool,
}

impl Inline {
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: InlineStyle::default(),
        }
    }
}
