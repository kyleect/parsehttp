use crate::{
    lexing::lex_errors::LexError,
    parsing::{
        models::HttpResponse, parse_errors::ParsingError, parsing::Parser,
        request_parser::HttpRequestParser, response_parser::HttpResponseParser,
    },
};

mod lexing;
mod parsing;
mod span;

pub use crate::lexing::lexer::Lexer;
pub use crate::lexing::request_lexer::HttpRequestLexer;
pub use crate::lexing::request_lexer::RequestTokenKind;
pub use crate::lexing::response_lexer::HttpResponseLexer;
pub use crate::lexing::response_lexer::ResponseTokenKind;
pub use crate::lexing::tokens::Token;
pub use crate::parsing::models::HttpRequest;
pub use crate::span::Span;

pub fn lex_request(src: &str) -> Result<Vec<Token<RequestTokenKind>>, LexError> {
    HttpRequestLexer::new(src).lex()
}

pub fn lex_response(src: &str) -> Result<Vec<Token<ResponseTokenKind>>, LexError> {
    HttpResponseLexer::new(src).lex()
}

pub fn parse_request(
    src: &str,
    tokens: Vec<Token<RequestTokenKind>>,
) -> Result<HttpRequest, ParsingError> {
    HttpRequestParser::new(src).parse(tokens)
}

pub fn parse_response(
    src: &str,
    tokens: Vec<Token<ResponseTokenKind>>,
) -> Result<HttpResponse, ParsingError> {
    HttpResponseParser::new(src).parse(tokens)
}
