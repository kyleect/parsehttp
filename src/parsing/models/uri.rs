use core::fmt;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// The URI in an HTTP request
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HttpUri(String);

impl HttpUri {
    pub fn new(uri: &str) -> Self {
        let uri = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri
        } else if uri.starts_with("/") {
            uri
        } else {
            &format!("https://{uri}")
        };

        Self(uri.to_string())
    }
}

impl fmt::Display for HttpUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for HttpUri {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
