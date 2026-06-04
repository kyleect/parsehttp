use crate::{lexing::tokens::Token, parsing::parse_errors::ParsingError};

pub trait Parser<T, R> {
    fn parse(self, tokens: Vec<Token<T>>) -> Result<R, ParsingError>;
}
