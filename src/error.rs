use reqwest;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use thiserror::Error;

/// Enum of errors in this crate.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", 0)]
    IOError { error: ::std::io::ErrorKind },
    #[error("{}", 0)]
    Utf8Error { error: ::std::str::Utf8Error },
    #[error("{}", 0)]
    SerdeError { error: serde_json::Error },
    #[error("{}", 0)]
    ReqwestError(#[from] reqwest::Error),
    #[error("{}", 0)]
    AddrParseError(#[from] std::net::AddrParseError),
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

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 2)?;
        match self {
            Error::IOError { error } => {
                state.serialize_field("reason", "IoError")?;
                state.serialize_field(
                    "message",
                    &format!("{}", std::io::Error::from(error.clone())),
                )?;
            }
            Error::Utf8Error { error } => {
                state.serialize_field("reason", "IoError")?;
                state.serialize_field("message", &format!("{}", error))?;
            }
            Error::SerdeError { error } => {
                state.serialize_field("reason", "JsonError")?;
                state.serialize_field("message", &format!("{}", error))?;
            }
            Error::ReqwestError(error) => {
                state.serialize_field("reason", "NetworkError")?;
                state.serialize_field("message", &format!("{}", error))?;
            }
            Error::AddrParseError(error) => {
                state.serialize_field("reason", "InvalidAddressError")?;
                state.serialize_field("message", &format!("{}", error))?;
            }
            Error::LocalError(error) => {
                state.serialize_field("reason", "InternalError")?;
                state.serialize_field("message", &format!("{}", error))?;
            }
        }
        state.end()
    }
}

#[cfg(test)]
mod serialize_error {
    use serde_json::Value;

    use super::*;
    use std::net::IpAddr;

    #[test]
    fn io_error() {
        let expected = serde_json::from_str::<Value>(
            "{\"message\":\"address in use\", \"reason\":\"IoError\"}",
        )
        .unwrap();

        let error = Error::IOError {
            error: std::io::ErrorKind::AddrInUse,
        };
        let message = serde_json::to_string(&error).unwrap();
        let message = serde_json::from_str::<Value>(&message).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn utf8_error() {
        use std::str;

        let expected = serde_json::from_str::<Value>(
            "{\"reason\":\"IoError\",\"message\":\"invalid utf-8 sequence of 1 bytes from index 1\"}"
        ).unwrap();

        let wrong_binary = vec![0, 159, 146, 150];
        let utf8_error_message = str::from_utf8(&wrong_binary).unwrap_err();
        let utf8_error_enum = Error::Utf8Error {
            error: utf8_error_message,
        };
        let message = serde_json::to_string(&utf8_error_enum).unwrap();
        let message = serde_json::from_str::<Value>(&message).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn serde_error() {
        let expected = serde_json::from_str::<Value>(
            "{\"reason\":\"JsonError\", \"message\":\"expected value at line 1 column 1\"}",
        )
        .unwrap();

        let serde_error = serde_json::from_str::<Value>("invalid json").unwrap_err();
        let serde_error_enum = Error::SerdeError { error: serde_error };
        let message = serde_json::to_string(&serde_error_enum).unwrap();
        let message = serde_json::from_str::<Value>(&message).unwrap();

        assert_eq!(expected, message);
    }

    #[tokio::test]
    async fn reqwest_error() {
        let expected = serde_json::from_str::<Value>(
            r#"{
                "reason":"NetworkError",
                "message": "error sending request for url (http://localhost:0/): error trying to connect: tcp connect error: Connection refused (os error 111)"
               }"#
        )
        .unwrap();

        let reqwest_error = reqwest::get("http://localhost:0").await.unwrap_err();
        let reqwest_error_enum = Error::ReqwestError(reqwest_error);
        let message = serde_json::to_string(&reqwest_error_enum).unwrap();
        let message = serde_json::from_str::<Value>(&message).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn address_error() {
        let expected = serde_json::from_str::<Value>(
            r#"{
                "reason":"InvalidAddressError", 
                "message":"invalid IP address syntax"
              }"#,
        )
        .unwrap();

        let result: Result<IpAddr, _> = "invalid addr".parse();
        let addr_error_enum = Error::AddrParseError(result.unwrap_err());
        let message = serde_json::to_string(&addr_error_enum).unwrap();
        let message = serde_json::from_str::<Value>(&message).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn internal_error() {
        let expected = serde_json::from_str::<Value>(
            r#"{
                "reason":"InternalError", 
                "message":"error"
              }"#,
        )
        .unwrap();

        let internal_error_enum = Error::create_local_error("error");
        let message = serde_json::to_string(&internal_error_enum).unwrap();
        let message = serde_json::from_str::<Value>(&message).unwrap();

        assert_eq!(expected, message);
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (serde_json::to_string(self), serde_json::to_string(other)) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        }
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
