use bon::Builder;

/// A range and position of text within source text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Builder)]
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
pub fn span_position(index: usize, line: usize, column: usize) -> SpanPosition {
    (index, line, column).into()
}

/// A range and position of text within source text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Builder)]
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

        let span = Span::builder()
            .start(span_position(0, 1, 1))
            .end(span_position(4, 1, 5))
            .build();

        assert_eq!("Hell", span.slice(source));

        // let span = Span::builder().range(6..11).line(2).column(6).build();
        // assert_eq!("World", span.slice(source));
    }
}
