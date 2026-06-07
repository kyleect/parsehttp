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

## What It Does

- Lex HTTP Request/Response messages into tokens.
- Parse tokens into `HttpRequest` and `HttpResponse`.
- Validates structure but not values

## Documentation

- [Docs](https://kyleect.github.io/parsehttp/)
- [API](https://kyleect.github.io/parsehttp/api/parsehttp/)
