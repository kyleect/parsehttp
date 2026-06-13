use crate::{lexing::tokens::Token, parsing::parse_errors::ParsingError};

/// A trait for parsing HTTP (request and response) messages
///
/// Returns a tuple containing the parsed data and data spans.
///
/// **Generics**
///
/// - `TokenKind`: The type of token used for parsing
/// - `ParsedData`: The type of data parsed
/// - `ParsedSpans`: The type of spans used
pub trait HttpMessageParser<TokenKind, ParsedData, ParsedSpans> {
    fn parse(
        self,
        tokens: Vec<Token<TokenKind>>,
    ) -> Result<(ParsedData, ParsedSpans), ParsingError>;
}
