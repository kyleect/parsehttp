# parsehttp

[![build-artifacts](https://github.com/kyleect/parsehttp/actions/workflows/build-artifacts.yml/badge.svg)](https://github.com/kyleect/parsehttp/actions/workflows/build-artifacts.yml)

## What It Does

- Lex HTTP Request/Response messages into tokens.
- Parse tokens into `HttpRequest` and `HttpResponse`.
- Validates structure but not values

## Documentation

- [Docs](https://kyleect.github.io/parsehttp/)
- [API](https://kyleect.github.io/parsehttp/api/parsehttp/)

## Usage

### CLI

#### Lexing

```sh
cargo run --example lex_request ./requests/post.http

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
cargo run --example parse_request ./requests/post.http

# POST / HTTP/1.1
# Host: example.com

# body
```

#### Parsing As Json

```sh
cargo run --example request_to_json --features json ./requests/post.http > jq .headers

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
use parsehttp::{lex_request, span, span_position, RequestTokenKind, Token};

let src = "\
    POST / HTTP/1.1\r\n\
    Host: example.com\r\n\
    \r\n\
    body";

let tokens: Vec<Token<RequestTokenKind>> = lex_request(src).unwrap();

// Getting token lexmes

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

let body = tokens
    .into_iter()
    .find(|Token { kind, .. }| *kind == RequestTokenKind::Body)
    .unwrap();

assert_eq!(body.span.slice(src), "body");

// A token's span, line, and column information

assert_eq!(
    vec![
        Token {
            kind: RequestTokenKind::Method,
            span: span(span_position(0, 1, 1), span_position(4, 1, 5)),
        },
        Token {
            kind: RequestTokenKind::Space,
            span: span(span_position(4, 1, 5), span_position(5, 1, 6)),
        },
        Token {
            kind: RequestTokenKind::Uri,
            span: span(span_position(5, 1, 6), span_position(6, 1, 7)),
        },
        Token {
            kind: RequestTokenKind::Space,
            span: span(span_position(6, 1, 7), span_position(7, 1, 8)),
        },
        Token {
            kind: RequestTokenKind::Version,
            span: span(span_position(7, 1, 8), span_position(15, 1, 16)),
        },
        Token {
            kind: RequestTokenKind::CrLf,
            span: span(span_position(15, 1, 16), span_position(17, 1, 18)),
        },
        Token {
            kind: RequestTokenKind::HeaderName,
            span: span(span_position(17, 2, 1), span_position(21, 2, 5)),
        },
        Token {
            kind: RequestTokenKind::Colon,
            span: span(span_position(21, 2, 5), span_position(22, 2, 6)),
        },
        Token {
            kind: RequestTokenKind::HeaderValue,
            span: span(span_position(23, 2, 7), span_position(34, 2, 18)),
        },
        Token {
            kind: RequestTokenKind::CrLf,
            span: span(span_position(34, 2, 18), span_position(36, 2, 20)),
        },
        Token {
            kind: RequestTokenKind::CrLf,
            span: span(span_position(36, 3, 1), span_position(38, 3, 3)),
        },
        Token {
            kind: RequestTokenKind::Body,
            span: span(span_position(38, 4, 1), span_position(42, 4, 1)),
        },
        Token {
            kind: RequestTokenKind::Eof,
            span: span(span_position(38, 4, 1), span_position(42, 4, 1)),
        },
    ],
    tokens,
);
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
