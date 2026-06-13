# Parsing

Convert a `Vec<Token<T>` in to either an `parsehttp::HttpRequest` or `parsehttp::HttpResponse`.

```rust
pub struct HttpRequest {
    pub uri: HttpUri,
    pub method: HttpMethod,
    pub http_version: HttpVersion,
    pub headers: Vec<HttpHeader>,
    pub body: Option<String>,
    pub spans: HttpRequestSpans,
}

pub struct HttpMethod(String);

pub struct HttpUri(String);

pub struct HttpVersion(String);

pub struct HttpHeader(String, String);

pub struct HttpRequestSpans {
    pub uri: Span,
    pub method: Span,
    pub http_version: Span,
    pub headers: Vec<Span>,
    pub body: Option<Span>,
}
```

## Example

```rust
use parsehttp::{lex_request, parse_request, HttpRequest};

let src = "\
    POST /hello HTTP/1.1\r\n\
    Host: example.com\r\n\
    \r\n\
    body";

let tokens = lex_request(src).expect("should produce tokens");
let result = parse_request(src, tokens);

assert_eq!(
    Ok((
        // The parsed request
        HttpRequest::post(
            "/",
            vec![("Host", "example.com").into()],
            Some("body".to_string()),
        ),
        // The parsed span information
        HttpRequestSpans {
            method: span(position(0, 1, 1), position(4, 1, 5)),
            uri: span(position(5, 1, 6), position(6, 1, 7)),
            http_version: span(position(7, 1, 8), position(15, 1, 16)),
            headers: vec![span(position(17, 2, 1), position(34, 2, 18))],
            body: Some(span(position(38, 4, 1), position(42, 4, 1)))
        }
    )),
    result
);
```
