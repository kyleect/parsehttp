# parsehttp CLI

## Lexing

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

## Parsing

```sh
cargo run --example parse_request ./requests/post.http

# POST / HTTP/1.1
# Host: example.com

# body
```

## Parsing As Json

```sh
cargo run --example request_to_json --features json ./requests/post.http > jq .headers

# [
#   [
#     "Host",
#     "example.com"
#   ]
# ]
```
