#[cfg(test)]
mod request_parser_tests {
    use parsehttp::{
        lex_request, lex_response, parse_request, parse_response, HttpRequest, HttpResponse,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_http_request() {
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
            Ok(HttpResponse {
                status_code: 200.into(),
                status_text: "OK".into(),
                headers: vec![("Content-Type", "text/html; charset=UTF-8").into()],
                body: Some("hello".into())
            }),
            request
        );
    }
}
