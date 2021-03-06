use std::str::Utf8Error;
use std::string::FromUtf8Error;

use failure::Fail;
use reqwest;
use serde::{Deserialize, Serialize};

/// Error response from some APIs.
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct ErrorResponse {
    /// Shows where this Json is from.
    pub command_type: String,
    /// Shows errors
    pub params: Errors,
}

/// Shows errors
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct Errors {
    /// Shows errors
    pub errors: Vec<ErrorItem>,
}

/// Shows errors
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct ErrorItem {
    /// Error kind
    pub field: String,
    /// Error detail message
    pub message: String,
}

/// Wrapper for reqwest::Error
#[derive(Debug)]
pub struct ReqwestError(pub reqwest::Error);

/// Enum of errors in this crate.
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Some I/O Error: {:?}", error)]
    IOError { error: ::std::io::ErrorKind },
    #[fail(display = "Serde error")]
    Serde { error: serde_json::Error },
    #[fail(display = "Utf8Error: {:?}", error)]
    Utf8Error { error: ::std::str::Utf8Error },
    #[fail(display = "ReqwestError: {:?}", error)]
    ReqwestError { error: ReqwestError },
    #[fail(display = "AddrParseError: {:?}", error)]
    AddrParseError { error: std::net::AddrParseError },
    #[fail(display = "Utf8Error: {:?}", error)]
    MyError { error: String },
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError {
            error: error.kind(),
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error::Utf8Error { error: error }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::Utf8Error {
            error: error.utf8_error(),
        }
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::MyError {
            error: error.to_string(),
        }
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::MyError { error: error }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError {
            error: ReqwestError(error),
        }
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        Error::ReqwestError { error: error }
    }
}

impl PartialEq for ReqwestError {
    fn eq(&self, other: &ReqwestError) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::Serde { error: error }
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(error: std::net::AddrParseError) -> Error {
        Error::AddrParseError { error: error }
    }
}

impl Error {
    /// Create error message
    #[allow(dead_code)]
    pub fn create_myerror(message: &str) -> Error {
        Error::MyError {
            error: message.to_string(),
        }
        .into()
    }
}
