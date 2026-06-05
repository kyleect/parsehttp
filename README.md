# HTTP Message Parser

## What It Does

- Lex HTTP Request/Response messages into tokens.
- Parse tokens into `HttpRequest` and `HttpResponse`.

## Usage

### CLI

#### Lexing

```sh
cargo run --example lex ./requests/post.http

# Method
# Space
# Uri
# Space
# HttpVersion
# CrLf
# HeaderName
# :
# HeaderValue
# CrLf
# CrLf
# Body
# Eof

```

#### Parsing

```sh
cargo run --example parse ./requests/post.http

# POST / HTTP/1.1
# Host: example.com

# body
```

#### Parsing As Json

```sh
cargo run --example json --features json ./requests/post.http > jq .headers

# [
#   [
#     "Host",
#     "example.com"
#   ]
# ]
```

### Library

#### Lexing

```rust
use parsehttp::{lex_request, RequestTokenKind, Span, Token};

let src = "\
    GET /hello HTTP/1.1\r\n\
    Host: example.com\r\n\
    \r\n\
    body";

let tokens: Vec<Token<RequestTokenKind>> = lex_request(src).expect("should produce tokens");

assert_eq!(
    vec![
        Token {
            kind: RequestTokenKind::Method,
            span: Span::builder().span(0..3).line(1).column(4).build()
        },
        Token {
            kind: RequestTokenKind::Space,
            span: Span::builder().span(3..4).line(1).column(5).build()
        },
        Token {
            kind: RequestTokenKind::Uri,
            span: Span::builder().span(4..10).line(1).column(11).build()
        },
        Token {
            kind: RequestTokenKind::Space,
            span: Span::builder().span(10..11).line(1).column(12).build()
        },
        Token {
            kind: RequestTokenKind::Version,
            span: Span::builder().span(11..19).line(1).column(20).build()
        },
        Token {
            kind: RequestTokenKind::CrLf,
            span: Span::builder().span(19..21).line(1).column(22).build()
        },
        Token {
            kind: RequestTokenKind::HeaderName,
            span: Span::builder().span(21..25).line(2).column(5).build()
        },
        Token {
            kind: RequestTokenKind::Colon,
            span: Span::builder().span(25..26).line(2).column(6).build()
        },
        Token {
            kind: RequestTokenKind::HeaderValue,
            span: Span::builder().span(27..38).line(2).column(18).build()
        },
        Token {
            kind: RequestTokenKind::CrLf,
            span: Span::builder().span(38..40).line(2).column(20).build()
        },
        Token {
            kind: RequestTokenKind::CrLf,
            span: Span::builder().span(40..42).line(3).column(3).build()
        },
        Token {
            kind: RequestTokenKind::Body,
            span: Span::builder().span(42..46).line(4).column(1).build()
        },
        Token {
            kind: RequestTokenKind::Eof,
            span: Span::builder().span(42..46).line(4).column(1).build()
        },
    ],
    tokens
);

assert_eq!(tokens[0].span.slice(src), "GET");
assert_eq!(tokens[1].span.slice(src), " ");
assert_eq!(tokens[2].span.slice(src), "/hello");
assert_eq!(tokens[3].span.slice(src), " ");
assert_eq!(tokens[4].span.slice(src), "HTTP/1.1");
assert_eq!(tokens[5].span.slice(src), "\r\n");
assert_eq!(tokens[6].span.slice(src), "Host");
assert_eq!(tokens[7].span.slice(src), ":");
assert_eq!(tokens[8].span.slice(src), "example.com");
assert_eq!(tokens[9].span.slice(src), "\r\n");
assert_eq!(tokens[10].span.slice(src), "\r\n");
assert_eq!(tokens[11].span.slice(src), "body");
assert_eq!(tokens[12].span.slice(src), "body");

let body = tokens
    .into_iter()
    .find(|Token { kind, .. }| *kind == RequestTokenKind::Body)
    .unwrap();

assert_eq!(body.span.slice(src), "body");
```

#### Parsing

```rust
use parsehttp::{lex_request, parse_request, HttpRequest};

let src = "\
    POST /hello HTTP/1.1\r\n\
    Host: example.com\r\n\
    \r\n\
    body";

let tokens = lex_request(src).expect("should produce tokens");
let request = parse_request(src, tokens);

assert_eq!(
    Ok(HttpRequest::post(
        "/hello",
        vec![("Host", "example.com").into()],
        Some("body".to_string())
    )),
    request
);
```
