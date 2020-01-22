use std::str::Utf8Error;
use std::string::FromUtf8Error;

use failure::Fail;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct ErrorResponse {
    pub command_type: String,
    pub params: Errors,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct Errors {
    pub errors: Vec<ErrorItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct ErrorItem {
    pub field: String,
    pub message: String,
}

#[derive(Debug)]
pub struct ReqwestError(pub reqwest::Error);

#[derive(Debug, Fail)]
pub enum ErrorEnum {
    #[fail(display = "Some I/O Error: {:?}", error)]
    IOError { error: ::std::io::ErrorKind },
    #[fail(display = "Serde error")]
    Serde { error: serde_json::Error },
    #[fail(display = "Utf8Error: {:?}", error)]
    Utf8Error { error: ::std::str::Utf8Error },
    #[fail(display = "ReqwestError: {:?}", error)]
    ReqwestError { error: ReqwestError },
    #[fail(display = "Utf8Error: {:?}", error)]
    MyError { error: String },
}

impl From<std::io::Error> for ErrorEnum {
    fn from(error: std::io::Error) -> Self {
        ErrorEnum::IOError {
            error: error.kind(),
        }
    }
}

impl From<Utf8Error> for ErrorEnum {
    fn from(error: Utf8Error) -> Self {
        ErrorEnum::Utf8Error { error: error }
    }
}

impl From<FromUtf8Error> for ErrorEnum {
    fn from(error: FromUtf8Error) -> Self {
        ErrorEnum::Utf8Error {
            error: error.utf8_error(),
        }
    }
}

impl From<&str> for ErrorEnum {
    fn from(error: &str) -> Self {
        ErrorEnum::MyError {
            error: error.to_string(),
        }
    }
}

impl From<String> for ErrorEnum {
    fn from(error: String) -> Self {
        ErrorEnum::MyError { error: error }
    }
}

impl From<reqwest::Error> for ErrorEnum {
    fn from(error: reqwest::Error) -> Self {
        ErrorEnum::ReqwestError {
            error: ReqwestError(error),
        }
    }
}

impl From<ReqwestError> for ErrorEnum {
    fn from(error: ReqwestError) -> Self {
        ErrorEnum::ReqwestError { error: error }
    }
}

impl PartialEq for ReqwestError {
    fn eq(&self, other: &ReqwestError) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}

impl From<serde_json::Error> for ErrorEnum {
    fn from(error: serde_json::Error) -> ErrorEnum {
        ErrorEnum::Serde { error: error }
    }
}

impl ErrorEnum {
    #[allow(dead_code)]
    pub fn create_myerror(message: &str) -> ErrorEnum {
        ErrorEnum::MyError {
            error: message.to_string(),
        }
        .into()
    }
}
