#[cfg(test)]
mod request_lexer_tests {
    use parsehttp::{lex_request, span, span_position, RequestTokenKind, Token};
    use pretty_assertions::assert_eq;

    #[test]
    fn lex_http_request() {
        let src = "\
            POST / HTTP/1.1\r\n\
            Host: example.com\r\n\
            \r\n\
            body";

        let tokens: Vec<Token<RequestTokenKind>> = lex_request(src).unwrap();

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
    }
}

#[cfg(test)]
mod response_lexer_tests {
    use parsehttp::{lex_response, span, span_position, ResponseTokenKind, Token};
    use pretty_assertions::assert_eq;

    #[test]
    fn lex_http_response() {
        let src = "\
        HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain\r\n\
        \r\n\
        hello";

        let tokens = lex_response(src).unwrap();

        assert_eq!(
            vec![
                Token {
                    kind: ResponseTokenKind::Version,
                    span: span(span_position(0, 1, 1), span_position(8, 1, 9)),
                },
                Token {
                    kind: ResponseTokenKind::Space,
                    span: span(span_position(8, 1, 9), span_position(9, 1, 10)),
                },
                Token {
                    kind: ResponseTokenKind::StatusCode,
                    span: span(span_position(9, 1, 10), span_position(12, 1, 13)),
                },
                Token {
                    kind: ResponseTokenKind::Space,
                    span: span(span_position(12, 1, 13), span_position(13, 1, 14)),
                },
                Token {
                    kind: ResponseTokenKind::ReasonPhrase,
                    span: span(span_position(13, 1, 14), span_position(15, 1, 16)),
                },
                Token {
                    kind: ResponseTokenKind::CrLf,
                    span: span(span_position(15, 1, 16), span_position(17, 1, 18)),
                },
                Token {
                    kind: ResponseTokenKind::HeaderName,
                    span: span(span_position(17, 2, 1), span_position(29, 2, 13)),
                },
                Token {
                    kind: ResponseTokenKind::Colon,
                    span: span(span_position(29, 2, 13), span_position(30, 2, 14)),
                },
                Token {
                    kind: ResponseTokenKind::HeaderValue,
                    span: span(span_position(31, 2, 15), span_position(41, 2, 25)),
                },
                Token {
                    kind: ResponseTokenKind::CrLf,
                    span: span(span_position(41, 2, 25), span_position(43, 2, 27)),
                },
                Token {
                    kind: ResponseTokenKind::CrLf,
                    span: span(span_position(43, 3, 1), span_position(45, 3, 3)),
                },
                Token {
                    kind: ResponseTokenKind::Body,
                    span: span(span_position(45, 4, 1), span_position(50, 4, 1)),
                },
                Token {
                    kind: ResponseTokenKind::Eof,
                    span: span(span_position(45, 4, 1), span_position(50, 4, 1)),
                },
            ],
            tokens
        );

        assert_eq!(tokens[0].slice(src), "HTTP/1.1");
        assert_eq!(tokens[1].slice(src), " ");
        assert_eq!(tokens[2].slice(src), "200");
        assert_eq!(tokens[3].slice(src), " ");
        assert_eq!(tokens[4].slice(src), "OK");
        assert_eq!(tokens[5].slice(src), "\r\n");
        assert_eq!(tokens[6].slice(src), "Content-Type");
        assert_eq!(tokens[7].slice(src), ":");
        assert_eq!(tokens[8].slice(src), "text/plain");
        assert_eq!(tokens[9].slice(src), "\r\n");
        assert_eq!(tokens[10].slice(src), "\r\n");
        assert_eq!(tokens[11].slice(src), "hello"); // Eof

        let body = tokens
            .into_iter()
            .find(|Token { kind, .. }| *kind == ResponseTokenKind::Body)
            .unwrap();

        assert_eq!(body.span.slice(src), "hello");
    }
}
