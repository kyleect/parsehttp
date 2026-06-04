#[cfg(test)]
mod request_lexer_tests {
    use http_lexer_project::{lex_request, RequestTokenKind, Span, Token};
    use pretty_assertions::assert_eq;

    #[test]
    fn lex_http_request() {
        let src = "\
            GET /hello HTTP/1.1\r\n\
            Host: example.com\r\n\
            \r\n\
            body";

        let tokens: Vec<Token<RequestTokenKind>> = lex_request(src).unwrap();

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
    }
}

#[cfg(test)]
mod response_lexer_tests {
    use http_lexer_project::{HttpResponseLexer, Lexer, ResponseTokenKind, Span, Token};
    use pretty_assertions::assert_eq;

    #[test]
    fn lex_http_response() {
        let src = "\
        HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain\r\n\
        \r\n\
        hello";

        let tokens = HttpResponseLexer::new(src).lex().unwrap();

        assert_eq!(
            vec![
                Token {
                    kind: ResponseTokenKind::Version,
                    span: Span::builder().span(0..8).line(1).column(9).build()
                },
                Token {
                    kind: ResponseTokenKind::Space,
                    span: Span::builder().span(8..9).line(1).column(10).build()
                },
                Token {
                    kind: ResponseTokenKind::StatusCode,
                    span: Span::builder().span(9..12).line(1).column(13).build()
                },
                Token {
                    kind: ResponseTokenKind::Space,
                    span: Span::builder().span(12..13).line(1).column(14).build()
                },
                Token {
                    kind: ResponseTokenKind::ReasonPhrase,
                    span: Span::builder().span(13..15).line(1).column(16).build()
                },
                Token {
                    kind: ResponseTokenKind::CrLf,
                    span: Span::builder().span(15..17).line(1).column(18).build()
                },
                Token {
                    kind: ResponseTokenKind::HeaderName,
                    span: Span::builder().span(17..29).line(2).column(13).build()
                },
                Token {
                    kind: ResponseTokenKind::Colon,
                    span: Span::builder().span(29..30).line(2).column(14).build()
                },
                Token {
                    kind: ResponseTokenKind::HeaderValue,
                    span: Span::builder().span(31..41).line(2).column(25).build()
                },
                Token {
                    kind: ResponseTokenKind::CrLf,
                    span: Span::builder().span(41..43).line(2).column(27).build()
                },
                Token {
                    kind: ResponseTokenKind::CrLf,
                    span: Span::builder().span(43..45).line(3).column(3).build()
                },
                Token {
                    kind: ResponseTokenKind::Body,
                    span: Span::builder().span(45..50).line(4).column(1).build()
                },
                Token {
                    kind: ResponseTokenKind::Eof,
                    span: Span::builder().span(45..50).line(4).column(1).build()
                },
            ],
            tokens
        );

        assert_eq!(tokens[0].span.slice(src), "HTTP/1.1");
        assert_eq!(tokens[1].span.slice(src), " ");
        assert_eq!(tokens[2].span.slice(src), "200");
        assert_eq!(tokens[3].span.slice(src), " ");
        assert_eq!(tokens[4].span.slice(src), "OK");
        assert_eq!(tokens[5].span.slice(src), "\r\n");
        assert_eq!(tokens[6].span.slice(src), "Content-Type");
        assert_eq!(tokens[7].span.slice(src), ":");
        assert_eq!(tokens[8].span.slice(src), "text/plain");
        assert_eq!(tokens[9].span.slice(src), "\r\n");
        assert_eq!(tokens[10].span.slice(src), "\r\n");
        assert_eq!(tokens[11].span.slice(src), "hello");

        let body = tokens
            .into_iter()
            .find(|Token { kind, .. }| *kind == ResponseTokenKind::Body)
            .unwrap();

        assert_eq!(body.span.slice(src), "hello");
    }
}
