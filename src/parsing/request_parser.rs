use crate::{
    lexing::{request_lexer::RequestTokenKind, tokens::Token},
    parsing::{
        models::{HttpHeader, HttpMethod, HttpRequest, HttpVersion, Uri},
        parsing::Parser,
    },
};

use super::parse_errors::ParsingError;

pub struct HttpRequestParser<'input> {
    source: &'input str,
    tokens: Vec<Token<RequestTokenKind>>,
    current: usize,
}

impl<'input> HttpRequestParser<'input> {
    pub fn new(source: &'input str) -> Self {
        Self {
            source: source,
            tokens: vec![],
            current: 0,
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn check(&mut self, kind: RequestTokenKind) -> bool {
        match self.peek() {
            Some(peek) => peek.kind == kind,
            None => false,
        }
    }

    /// Consume the current token if it matches expect kind
    fn consume(&mut self, kind: RequestTokenKind) -> Result<(), ParsingError> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            let prev = self.prev().unwrap();
            let prev_lexme = prev.span.slice(self.source);
            Err(ParsingError::UnexpectedToken {
                line: self.peek().unwrap().span.line,
                message: format!("Expected '{}' but got '{}'", kind, prev_lexme),
            })
        }
    }

    fn peek(&self) -> Option<&Token<RequestTokenKind>> {
        return self.tokens.get(self.current);
    }

    fn prev(&self) -> Option<&Token<RequestTokenKind>> {
        return self.tokens.get(self.current - 1);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn parse_method(&mut self) -> Result<HttpMethod, ParsingError> {
        let token = match self.peek() {
            Some(t) => t.clone(),
            None => return Err(ParsingError::UnexpectedEof),
        };

        if token.kind != RequestTokenKind::Method {
            return Err(ParsingError::UnexpectedToken {
                line: token.span.line,
                message: format!(
                    "Expected a method but got: '{}'",
                    token.span.slice(self.source)
                ),
            });
        }

        self.advance();

        Ok(token.span.slice(self.source).into())
    }

    fn parse_uri(&mut self) -> Result<Uri, ParsingError> {
        let token = match self.peek() {
            Some(t) => t.clone(),
            None => return Err(ParsingError::UnexpectedEof),
        };
        let lexme = token.span.slice(self.source);

        if token.kind != RequestTokenKind::Uri {
            return Err(ParsingError::UnexpectedToken {
                line: token.span.line,
                message: format!("Expected a uri but got: '{}'", lexme),
            });
        }

        self.advance();

        Ok(lexme.into())
    }

    fn parse_version(&mut self) -> Result<HttpVersion, ParsingError> {
        let token = match self.peek() {
            Some(t) => t.clone(),
            None => return Err(ParsingError::UnexpectedEof),
        };

        if token.kind != RequestTokenKind::Version {
            return Err(ParsingError::UnexpectedToken {
                line: token.span.line,
                message: format!(
                    "Expected a version but got: '{}'",
                    token.span.slice(self.source)
                ),
            });
        }

        self.advance();

        Ok(token.span.slice(self.source).into())
    }

    fn parse_headers(&mut self) -> Result<Vec<HttpHeader>, ParsingError> {
        let mut headers = Vec::new();

        while !self.check(RequestTokenKind::CrLf) && !self.is_at_end() {
            let name_tok = match self.peek() {
                Some(t) => t.clone(),
                None => return Err(ParsingError::UnexpectedEof),
            };

            if name_tok.kind != RequestTokenKind::HeaderName {
                return Err(ParsingError::UnexpectedToken {
                    line: name_tok.span.line,
                    message: format!(
                        "Expected a header key but got: '{}'",
                        name_tok.span.slice(self.source)
                    ),
                });
            }

            self.advance();

            if !self.check(RequestTokenKind::Colon) {
                return Err(ParsingError::UnexpectedToken {
                    line: self.peek().unwrap().span.line,
                    message: format!(
                        "Expected a header separator but got: '{}'",
                        self.peek().unwrap().span.slice(self.source)
                    ),
                });
            }

            self.advance();

            let value_tok = match self.peek() {
                Some(t) => t.clone(),
                None => return Err(ParsingError::UnexpectedEof),
            };
            if value_tok.kind != RequestTokenKind::HeaderValue {
                return Err(ParsingError::UnexpectedToken {
                    line: value_tok.span.line,
                    message: format!(
                        "Expected a header value but got: '{}'",
                        value_tok.span.slice(self.source)
                    ),
                });
            }
            self.advance();

            if !self.check(RequestTokenKind::CrLf) {
                return Err(ParsingError::UnexpectedToken {
                    line: name_tok.span.line,
                    message: format!(
                        "Expected a crlf but got: '{}'",
                        name_tok.span.slice(self.source)
                    ),
                });
            }

            headers.push(HttpHeader::new(
                name_tok.span.slice(self.source),
                value_tok.span.slice(self.source),
            ));
        }

        self.consume(RequestTokenKind::CrLf)?;

        Ok(headers)
    }

    fn parse_body(&mut self) -> Result<Option<String>, ParsingError> {
        let token = match self.peek() {
            Some(t) => t.clone(),
            None => return Ok(None),
        };

        if token.kind != RequestTokenKind::Body {
            return Err(ParsingError::UnexpectedToken {
                line: token.span.line,
                message: format!(
                    "Expected a version but got: '{}'",
                    token.span.slice(self.source)
                ),
            });
        }

        self.advance();

        Ok(Some(token.span.slice(self.source).to_string()))
    }
}

impl<'input> Parser<RequestTokenKind, HttpRequest> for HttpRequestParser<'input> {
    fn parse(mut self, tokens: Vec<Token<RequestTokenKind>>) -> Result<HttpRequest, ParsingError> {
        self.tokens = tokens;

        let method = self.parse_method()?;

        self.consume(RequestTokenKind::Space)?;

        let uri = self.parse_uri()?;

        self.consume(RequestTokenKind::Space)?;

        let http_version = self.parse_version()?;

        self.consume(RequestTokenKind::CrLf)?;

        let headers = self.parse_headers()?;

        self.consume(RequestTokenKind::CrLf)?;

        let body = self.parse_body()?;

        let request = HttpRequest {
            uri,
            method,
            http_version,
            headers: headers,
            body: body,
        };

        Ok(request)
    }
}
