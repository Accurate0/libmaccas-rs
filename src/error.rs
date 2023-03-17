use std::{error::Error, fmt::Display, num::ParseIntError};

use http::StatusCode;

#[derive(Debug)]
pub enum ClientError {
    RequestOrMiddlewareError(reqwest_middleware::Error),
    RequestError(reqwest::Error),
    Other(anyhow::Error),
}

impl From<anyhow::Error> for ClientError {
    fn from(e: anyhow::Error) -> Self {
        Self::Other(e)
    }
}

impl From<reqwest_middleware::Error> for ClientError {
    fn from(e: reqwest_middleware::Error) -> Self {
        Self::RequestOrMiddlewareError(e)
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestError(e)
    }
}

impl From<ParseIntError> for ClientError {
    fn from(e: ParseIntError) -> Self {
        Self::Other(e.into())
    }
}

impl ClientError {
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            ClientError::RequestOrMiddlewareError(e) => match e {
                reqwest_middleware::Error::Middleware(_) => None,
                reqwest_middleware::Error::Reqwest(e) => e.status(),
            },
            ClientError::RequestError(e) => e.status(),
            ClientError::Other(_) => None,
        }
    }
}

impl Error for ClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ClientError::RequestOrMiddlewareError(e) => Some(e),
            ClientError::RequestError(e) => Some(e),
            ClientError::Other(e) => e.source(),
        }
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::RequestOrMiddlewareError(e) => e.fmt(f),
            ClientError::RequestError(e) => e.fmt(f),
            ClientError::Other(e) => e.fmt(f),
        }
    }
}
