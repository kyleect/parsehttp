#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    UnexpectedEof,
    InvalidToken { line: u32 },
}
