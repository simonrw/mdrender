use terminal_size::{terminal_size, Width};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Viewport {
    pub width: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContentArea {
    pub left_padding: usize,
    pub width: usize,
}

impl Viewport {
    pub fn resolve(override_width: Option<usize>) -> Self {
        let width = override_width
            .or_else(|| terminal_size().map(|(Width(width), _)| width as usize))
            .unwrap_or(80)
            .max(1);
        Self { width }
    }

    pub fn content_area(self) -> ContentArea {
        if self.width >= 104 {
            let width = 88.min(self.width);
            ContentArea {
                left_padding: (self.width - width) / 2,
                width,
            }
        } else {
            ContentArea {
                left_padding: 0,
                width: self.width,
            }
        }
    }
}

pub fn display_width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

pub fn char_width(ch: char) -> usize {
    UnicodeWidthChar::width(ch).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_override_is_authoritative() {
        assert_eq!(Viewport::resolve(Some(120)).width, 120);
    }

    #[test]
    fn wide_viewport_uses_readable_measure() {
        assert_eq!(
            Viewport { width: 120 }.content_area(),
            ContentArea {
                left_padding: 16,
                width: 88
            }
        );
    }

    #[test]
    fn unicode_width_uses_terminal_cells() {
        assert_eq!(display_width("a界"), 3);
    }
}
