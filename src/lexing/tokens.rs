use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<T> {
    pub kind: T,
    pub span: Span,
}
