use std::fmt::Display;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use crate::parsing::models::{HttpHeader, HttpVersion, Uri};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    Other(String),
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "PATCH" => HttpMethod::PATCH,
            "DELETE" => HttpMethod::DELETE,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            _ => HttpMethod::Other(value.to_string()),
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTION",
            HttpMethod::Other(string) => string,
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod test_from_str_to_http_method {
    use rstest::rstest;

    use crate::parsing::models::HttpMethod;

    #[rstest]
    fn from_str_to_http_method(
        #[values(
            ("GET", HttpMethod::GET),
            ("POST", HttpMethod::POST),
            ("PUT", HttpMethod::PUT),
            ("PATCH", HttpMethod::PATCH),
            ("DELETE", HttpMethod::DELETE),
            ("HEAD", HttpMethod::HEAD),
            ("OPTIONS", HttpMethod::OPTIONS),
            ("OTHER", HttpMethod::Other(String::from("OTHER"))),
        )]
        pair: (&str, HttpMethod),
    ) {
        let (input, expected) = pair;

        assert_eq!(expected, input.into())
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HttpRequest {
    pub uri: Uri,
    pub method: HttpMethod,
    pub http_version: HttpVersion,
    pub headers: Vec<HttpHeader>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn get(uri: &str, headers: Vec<HttpHeader>) -> Self {
        Self {
            uri: uri.into(),
            method: HttpMethod::GET,
            http_version: Default::default(),
            headers,
            body: None,
        }
    }

    pub fn post(uri: &str, headers: Vec<HttpHeader>, body: Option<String>) -> Self {
        Self {
            uri: uri.into(),
            method: HttpMethod::POST,
            headers,
            body,
            http_version: Default::default(),
        }
    }

    pub fn headers(&self) -> &Vec<HttpHeader> {
        &self.headers
    }

    pub fn get_header(&self, key: &str) -> Option<&HttpHeader> {
        self.headers.iter().find(|header| header.key() == key)
    }

    /// Set or update header by key
    pub fn set_header(&mut self, key: &str, value: &str) {
        let existing_header: Option<&mut HttpHeader> = self.get_header_mut(key);
        if let Some(header) = existing_header {
            *header = (key, value).into();
        } else {
            self.headers.push((key, value).into());
        }
    }

    pub fn get_header_mut(&mut self, key: &str) -> Option<&mut HttpHeader> {
        self.headers.iter_mut().find(|header| header.key() == key)
    }

    pub fn get_body(&self) -> &Option<String> {
        &self.body
    }
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = if self.headers.is_empty() {
            None
        } else {
            Some(format!(
                "{}\r\n",
                self.headers
                    .clone()
                    .into_iter()
                    .map(|x| format!("{}: {}", x.key(), x.value()))
                    .collect::<Vec<String>>()
                    .join("\r\n")
                    .trim_end()
            ))
        };

        let body = self
            .body
            .clone()
            .and_then(|x| if x.is_empty() { None } else { Some(x) });

        let the_rest = match (&headers, &body) {
            (Some(headers), Some(body)) => format!("{headers}\r\n{body}"),
            (Some(headers), None) => format!("{}\r\n", headers),
            (None, Some(body)) => format!("\r\n{body}"),
            (None, None) => String::new(),
        };

        write!(
            f,
            "{} {} {}\r\n{}",
            self.method, self.uri, self.http_version, the_rest
        )
    }
}

#[cfg(test)]
mod request_tests {
    use crate::parsing::models::{HttpHeader, HttpMethod, HttpRequest};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_request_to_string() {
        let actual = format!(
            "{}",
            HttpRequest::get("/hello", vec![("Host", "example.com").into()])
        );

        let expected = "\
            GET /hello HTTP/1.1\r\n\
            Host: example.com\r\n\
            \r\n";

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_post_request_to_string() {
        let actual = format!(
            "{}",
            HttpRequest::post(
                "/hello",
                vec![("Host", "example.com").into()],
                Some("body".to_string()),
            )
        );

        let expected = "\
            POST /hello HTTP/1.1\r\n\
            Host: example.com\r\n\
            \r\n\
            body";

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_request_with_headers() {
        let mut request = HttpRequest::get(
            "https://example.com",
            vec!["Authorization: Bearer token".into()],
        );

        request.set_header("X-API-Key", "API Key");

        assert_eq!(
            request.get_header("Authorization").unwrap().value(),
            "Bearer token"
        );

        assert_eq!(request.get_header("X-API-Key").unwrap().value(), "API Key");

        let expected_headers_in_order: Vec<HttpHeader> = vec![
            "Authorization: Bearer token".into(),
            ("X-API-Key", "API Key").into(),
        ];

        assert_eq!(&expected_headers_in_order, request.headers())
    }

    #[test]
    fn test_request_get() {
        let request = HttpRequest::get(
            "https://example.com",
            vec!["User-Agent: curl/7.64.1".into()],
        );
        assert_eq!(request.method, HttpMethod::GET);
        assert!(request.body.is_none());

        let expected_headers_in_order: Vec<HttpHeader> = vec!["User-Agent: curl/7.64.1".into()];

        assert_eq!(expected_headers_in_order, *request.headers())
    }

    #[test]
    fn test_request_post() {
        let headers = vec!["Content-Type: application/json".into()];
        let body = Some("{\"key\": \"value\"}".to_string());
        let request = HttpRequest::post("https://example.com", headers, body);
        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(
            request.get_body(),
            &Some("{\"key\": \"value\"}".to_string())
        );

        let expected_headers_in_order: Vec<HttpHeader> =
            vec!["Content-Type: application/json".into()];

        assert_eq!(expected_headers_in_order, *request.headers())
    }
}
