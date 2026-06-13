use core::fmt;
use std::fmt::Display;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use crate::{parsing::models::HttpHeader, position, span, HttpVersion, Span};

/// A parsed HTTP Response
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub status_code: HttpStatusCode,
    pub status_text: HttpStatusText,
    pub headers: Vec<HttpHeader>,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new(
        version: HttpVersion,
        status_code: HttpStatusCode,
        status_text: HttpStatusText,
        headers: Vec<HttpHeader>,
        body: Option<&str>,
    ) -> Self {
        Self {
            version,
            status_code,
            status_text,
            headers,
            body: body.map(|b| b.to_string()),
        }
    }

    pub fn headers(&self) -> &Vec<HttpHeader> {
        &self.headers
    }

    pub fn get_header(&self, key: &str) -> Option<&HttpHeader> {
        self.headers.iter().find(|header| header.key() == key)
    }

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

    pub fn set_body(&mut self, value: Option<String>) {
        self.body = value;
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = if self.headers.is_empty() {
            None
        } else {
            Some(format!(
                "{}\n",
                self.headers
                    .clone()
                    .into_iter()
                    .map(|x| format!("{}: {}", x.key(), x.value()))
                    .collect::<Vec<String>>()
                    .join("\n")
                    .trim_end()
            ))
        };

        let body = self
            .body
            .clone()
            .and_then(|x| if x.is_empty() { None } else { Some(x) });

        let the_rest = match (&headers, &body) {
            (Some(headers), Some(body)) => format!("{headers}\n{body}"),
            (Some(headers), None) => headers.to_string(),
            (None, Some(body)) => format!("\n{body}"),
            (None, None) => String::new(),
        };

        write!(f, "HTTP/1.1 {} OK\r\n{}", self.status_code, the_rest)
    }
}

/// Numeric status code of HTTP response
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HttpStatusCode(u16);

impl HttpStatusCode {
    pub fn new(status_code: u16) -> Self {
        Self(status_code)
    }
}

impl fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for HttpStatusCode {
    fn from(value: u16) -> Self {
        HttpStatusCode(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HttpStatusText(String);

impl HttpStatusText {
    pub fn new(status_text: &str) -> Self {
        status_text.into()
    }
}

impl fmt::Display for HttpStatusText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for HttpStatusText {
    fn from(value: String) -> Self {
        HttpStatusText(value)
    }
}

impl From<&str> for HttpStatusText {
    fn from(value: &str) -> Self {
        HttpStatusText(value.to_string())
    }
}

/// Span information for a response
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HttpResponseSpans {
    pub http_version: Span,
    pub status_code: Span,
    pub status_text: Span,
    pub headers: Vec<Span>,
    pub body: Option<Span>,
}

impl Default for HttpResponseSpans {
    fn default() -> Self {
        Self {
            http_version: span(position(0, 0, 0), position(0, 0, 0)),
            status_code: span(position(0, 0, 0), position(0, 0, 0)),
            status_text: span(position(0, 0, 0), position(0, 0, 0)),
            headers: vec![span(position(0, 0, 0), position(0, 0, 0))],
            body: Some(span(position(0, 0, 0), position(0, 0, 0))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_status_code_new() {
        let status_code = HttpStatusCode::new(200);
        assert_eq!(status_code.0, 200);
    }

    #[test]
    fn test_http_status_code_display() {
        let status_code = HttpStatusCode::new(200);
        assert_eq!(format!("{status_code}"), "200");
    }

    #[test]
    fn test_http_status_code_from() {
        let status_code: HttpStatusCode = 200.into();
        assert_eq!(status_code.0, 200);
    }

    #[test]
    fn test_http_response_new() {
        let headers = vec!["Content-Type: application/json".into()];
        let body = Some("{\"message\": \"Hello, world!\"}");
        let response = HttpResponse::new(
            "HTTP/1.1".into(),
            200.into(),
            "OK".into(),
            headers.clone(),
            body,
        );

        assert_eq!(response.status_code.0, 200);
        assert_eq!(response.headers.len(), 1);
        assert_eq!(response.headers[0].key(), "Content-Type");
        assert_eq!(response.headers[0].value(), "application/json");
        assert_eq!(response.body, body.map(|b| b.to_string()));
    }

    #[test]
    fn test_http_response_headers() {
        let response = HttpResponse::new(
            "HTTP/1.1".into(),
            200.into(),
            "OK".into(),
            vec!["Content-Type: application/json".into()].clone(),
            None,
        );

        let expected_headers_in_order: Vec<HttpHeader> =
            vec!["Content-Type: application/json".into()];

        assert_eq!(&expected_headers_in_order, response.headers());
    }

    #[test]
    fn test_http_response_get_header() {
        let headers = vec!["Content-Type: application/json".into()];
        let response = HttpResponse::new(
            "HTTP/1.1".into(),
            200.into(),
            "OK".into(),
            headers.clone(),
            None,
        );
        let header = response.get_header("Content-Type");
        assert_eq!(
            Some(&HttpHeader::new("Content-Type", "application/json")),
            header
        );
    }

    #[test]
    fn test_http_response_set_header() {
        let mut response = HttpResponse::new(
            "HTTP/1.1".into(),
            200.into(),
            "OK".into(),
            vec!["Content-Type: application/json".into()],
            None,
        );

        response.set_header("Content-Type", "text/plain");

        let header = response.get_header("Content-Type").unwrap();

        assert_eq!(header.value(), "text/plain");
    }

    // #[test]
    // fn test_http_response_get_header_mut() {
    //     let mut response = HttpResponse::new(
    //         HttpStatusCode::OK,
    //         vec!["Content-Type: application/json".into()],
    //         None,
    //     );

    //     if let Some(header) = response.get_header_mut("Content-Type") {
    //         header.set_value("text/plain");
    //     }

    //     let header = response.get_header("Content-Type").unwrap();
    //     assert_eq!(header.value(), "application/json");
    // }

    #[test]
    fn test_http_response_get_body() {
        let body = Some("{\"message\": \"Hello, world!\"}");
        let response = HttpResponse::new("HTTP/1.1".into(), 200.into(), "OK".into(), vec![], body);
        assert_eq!(response.get_body(), &body.map(|b| b.to_string()));
    }

    #[test]
    fn test_http_response_set_body() {
        let mut response =
            HttpResponse::new("HTTP/1.1".into(), 200.into(), "OK".into(), vec![], None);
        let new_body = Some("{\"message\": \"Goodbye, world!\"}").map(|b| b.to_string());
        response.set_body(new_body.clone());
        assert_eq!(response.get_body(), &new_body);
    }
}
