/// An error that occurs during lexing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    UnexpectedEof,
    InvalidToken { line: usize },
}
