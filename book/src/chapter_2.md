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
