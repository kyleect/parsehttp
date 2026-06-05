#[cfg(test)]
mod request_parser_tests {
    use parsehttp::{lex_request, parse_request, HttpRequest};
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
}
