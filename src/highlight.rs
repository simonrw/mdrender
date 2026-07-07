use crate::model::{Inline, InlineStyle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodeStyle {
    Keyword,
    String,
    Number,
    Comment,
    Plain,
}

pub fn highlight_code(language: Option<&str>, line: &str) -> Vec<(String, CodeStyle)> {
    match language.unwrap_or("").to_ascii_lowercase().as_str() {
        "rust" | "rs" | "toml" | "json" | "yaml" | "yml" | "sh" | "bash" | "js" | "javascript"
        | "ts" | "typescript" | "md" | "markdown" => lightweight_highlight(line),
        _ => vec![(line.to_string(), CodeStyle::Plain)],
    }
}

pub fn code_inline(text: impl Into<String>, style: CodeStyle) -> Inline {
    let inline_style = InlineStyle {
        code: true,
        link: Some(format!("code:{style:?}")),
        ..InlineStyle::default()
    };
    Inline {
        text: text.into(),
        style: inline_style,
    }
}

fn lightweight_highlight(line: &str) -> Vec<(String, CodeStyle)> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with('#') {
        return vec![(line.to_string(), CodeStyle::Comment)];
    }

    line.split_inclusive(|ch: char| ch.is_whitespace())
        .map(|part| {
            let bare = part.trim_matches(|ch: char| !ch.is_alphanumeric() && ch != '_');
            let style = if matches!(
                bare,
                "fn" | "let"
                    | "mut"
                    | "pub"
                    | "use"
                    | "struct"
                    | "enum"
                    | "impl"
                    | "const"
                    | "return"
                    | "function"
                    | "class"
                    | "import"
                    | "export"
                    | "true"
                    | "false"
            ) {
                CodeStyle::Keyword
            } else if bare.parse::<f64>().is_ok() {
                CodeStyle::Number
            } else if part.contains('"') || part.contains('\'') {
                CodeStyle::String
            } else {
                CodeStyle::Plain
            };
            (part.to_string(), style)
        })
        .collect()
}
