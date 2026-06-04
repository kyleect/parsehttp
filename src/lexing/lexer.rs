use crate::lexing::{lex_errors::LexError, tokens::Token};

pub trait Lexer<T> {
    fn lex(self) -> Result<Vec<Token<T>>, LexError>;
}
