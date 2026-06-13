# 📄 parsehttp

[![build-artifacts](https://github.com/kyleect/parsehttp/actions/workflows/build-artifacts.yml/badge.svg)](https://github.com/kyleect/parsehttp/actions/workflows/build-artifacts.yml)

```rust
use parsehttp::{lex_request, parse_request, HttpRequest};

let src = "\
    POST /hello HTTP/1.1\r\n\
    Host: example.com\r\n\
    \r\n\
    body";

let tokens = lex_request(src).expect("should produce tokens");
let (request, request_spans) = parse_request(src, tokens).expect("should parse");

assert_eq!(
    HttpRequest::post(
        "/",
        vec![("Host", "example.com").into()],
        Some("body".to_string()),
    ),
    request
);

assert_eq!(
    span(position(0, 1, 1), position(4, 1, 5)),
    request_spans.method
);
```

## What It Does

- Parse source text in to `HttpRequest` and `HttpResponse`
- Validates request/response structure but not literal values
- Provides span, line, and column information
- Use tokens to get slices of the source text

## Documentation

- [Docs](https://kyleect.github.io/parsehttp/)
- [API](https://kyleect.github.io/parsehttp/api/parsehttp/)
