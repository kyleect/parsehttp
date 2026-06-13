use crate::{lexing::tokens::Token, parsing::parse_errors::ParsingError};

pub trait Parser<T, R, S> {
    fn parse(self, tokens: Vec<Token<T>>) -> Result<(R, S), ParsingError>;
}
