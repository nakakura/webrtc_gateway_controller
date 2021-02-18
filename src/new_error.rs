use reqwest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Enum of errors in this crate.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", 0)]
    IOError { error: ::std::io::ErrorKind },
    #[error("{}", 0)]
    SerdeError { error: serde_json::Error },
    #[error("{}", 0)]
    ReqwestError(#[from] reqwest::Error),
    #[error("{}", 0)]
    LocalError(String),
}

impl Error {
    /// Create error message to show internal error
    #[allow(dead_code)]
    pub fn create_local_error(message: &str) -> Error {
        Error::LocalError(message.into())
    }
}

/// Error response from some APIs.
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub(crate) struct ErrorResponse {
    /// Shows where this Json is from.
    pub command_type: String,
    /// Shows errors
    pub params: Errors,
}

/// Shows errors
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub(crate) struct Errors {
    /// Shows errors
    pub errors: Vec<ErrorItem>,
}

/// Shows errors
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub(crate) struct ErrorItem {
    /// Error kind
    pub field: String,
    /// Error detail message
    pub message: String,
}
