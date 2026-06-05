use crate::{
    parsing::{
        models::{HttpHeader, HttpResponse, HttpStatusCode, HttpVersion},
        parse_errors::ParsingError,
        parsing::Parser,
    },
    ResponseTokenKind, Token,
};

pub struct HttpResponseParser<'input> {
    source: &'input str,
    tokens: Vec<Token<ResponseTokenKind>>,
    current: usize,
}

impl<'input> HttpResponseParser<'input> {
    pub fn new(source: &'input str) -> Self {
        Self {
            source,
            tokens: vec![],
            current: 0,
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn check(&mut self, kind: ResponseTokenKind) -> bool {
        match self.peek() {
            Some(peek) => peek.kind == kind,
            None => false,
        }
    }

    /// Consume the current token if it matches expect kind
    fn consume(&mut self, kind: ResponseTokenKind) -> Result<(), ParsingError> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            let prev = self.prev().unwrap();
            let prev_lexme = prev.span.slice(self.source);
            Err(ParsingError::UnexpectedToken {
                line: self.peek().unwrap().span.start.line,
                message: format!("Expected '{}' but got '{}'", kind, prev_lexme),
            })
        }
    }

    fn peek(&self) -> Option<&Token<ResponseTokenKind>> {
        self.tokens.get(self.current)
    }

    fn prev(&self) -> Option<&Token<ResponseTokenKind>> {
        self.tokens.get(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn parse_status_code(&mut self) -> Result<HttpStatusCode, ParsingError> {
        let token = match self.peek() {
            Some(token) => *token,
            None => return Err(ParsingError::UnexpectedEof),
        };

        if token.kind != ResponseTokenKind::StatusCode {
            return Err(ParsingError::UnexpectedToken {
                line: token.span.start.line,
                message: format!(
                    "Expected a status code but got: '{}'",
                    token.span.slice(self.source)
                ),
            });
        }

        self.advance();

        Ok(HttpStatusCode::new(
            token.span.slice(self.source).parse::<u16>().unwrap(),
        ))
    }

    fn parse_version(&mut self) -> Result<HttpVersion, ParsingError> {
        let token = match self.peek() {
            Some(token) => *token,
            None => return Err(ParsingError::UnexpectedEof),
        };

        if token.kind != ResponseTokenKind::Version {
            return Err(ParsingError::UnexpectedToken {
                line: token.span.start.line,
                message: format!(
                    "Expected a version but got: '{}'",
                    token.span.slice(self.source)
                ),
            });
        }

        self.advance();

        Ok(token.span.slice(self.source).into())
    }

    fn parse_header_name(&mut self) -> Result<Token<ResponseTokenKind>, ParsingError> {
        let name_tok = match self.peek() {
            Some(token) => *token,
            None => return Err(ParsingError::UnexpectedEof),
        };

        if name_tok.kind != ResponseTokenKind::HeaderName {
            return Err(ParsingError::UnexpectedToken {
                line: name_tok.span.start.line,
                message: format!(
                    "Expected a header key but got: '{}'",
                    name_tok.span.slice(self.source)
                ),
            });
        }

        self.advance();

        Ok(name_tok)
    }

    fn parse_header_value(&mut self) -> Result<Token<ResponseTokenKind>, ParsingError> {
        let value_tok = match self.peek() {
            Some(token) => *token,
            None => return Err(ParsingError::UnexpectedEof),
        };
        if value_tok.kind != ResponseTokenKind::HeaderValue {
            return Err(ParsingError::UnexpectedToken {
                line: value_tok.span.start.line,
                message: format!(
                    "Expected a header value but got: '{}'",
                    value_tok.span.slice(self.source)
                ),
            });
        }
        self.advance();

        Ok(value_tok)
    }

    fn parse_headers(&mut self) -> Result<Vec<HttpHeader>, ParsingError> {
        let mut headers = Vec::new();

        while !self.check(ResponseTokenKind::CrLf) && !self.is_at_end() {
            let name_tok = self.parse_header_name()?;

            self.consume(ResponseTokenKind::Colon)?;

            let value_tok = self.parse_header_value()?;

            if !self.check(ResponseTokenKind::CrLf) {
                return Err(ParsingError::UnexpectedToken {
                    line: name_tok.span.start.line,
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

        Ok(headers)
    }

    fn parse_body(&mut self) -> Result<Option<String>, ParsingError> {
        let token = match self.peek() {
            Some(token) => *token,
            None => return Ok(None),
        };

        match token.kind {
            ResponseTokenKind::CrLf => {
                self.advance();
                Ok(Some(token.span.slice(self.source).to_string()))
            }
            ResponseTokenKind::Body => {
                self.advance();
                Ok(Some(token.span.slice(self.source).to_string()))
            }
            _ => Err(ParsingError::UnexpectedToken {
                line: token.span.start.line,
                message: format!(
                    "Expected a body or crlf but got: '{}' - {}",
                    token.span.slice(self.source),
                    token.kind
                ),
            }),
        }
    }
}

impl<'input> Parser<ResponseTokenKind, HttpResponse> for HttpResponseParser<'input> {
    fn parse(
        mut self,
        tokens: Vec<Token<ResponseTokenKind>>,
    ) -> Result<HttpResponse, ParsingError> {
        self.tokens = tokens;

        self.parse_version()?;

        self.consume(ResponseTokenKind::Space)?;

        let status_code = self.parse_status_code()?;

        self.consume(ResponseTokenKind::Space)?;

        self.consume(ResponseTokenKind::ReasonPhrase)?;

        self.consume(ResponseTokenKind::CrLf)?;

        let headers = self.parse_headers()?;

        self.consume(ResponseTokenKind::CrLf)?;

        self.consume(ResponseTokenKind::CrLf)?;

        let body = if self.check(ResponseTokenKind::Eof) {
            None
        } else {
            self.parse_body()?
        };

        let request = HttpResponse {
            status_code,
            headers,
            body,
        };

        Ok(request)
    }
}
