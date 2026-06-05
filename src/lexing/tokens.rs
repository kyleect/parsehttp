use crate::span::Span;

/// Used by the request and response parsers
///
/// Produced by the request or response lexers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<T> {
    pub kind: T,
    pub span: Span,
}
