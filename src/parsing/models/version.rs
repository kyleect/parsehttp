use core::fmt;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// HTTP version string
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HttpVersion(String);

impl HttpVersion {
    pub fn new(version: &str) -> Self {
        Self(version.to_string())
    }
}

impl Default for HttpVersion {
    fn default() -> Self {
        Self("HTTP/1.1".to_string())
    }
}

impl From<&str> for HttpVersion {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod http_version_tests {
    use super::*;

    #[test]
    fn test_default_value() {
        let version = HttpVersion::default();
        assert_eq!(version.to_string(), "HTTP/1.1");
    }

    #[test]
    fn test_from_str_with_http_prefix() {
        let version = HttpVersion::from("HTTP/1.1");
        assert_eq!(version.to_string(), "HTTP/1.1");
    }

    #[test]
    fn test_from_str_without_http_prefix() {
        let version = HttpVersion::from("1.1");
        assert_eq!(version.to_string(), "1.1");
    }
}
