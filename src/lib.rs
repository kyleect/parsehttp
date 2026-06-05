use crate::{
    lexing::lex_errors::LexError,
    parsing::{
        parse_errors::ParsingError, parsing::Parser, request_parser::HttpRequestParser,
        response_parser::HttpResponseParser,
    },
};

mod lexing;
mod parsing;
mod span;

use crate::lexing::lexer::Lexer;
use crate::lexing::request_lexer::HttpRequestLexer;
pub use crate::lexing::request_lexer::RequestTokenKind;
use crate::lexing::response_lexer::HttpResponseLexer;
pub use crate::lexing::response_lexer::ResponseTokenKind;
pub use crate::lexing::tokens::Token;
pub use crate::parsing::models::HttpHeader;
pub use crate::parsing::models::HttpMethod;
pub use crate::parsing::models::HttpRequest;
pub use crate::parsing::models::HttpResponse;
pub use crate::parsing::models::HttpStatusCode;
pub use crate::parsing::models::HttpUri;
pub use crate::parsing::models::HttpVersion;
pub use crate::span::Span;

/// Lex a HTTP request from a string in to tokens
pub fn lex_request(src: &str) -> Result<Vec<Token<RequestTokenKind>>, LexError> {
    HttpRequestLexer::new(src).lex()
}

/// Lex a HTTP response from a string in to tokens
pub fn lex_response(src: &str) -> Result<Vec<Token<ResponseTokenKind>>, LexError> {
    HttpResponseLexer::new(src).lex()
}

/// Parse tokens in to an HTTP request
pub fn parse_request(
    src: &str,
    tokens: Vec<Token<RequestTokenKind>>,
) -> Result<HttpRequest, ParsingError> {
    HttpRequestParser::new(src).parse(tokens)
}

/// Parse tokens in to an HTTP response
pub fn parse_response(
    src: &str,
    tokens: Vec<Token<ResponseTokenKind>>,
) -> Result<HttpResponse, ParsingError> {
    HttpResponseParser::new(src).parse(tokens)
}
