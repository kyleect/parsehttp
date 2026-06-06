use crate::span::Span;

/// Used by the request and response parsers
///
/// Produced by the request or response lexers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<T> {
    pub kind: T,
    pub span: Span,
}

impl<T> Token<T> {
    /// Get the slice of text the span represents in the source text
    pub fn slice<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start.index..self.span.end.index]
    }
}
