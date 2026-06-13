#[cfg(test)]
mod request_parser_tests {
    use parsehttp::{
        lex_request, lex_response, parse_request, parse_response, position, span, HttpRequest,
        HttpRequestSpans, HttpResponse, HttpResponseSpans,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_http_request() {
        let src = "\
            POST / HTTP/1.1\r\n\
            Host: example.com\r\n\
            \r\n\
            body";

        let tokens = lex_request(src).expect("should produce tokens");
        let request = parse_request(src, tokens);

        assert_eq!(
            Ok((
                HttpRequest::post(
                    "/",
                    vec![("Host", "example.com").into()],
                    Some("body".to_string()),
                ),
                HttpRequestSpans {
                    method: span(position(0, 1, 1), position(4, 1, 5)),
                    uri: span(position(5, 1, 6), position(6, 1, 7)),
                    http_version: span(position(7, 1, 8), position(15, 1, 16)),
                    headers: vec![span(position(17, 2, 1), position(34, 2, 18))],
                    body: Some(span(position(38, 4, 1), position(42, 4, 1)))
                }
            )),
            request
        );
    }

    #[test]
    fn parse_http_response() {
        let src = "\
            HTTP/1.1 200 OK\r\n\
            Content-Type: text/html; charset=UTF-8\r\n\
            \r\n\
            hello";

        let tokens = lex_response(src).expect("should produce tokens");
        let request = parse_response(src, tokens);

        assert_eq!(
            Ok((
                HttpResponse {
                    version: "HTTP/1.1".into(),
                    status_code: 200.into(),
                    status_text: "OK".into(),
                    headers: vec![("Content-Type", "text/html; charset=UTF-8").into()],
                    body: Some("hello".into())
                },
                HttpResponseSpans {
                    http_version: span(position(0, 1, 1), position(8, 1, 9)),
                    status_code: span(position(9, 1, 10), position(12, 1, 13)),
                    status_text: span(position(13, 1, 14), position(15, 1, 16)),
                    headers: vec![span(position(17, 2, 1), position(55, 2, 39))],
                    body: Some(span(position(59, 4, 1), position(64, 4, 1)))
                }
            )),
            request
        );
    }
}
