use std::ops::Range;

use bon::bon;

/// A range and position of text within source text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
    pub line: u32,
    pub column: u32,
}

#[bon]
impl Span {
    #[builder]
    pub fn new(range: Range<u32>, line: u32, column: u32) -> Self {
        Self {
            start: range.start,
            end: range.end,
            line,
            column,
        }
    }

    /// Get the slice of text the span represents in the source text
    pub fn slice<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start as usize..self.end as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice() {
        let source = "Hello\nWorld";

        let span = Span::builder().range(0..4).line(1).column(5).build();
        assert_eq!("Hell", span.slice(source));

        let span = Span::builder().range(6..11).line(2).column(6).build();
        assert_eq!("World", span.slice(source));
    }
}
