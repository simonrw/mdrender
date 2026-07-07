use anstyle::{AnsiColor, Color, Style};

use crate::highlight::CodeStyle;
use crate::model::{Inline, InlineStyle};

pub fn paint_inline(run: &Inline, color: bool) -> String {
    if !color {
        return run.text.clone();
    }
    let style = style_for(&run.style);
    format!("{style}{}{style:#}", run.text)
}

fn style_for(inline: &InlineStyle) -> Style {
    if let Some(kind) = inline
        .link
        .as_deref()
        .and_then(|link| link.strip_prefix("code:"))
    {
        return match kind {
            "Keyword" => Style::new()
                .fg_color(Some(Color::Ansi(AnsiColor::Blue)))
                .bold(),
            "String" => Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))),
            "Number" => Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
            "Comment" => Style::new()
                .fg_color(Some(Color::Ansi(AnsiColor::BrightBlack)))
                .italic(),
            _ => Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
        };
    }

    if inline.code {
        Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan)))
    } else if inline.strong {
        Style::new().bold()
    } else if inline.emphasis {
        Style::new().italic()
    } else if inline.link.is_some() {
        Style::new()
            .fg_color(Some(Color::Ansi(AnsiColor::Blue)))
            .underline()
    } else if inline.strike {
        Style::new().strikethrough()
    } else {
        Style::new()
    }
}

#[allow(dead_code)]
fn _keep_code_style_used(_: CodeStyle) {}
