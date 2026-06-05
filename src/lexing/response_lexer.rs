use super::*;
use crate::{
    lexing::{lex_errors::LexError, lexer::Lexer, tokens::Token},
    span::{span_position, Span},
};
use std::fmt::Display;

/// The different kinds of tokens for HTTP responses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseTokenKind {
    Version,
    StatusCode,
    ReasonPhrase,
    HeaderName,
    HeaderValue,
    Colon,
    Space,
    CrLf,
    Body,
    Eof,
}

impl Display for ResponseTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ResponseTokenKind::Version => "HttpVersion",
            ResponseTokenKind::StatusCode => "StatusCode",
            ResponseTokenKind::ReasonPhrase => "ReasonPhrase",
            ResponseTokenKind::HeaderName => "HeaderName",
            ResponseTokenKind::HeaderValue => "HeaderValue",
            ResponseTokenKind::Colon => ":",
            ResponseTokenKind::Space => "Space",
            ResponseTokenKind::CrLf => "CrLf",
            ResponseTokenKind::Body => "Body",
            ResponseTokenKind::Eof => "Eof",
        };
        write!(f, "{string}")
    }
}

pub struct HttpResponseLexer<'a> {
    bytes: &'a [u8],
    start: usize,
    current: usize,
    start_line: usize,
    start_column: usize,
    current_line: usize,
    current_column: usize,
}

impl<'a> HttpResponseLexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            bytes: src.as_bytes(),
            start: 0,
            current: 0,
            start_line: 1,
            start_column: 1,
            current_line: 1,
            current_column: 1,
        }
    }

    fn lex_status_line(
        &mut self,
        tokens: &mut Vec<tokens::Token<ResponseTokenKind>>,
    ) -> Result<(), LexError> {
        self.lex_until_space(tokens, ResponseTokenKind::Version)?;
        self.consume_space(tokens)?;
        self.lex_until_space(tokens, ResponseTokenKind::StatusCode)?;
        self.consume_space(tokens)?;
        self.lex_until_crlf(tokens, ResponseTokenKind::ReasonPhrase)?;
        self.consume_crlf(tokens)?;
        Ok(())
    }

    fn lex_headers(
        &mut self,
        tokens: &mut Vec<tokens::Token<ResponseTokenKind>>,
    ) -> Result<(), LexError> {
        loop {
            if self.check_crlf() {
                self.consume_crlf(tokens)?;
                break;
            }
            self.lex_until_byte(tokens, b':', ResponseTokenKind::HeaderName)?;
            self.start = self.current;
            self.start_line = self.current_line;
            self.start_column = self.current_column;
            self.consume_byte(b':')?;
            tokens.push(self.token(ResponseTokenKind::Colon));
            self.optional_spaces();
            self.lex_until_crlf(tokens, ResponseTokenKind::HeaderValue)?;
            self.consume_crlf(tokens)?;
        }
        Ok(())
    }

    fn lex_body(
        &mut self,
        tokens: &mut Vec<tokens::Token<ResponseTokenKind>>,
    ) -> Result<(), LexError> {
        if self.is_at_end() {
            return Ok(());
        }
        self.start = self.current;
        self.start_line = self.current_line;
        self.start_column = self.current_column;
        self.current = self.bytes.len();
        tokens.push(self.token(ResponseTokenKind::Body));
        Ok(())
    }

    fn lex_until_space(
        &mut self,
        tokens: &mut Vec<tokens::Token<ResponseTokenKind>>,
        kind: ResponseTokenKind,
    ) -> Result<(), LexError> {
        self.start = self.current;
        self.start_line = self.current_line;
        self.start_column = self.current_column;
        while let Some(b) = self.peek() {
            if b == b' ' {
                break;
            }
            self.advance();
        }
        tokens.push(self.token(kind));
        Ok(())
    }

    fn lex_until_crlf(
        &mut self,
        tokens: &mut Vec<tokens::Token<ResponseTokenKind>>,
        kind: ResponseTokenKind,
    ) -> Result<(), LexError> {
        self.start = self.current;
        self.start_line = self.current_line;
        self.start_column = self.current_column;
        while !self.check_crlf() {
            if self.is_at_end() {
                return Err(LexError::UnexpectedEof);
            }
            self.advance();
        }
        tokens.push(self.token(kind));
        Ok(())
    }

    fn lex_until_byte(
        &mut self,
        tokens: &mut Vec<tokens::Token<ResponseTokenKind>>,
        stop: u8,
        kind: ResponseTokenKind,
    ) -> Result<(), LexError> {
        self.start = self.current;
        self.start_line = self.current_line;
        self.start_column = self.current_column;
        while let Some(b) = self.peek() {
            if b == stop {
                break;
            }
            self.advance();
        }
        tokens.push(self.token(kind));
        Ok(())
    }

    fn consume_space(
        &mut self,
        tokens: &mut Vec<tokens::Token<ResponseTokenKind>>,
    ) -> Result<(), LexError> {
        self.start = self.current;
        self.start_line = self.current_line;
        self.start_column = self.current_column;
        self.consume_byte(b' ')?;
        tokens.push(self.token(ResponseTokenKind::Space));
        Ok(())
    }

    fn consume_crlf(
        &mut self,
        tokens: &mut Vec<tokens::Token<ResponseTokenKind>>,
    ) -> Result<(), LexError> {
        self.start = self.current;
        self.start_line = self.current_line;
        self.start_column = self.current_column;
        self.consume_byte(b'\r')?;
        self.consume_byte(b'\n')?;
        tokens.push(self.token(ResponseTokenKind::CrLf));
        self.current_line += 1;
        self.current_column = 1;
        Ok(())
    }

    fn optional_spaces(&mut self) {
        while self.peek() == Some(b' ') {
            self.advance();
        }
        // After skipping optional spaces we start a new token,
        // so update start positions to the next character.
        self.start = self.current;
        self.start_line = self.current_line;
        self.start_column = self.current_column;
    }

    fn consume_byte(&mut self, expected: u8) -> Result<(), LexError> {
        match self.peek() {
            Some(b) if b == expected => {
                self.advance();
                Ok(())
            }
            _ => Err(LexError::InvalidToken {
                line: self.current_line,
            }),
        }
    }

    fn check_crlf(&self) -> bool {
        matches!((self.peek(), self.peek_next()), (Some(b'\r'), Some(b'\n')))
    }

    fn advance(&mut self) {
        self.current += 1;
        self.current_column += 1;
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<u8> {
        self.bytes.get(self.current + 1).copied()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.bytes.len()
    }

    fn token(&self, kind: ResponseTokenKind) -> tokens::Token<ResponseTokenKind> {
        tokens::Token {
            kind,
            span: Span {
                start: span_position(self.start, self.start_line, self.start_column),
                end: span_position(self.current, self.current_line, self.current_column),
            },
        }
    }
}

impl<'input> Lexer<ResponseTokenKind> for HttpResponseLexer<'input> {
    fn lex(mut self) -> Result<Vec<Token<ResponseTokenKind>>, LexError> {
        let mut tokens = Vec::new();
        self.lex_status_line(&mut tokens)?;
        self.lex_headers(&mut tokens)?;
        self.lex_body(&mut tokens)?;
        tokens.push(self.token(ResponseTokenKind::Eof));
        Ok(tokens)
    }
}
