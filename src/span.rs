use bon::Builder;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use crate::Token;

/// A pair of `T` data and [`Span`]
pub type Spanned<T> = (T, Span);

pub fn get_spanned_value<T>((value, _): Spanned<T>) -> T {
    value
}

pub fn get_spanned_span<T>((_, span): Spanned<T>) -> Span {
    span
}

pub fn get_spanned_lexme<T>(token: Token<T>, source: &str) -> Spanned<String> {
    (token.span.slice(source).to_string(), token.span)
}

/// A range and position of text within source text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Builder)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct SpanPosition {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl From<(usize, usize, usize)> for SpanPosition {
    fn from(value: (usize, usize, usize)) -> Self {
        Self::builder()
            .index(value.0)
            .line(value.1)
            .column(value.2)
            .build()
    }
}

/// Construct a span position
pub fn position(index: usize, line: usize, column: usize) -> SpanPosition {
    (index, line, column).into()
}

/// A range and position of text within source text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Builder)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Span {
    pub start: SpanPosition,
    pub end: SpanPosition,
}

impl Span {
    /// Get the slice of text the span represents in the source text
    pub fn slice<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start.index..self.end.index]
    }
}

/// Construct a span from two span positions
pub fn span(start: impl Into<SpanPosition>, end: impl Into<SpanPosition>) -> Span {
    Span {
        start: start.into(),
        end: end.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice() {
        let source = "Hello\nWorld";

        assert_eq!(
            "Hell",
            span(position(0, 1, 1), position(4, 1, 5)).slice(source)
        );

        assert_eq!(
            "World",
            span(position(6, 2, 1), position(11, 2, 6)).slice(source)
        );
    }
}
