#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsingError {
    UnexpectedEof,
    UnexpectedToken { line: usize, message: String },
}
