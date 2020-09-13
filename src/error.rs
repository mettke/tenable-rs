use http::{header::InvalidHeaderValue, status::StatusCode};
use std::{error, fmt};

/// Possible error types occuring in this library
#[derive(Debug)]
pub enum Error<RE: fmt::Debug> {
    /// AccessKey or SecretKey contains invalid characters.
    InvalidAuth(InvalidHeaderValue),
    /// Unable to build Http Request.
    Http(http::Error),
    /// User is not allowed to perform this operation.
    InsufficientPermission,
    /// Rate Limit reached. Try again later.
    RateLimitReached,
    /// The Backoff function reached a number to high to represent while waiting
    MaximumWaitTimeReached,
    /// API returned unexpected status code.
    UnexpectedStatusCode(StatusCode),
    /// Error in inner request client.
    Request(RE),
    /// Unable to transform response to concret type.
    Deserialization(serde_json::Error),
}

impl<RE: 'static + fmt::Debug + error::Error> error::Error for Error<RE> {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::InvalidAuth(err) => Some(err),
            Self::Http(err) => Some(err),
            Self::Request(err) => Some(err),
            Self::Deserialization(err) => Some(err),
            Self::InsufficientPermission
            | Self::RateLimitReached
            | Self::MaximumWaitTimeReached
            | Self::UnexpectedStatusCode(_) => None,
        }
    }
}

impl<RE: fmt::Debug> fmt::Display for Error<RE> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAuth(_) => {
                write!(f, "AccessKey or SecretKey contains invalid characters.")
            }
            Self::Http(_) => write!(f, "Unable to build Http Request."),
            Self::InsufficientPermission => {
                write!(f, "User is not allowed to perform this operation.")
            }
            Self::RateLimitReached => write!(f, "Rate Limit reached. Try again later."),
            Self::MaximumWaitTimeReached => write!(
                f,
                "The Backoff function reached a number to high to represent while waiting."
            ),
            Self::UnexpectedStatusCode(code) => {
                write!(f, "API returned unexpected status code: {}.", code)
            }
            Self::Request(_) => write!(f, "Error in inner request client."),
            Self::Deserialization(_) => write!(f, "Unable to transform response to concret type."),
        }
    }
}

impl<RE: fmt::Debug> From<InvalidHeaderValue> for Error<RE> {
    #[inline]
    fn from(err: InvalidHeaderValue) -> Self {
        Self::InvalidAuth(err)
    }
}

impl<RE: fmt::Debug> From<http::Error> for Error<RE> {
    #[inline]
    fn from(err: http::Error) -> Self {
        Self::Http(err)
    }
}

impl<RE: fmt::Debug> From<serde_json::Error> for Error<RE> {
    #[inline]
    fn from(err: serde_json::Error) -> Self {
        Self::Deserialization(err)
    }
}
