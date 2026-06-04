#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsingError {
    UnexpectedEof,
    UnexpectedToken { line: u32, message: String },
}
