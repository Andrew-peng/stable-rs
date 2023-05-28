use std::error;
use std::fmt;

use serde_json;

use crate::client;

/// Error type that occurs when an API request fails for some reason.
#[derive(Debug)]
pub enum StabilityError {
    /// Occurs when the API has returned a non-success error code.
    Status(client::Response),
    /// Occurs if the HTTP response from Stability was corrupt and
    /// reqwest could not parse it.
    Network(client::Error),
    /// Occurs if serde could not Deserialize the response.
    Parse(serde_json::Error),
    /// Occurs if there is a grant error.
    Auth(String),
    InvalidHeader(reqwest::header::InvalidHeaderValue),
}

impl From<client::Error> for StabilityError {
    fn from(e: client::Error) -> Self {
        StabilityError::Network(e)
    }
}

impl From<serde_json::Error> for StabilityError {
    fn from(e: serde_json::Error) -> Self {
        StabilityError::Parse(e)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for StabilityError {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        StabilityError::InvalidHeader(e)
    }
}

impl fmt::Display for StabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StabilityError::Status(ref err) => write!(f, "Status error: {}", err.status()),
            StabilityError::Network(ref err) => err.fmt(f),
            StabilityError::Parse(ref err) => err.fmt(f),
            StabilityError::Auth(ref err) => write!(f, "Auth error: {}", err),
            StabilityError::InvalidHeader(ref err) => write!(f, "Invalid header: {}", err),
        }
    }
}

impl error::Error for StabilityError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            StabilityError::Status(_) => None,
            StabilityError::Auth(_) => None,
            StabilityError::Network(ref err) => Some(err),
            StabilityError::Parse(ref err) => Some(err),
            StabilityError::InvalidHeader(ref err) => Some(err),
        }
    }
}