use crate::layout::{char_width, display_width};
use crate::model::{Inline, InlineStyle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StyledLine {
    pub runs: Vec<Inline>,
}

impl StyledLine {
    pub fn text(&self) -> String {
        self.runs.iter().map(|run| run.text.as_str()).collect()
    }
}

pub fn wrap_inlines(inlines: &[Inline], width: usize) -> Vec<StyledLine> {
    let tokens = tokenize(inlines, width.max(1));
    if tokens.is_empty() {
        return vec![StyledLine { runs: Vec::new() }];
    }
    let breaks = best_breaks(&tokens, width.max(1));
    lines_from_breaks(&tokens, &breaks)
}

#[derive(Clone, Debug)]
struct Token {
    text: String,
    style: InlineStyle,
    width: usize,
    penalty: usize,
}

fn tokenize(inlines: &[Inline], width: usize) -> Vec<Token> {
    let mut tokens = Vec::new();
    for inline in inlines {
        for raw in split_breaks(&inline.text) {
            for piece in split_long_token(raw, &inline.style, width) {
                tokens.push(piece);
            }
        }
    }
    tokens
}

fn split_breaks(text: &str) -> Vec<&str> {
    let mut pieces = Vec::new();
    let mut start = 0;
    for (index, ch) in text.char_indices() {
        if ch.is_whitespace() {
            if start < index {
                pieces.push(&text[start..index]);
            }
            start = index + ch.len_utf8();
        } else if matches!(ch, '/' | '?' | '&' | '-' | '_') {
            let end = index + ch.len_utf8();
            pieces.push(&text[start..end]);
            start = end;
        }
    }
    if start < text.len() {
        pieces.push(&text[start..]);
    }
    pieces
}

fn split_long_token(text: &str, style: &InlineStyle, width: usize) -> Vec<Token> {
    if display_width(text) <= width {
        return vec![Token::new(text.to_string(), style.clone(), 0)];
    }

    let mut out = Vec::new();
    let mut current = String::new();
    let mut current_width = 0;
    for ch in text.chars() {
        let ch_width = char_width(ch);
        if current_width > 0 && current_width + ch_width > width {
            let needs_hyphen = !style.code && current_width < width;
            if needs_hyphen {
                current.push('-');
            }
            out.push(Token::new(
                current,
                style.clone(),
                if needs_hyphen { 80 } else { 160 },
            ));
            current = String::new();
            current_width = 0;
        }
        current.push(ch);
        current_width += ch_width;
    }
    if !current.is_empty() {
        out.push(Token::new(current, style.clone(), 0));
    }
    out
}

impl Token {
    fn new(text: String, style: InlineStyle, penalty: usize) -> Self {
        let width = display_width(&text);
        Self {
            text,
            style,
            width,
            penalty,
        }
    }
}

fn best_breaks(tokens: &[Token], width: usize) -> Vec<usize> {
    let n = tokens.len();
    let mut cost = vec![usize::MAX / 4; n + 1];
    let mut next = vec![n; n + 1];
    cost[n] = 0;

    for i in (0..n).rev() {
        let mut line_width = 0;
        for j in i..n {
            let sep = usize::from(j > i);
            line_width += sep + tokens[j].width;
            if line_width > width {
                break;
            }
            let leftover = width - line_width;
            let final_line = j + 1 == n;
            let weak_word = matches!(
                tokens[j].text.as_str(),
                "a" | "an" | "the" | "of" | "to" | "and" | "or" | "in"
            );
            let badness = if final_line {
                leftover
            } else {
                leftover * leftover * leftover
            };
            let candidate = badness
                + tokens[j].penalty
                + if tokens[j].style.code && !final_line {
                    40
                } else {
                    0
                }
                + if weak_word && !final_line { 25 } else { 0 }
                + cost[j + 1];
            if candidate < cost[i] {
                cost[i] = candidate;
                next[i] = j + 1;
            }
        }
    }

    let mut breaks = Vec::new();
    let mut i = 0;
    while i < n {
        let j = next[i].max(i + 1);
        breaks.push(j);
        i = j;
    }
    breaks
}

fn lines_from_breaks(tokens: &[Token], breaks: &[usize]) -> Vec<StyledLine> {
    let mut lines = Vec::new();
    let mut start = 0;
    for &end in breaks {
        let mut runs = Vec::new();
        for (index, token) in tokens[start..end].iter().enumerate() {
            if index > 0 {
                runs.push(Inline::plain(" "));
            }
            runs.push(Inline {
                text: token.text.clone(),
                style: token.style.clone(),
            });
        }
        lines.push(StyledLine { runs });
        start = end;
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_at_words() {
        let lines = wrap_inlines(
            &[Inline::plain(
                "This paragraph contains natural word breaks.",
            )],
            18,
        );
        assert_eq!(
            lines.iter().map(StyledLine::text).collect::<Vec<_>>(),
            vec!["This paragraph", "contains natural", "word breaks."]
        );
    }

    #[test]
    fn splits_long_tokens() {
        let lines = wrap_inlines(&[Inline::plain("supercalifragilistic")], 8);
        assert!(lines.iter().all(|line| display_width(&line.text()) <= 8));
    }
}
