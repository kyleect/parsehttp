# Lexing

Convert an HTTP request or response source string in to a list of tokens using `parsehttp::lex_request(source_text) -> Vec<Token<RequestTokenKind>`.

```rust
use parsehttp::{lex_request, span, position, RequestTokenKind, Token};

let request_src = "\
    POST / HTTP/1.1\r\n\
    Host: example.com\r\n\
    \r\n\
    body";

let tokens: Vec<Token<RequestTokenKind>> = lex_request(request_src).unwrap();
```

## Tokens

A `parsehttp::Token<TokenKind>` represents the atomic parts of an HTTP request or response.

```rust
pub struct Token<TokenKind> {
    pub kind: TokenKind,
    pub span: Span,
}

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
```

## Token Spans

Tokens record both the starting and ending `Span` and `SpanPosition`. These values are recorded during the lexing pass.

```rust
pub struct Span {
    pub start: SpanPosition,
    pub end: SpanPosition,
}

pub struct SpanPosition {
    pub index: usize,  // (0 index)
    pub line: usize,   // (1 index)
    pub column: usize, // (1 index)
}
```

There are two helper functions for creating spans:

- `parsehttp::span(start: SpanPosition, end: SpanPosition) -> Span`
- `parsehttp::position(index: usize, line: usize, column: usize) -> SpanPosition`

### Example

```rust
use parsehttp::{lex_request, span, position, RequestTokenKind, Token};

let request_src = "\
    POST / HTTP/1.1\r\n\
    Host: example.com\r\n\
    \r\n\
    body";

let tokens: Vec<Token<RequestTokenKind>> = lex_request(request_src).unwrap();

assert_eq!(
    vec![
        Token {
            kind: RequestTokenKind::Method,
            span: span(position(0, 1, 1), position(4, 1, 5)),
        },
        Token {
            kind: RequestTokenKind::Space,
            span: span(position(4, 1, 5), position(5, 1, 6)),
        },
        Token {
            kind: RequestTokenKind::Uri,
            span: span(position(5, 1, 6), position(6, 1, 7)),
        },
        Token {
            kind: RequestTokenKind::Space,
            span: span(position(6, 1, 7), position(7, 1, 8)),
        },
        Token {
            kind: RequestTokenKind::Version,
            span: span(position(7, 1, 8), position(15, 1, 16)),
        },
        Token {
            kind: RequestTokenKind::CrLf,
            span: span(position(15, 1, 16), position(17, 1, 18)),
        },
        Token {
            kind: RequestTokenKind::HeaderName,
            span: span(position(17, 2, 1), position(21, 2, 5)),
        },
        Token {
            kind: RequestTokenKind::Colon,
            span: span(position(21, 2, 5), position(22, 2, 6)),
        },
        Token {
            kind: RequestTokenKind::HeaderValue,
            span: span(position(23, 2, 7), position(34, 2, 18)),
        },
        Token {
            kind: RequestTokenKind::CrLf,
            span: span(position(34, 2, 18), position(36, 2, 20)),
        },
        Token {
            kind: RequestTokenKind::CrLf,
            span: span(position(36, 3, 1), position(38, 3, 3)),
        },
        Token {
            kind: RequestTokenKind::Body,
            span: span(position(38, 4, 1), position(42, 4, 1)),
        },
        Token {
            kind: RequestTokenKind::Eof,
            span: span(position(38, 4, 1), position(42, 4, 1)),
        },
    ],
    tokens,
);
```

## Getting Token Text/Lexme From Source Text

A token can be used to extract the text of a token from the source text using `token.slice(source_text)`.

### Example

```rust
use parsehttp::{lex_request, RequestTokenKind, Token};

let request_src = "\
    POST / HTTP/1.1\r\n\
    Host: example.com\r\n\
    \r\n\
    body";

let tokens: Vec<Token<RequestTokenKind>> = lex_request(request_src).unwrap();

assert_eq!(tokens[0].slice(src), "POST");
assert_eq!(tokens[1].slice(src), " ");
assert_eq!(tokens[2].slice(src), "/");
assert_eq!(tokens[3].slice(src), " ");
assert_eq!(tokens[4].slice(src), "HTTP/1.1");
assert_eq!(tokens[5].slice(src), "\r\n");
assert_eq!(tokens[6].slice(src), "Host");
assert_eq!(tokens[7].slice(src), ":");
assert_eq!(tokens[8].slice(src), "example.com");
assert_eq!(tokens[9].slice(src), "\r\n");
assert_eq!(tokens[10].slice(src), "\r\n");
assert_eq!(tokens[11].slice(src), "body");
assert_eq!(tokens[12].slice(src), "body"); // Eof
```
