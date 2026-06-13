use crate::parsing::{
    message_parser::HttpMessageParser, request_parser::HttpRequestParser,
    response_parser::HttpResponseParser,
};

mod lexing;
mod parsing;
mod span;

pub use crate::lexing::lex_errors::LexError;
use crate::lexing::lexer::Lexer;
use crate::lexing::request_lexer::HttpRequestLexer;
pub use crate::lexing::request_lexer::RequestTokenKind;
use crate::lexing::response_lexer::HttpResponseLexer;
pub use crate::lexing::response_lexer::ResponseTokenKind;
pub use crate::lexing::tokens::Token;
pub use crate::parsing::models::{
    HttpHeader, HttpMethod, HttpRequest, HttpRequestSpans, HttpResponse, HttpResponseSpans,
    HttpStatusCode, HttpStatusText, HttpUri, HttpVersion,
};
pub use crate::parsing::parse_errors::ParsingError;
pub use crate::span::{position, span, Span, Spanned};

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
) -> Result<(HttpRequest, HttpRequestSpans), ParsingError> {
    HttpRequestParser::new(src).parse(tokens)
}

/// Parse tokens in to an HTTP response
pub fn parse_response(
    src: &str,
    tokens: Vec<Token<ResponseTokenKind>>,
) -> Result<(HttpResponse, HttpResponseSpans), ParsingError> {
    HttpResponseParser::new(src).parse(tokens)
}
