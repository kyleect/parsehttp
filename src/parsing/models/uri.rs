use core::fmt;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// The URI in an HTTP request
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HttpUri(String);

impl HttpUri {
    pub fn new(uri: &str) -> Self {
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
