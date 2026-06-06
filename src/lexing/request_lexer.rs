use std::fmt::Display;

use crate::{
    lexing::{lex_errors::LexError, lexer::Lexer, tokens::Token},
    span::{position, Span},
};

/// The different kinds of tokens for HTTP requests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestTokenKind {
    Method,
    Uri,
    Version,
    HeaderName,
    HeaderValue,
    Colon,
    Space,
    CrLf,
    Body,
    Eof,
}

impl Display for RequestTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            RequestTokenKind::Method => "Method",
            RequestTokenKind::Uri => "Uri",
            RequestTokenKind::Version => "HttpVersion",
            RequestTokenKind::HeaderName => "HeaderName",
            RequestTokenKind::HeaderValue => "HeaderValue",
            RequestTokenKind::Colon => ":",
            RequestTokenKind::Space => "Space",
            RequestTokenKind::CrLf => "CrLf",
            RequestTokenKind::Body => "Body",
            RequestTokenKind::Eof => "Eof",
        };

        write!(f, "{string}")
    }
}

pub struct HttpRequestLexer<'a> {
    bytes: &'a [u8],
    start: usize,
    current: usize,
    start_line: usize,
    start_column: usize,
    current_line: usize,
    current_column: usize,
}

impl<'a> HttpRequestLexer<'a> {
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

    fn lex_request_line(
        &mut self,
        tokens: &mut Vec<Token<RequestTokenKind>>,
    ) -> Result<(), LexError> {
        self.lex_until_space(tokens, RequestTokenKind::Method)?;
        self.consume_space(tokens)?;

        self.lex_until_space(tokens, RequestTokenKind::Uri)?;
        self.consume_space(tokens)?;

        self.lex_until_crlf(tokens, RequestTokenKind::Version)?;
        self.consume_crlf(tokens)?;

        Ok(())
    }

    fn lex_headers(&mut self, tokens: &mut Vec<Token<RequestTokenKind>>) -> Result<(), LexError> {
        loop {
            if self.check_crlf() {
                self.consume_crlf(tokens)?;
                break;
            }

            self.consume_until_byte(tokens, b':', RequestTokenKind::HeaderName)?;

            self.start = self.current;
            self.start_column = self.current_column;
            self.start_line = self.current_line;

            self.consume_byte(b':')?;
            tokens.push(self.token(RequestTokenKind::Colon));

            self.consume_whitespace();

            self.lex_until_crlf(tokens, RequestTokenKind::HeaderValue)?;

            self.consume_crlf(tokens)?;
        }

        Ok(())
    }

    fn lex_body(&mut self, tokens: &mut Vec<Token<RequestTokenKind>>) -> Result<(), LexError> {
        if self.is_at_end() {
            return Ok(());
        }

        self.start = self.current;
        self.start_column = self.current_column;
        self.start_line = self.current_line;
        self.current = self.bytes.len();

        tokens.push(self.token(RequestTokenKind::Body));

        Ok(())
    }

    fn lex_until_space(
        &mut self,
        tokens: &mut Vec<Token<RequestTokenKind>>,
        kind: RequestTokenKind,
    ) -> Result<(), LexError> {
        self.start = self.current;
        self.start_column = self.current_column;
        self.start_line = self.current_line;

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
        tokens: &mut Vec<Token<RequestTokenKind>>,
        kind: RequestTokenKind,
    ) -> Result<(), LexError> {
        self.start = self.current;
        self.start_column = self.current_column;
        self.start_line = self.current_line;

        while !self.check_crlf() {
            if self.is_at_end() {
                return Err(LexError::UnexpectedEof);
            }

            self.advance();
        }

        tokens.push(self.token(kind));

        Ok(())
    }

    fn consume_until_byte(
        &mut self,
        tokens: &mut Vec<Token<RequestTokenKind>>,
        stop: u8,
        kind: RequestTokenKind,
    ) -> Result<(), LexError> {
        self.start = self.current;
        self.start_column = self.current_column;
        self.start_line = self.current_line;

        while let Some(b) = self.peek() {
            if b == stop {
                break;
            }

            self.advance();
        }

        tokens.push(self.token(kind));

        Ok(())
    }

    fn consume_space(&mut self, tokens: &mut Vec<Token<RequestTokenKind>>) -> Result<(), LexError> {
        self.start = self.current;
        self.start_column = self.current_column;
        self.start_line = self.current_line;

        self.consume_byte(b' ')?;

        tokens.push(self.token(RequestTokenKind::Space));

        Ok(())
    }

    fn consume_crlf(&mut self, tokens: &mut Vec<Token<RequestTokenKind>>) -> Result<(), LexError> {
        self.start = self.current;
        self.start_column = self.current_column;
        self.start_line = self.current_line;

        self.consume_byte(b'\r')?;
        self.consume_byte(b'\n')?;

        tokens.push(self.token(RequestTokenKind::CrLf));

        self.current_line += 1;
        self.current_column = 1;

        Ok(())
    }

    fn consume_whitespace(&mut self) {
        while self.peek() == Some(b' ') {
            self.advance();
        }

        self.start = self.current;
        self.start_column = self.current_column;
        self.start_line = self.current_line;
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

    fn token(&self, kind: RequestTokenKind) -> Token<RequestTokenKind> {
        Token {
            kind,
            span: Span {
                start: position(self.start, self.start_line, self.start_column),
                end: position(self.current, self.current_line, self.current_column),
            },
        }
    }
}

type RequestToken = Token<RequestTokenKind>;

impl<'input> Lexer<RequestTokenKind> for HttpRequestLexer<'input> {
    fn lex(mut self) -> Result<Vec<RequestToken>, LexError> {
        let mut tokens = Vec::new();

        self.lex_request_line(&mut tokens)?;
        self.lex_headers(&mut tokens)?;
        self.lex_body(&mut tokens)?;

        tokens.push(self.token(RequestTokenKind::Eof));

        Ok(tokens)
    }
}
