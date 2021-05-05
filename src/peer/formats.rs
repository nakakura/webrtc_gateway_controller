use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::data::DataConnectionIdWrapper;
use crate::error;
use crate::media::MediaConnectionIdWrapper;

/// Identifier for PeerObject.
///
/// To avoid misuse, it is used with Token.
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct PeerId(pub String);

impl PeerId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn new(peer_id: impl Into<String>) -> Self {
        PeerId(peer_id.into())
    }
}

/// Token to avoid mesuse of Peer.
///
/// It is used with PeerId.
#[derive(Serialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Token(String);

impl Token {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn try_create(token: impl Into<String>) -> Result<Self, error::Error> {
        // peer token's prefix is composed of a UUID and a prefix "pt-".
        let token_string = token.into();
        if !token_string.starts_with("pt-") {
            return Err(error::Error::create_local_error(
                "token str\'s prefix is \"pt-\"",
            ));
        }
        if token_string.len() != 39 {
            // It's length is 39(UUID: 36 + prefix: 3).
            return Err(error::Error::create_local_error(
                "token str's length should be 39",
            ));
        }
        if !token_string.is_ascii() {
            return Err(error::Error::create_local_error(
                "token str should be ascii",
            ));
        }

        Ok(Token(token_string))
    }
}

#[test]
fn create_token_success() {
    let token = Token::try_create("pt-9749250e-d157-4f80-9ee2-359ce8524308").unwrap();
    assert_eq!(token.as_str(), "pt-9749250e-d157-4f80-9ee2-359ce8524308");
}

#[test]
fn create_token_not_start_with_pt() {
    // peer token's prefix is "pt-"
    let token = Token::try_create("vi-9749250e-d157-4f80-9ee2-359ce8524308");
    if let Err(error::Error::LocalError(err)) = token {
        assert_eq!(err.as_str(), "token str\'s prefix is \"pt-\"");
    } else {
        unreachable!();
    }
}

#[test]
fn create_token_not_sufficient_length() {
    // this test is executed with 38 chars
    let token = Token::try_create("pt-9749250e-d157-4f80-9ee2-359ce852430");
    if let Err(error::Error::LocalError(err)) = token {
        assert_eq!(err.as_str(), "token str\'s length should be 39");
    } else {
        unreachable!();
    }
}

#[test]
fn create_token_too_long() {
    // this test is executed with 40 chars
    let token = Token::try_create("pt-9749250e-d157-4f80-9ee2-359ce85243080");
    if let Err(error::Error::LocalError(err)) = token {
        assert_eq!(err.as_str(), "token str\'s length should be 39");
    } else {
        unreachable!();
    }
}

#[test]
fn create_token_not_ascii_str() {
    let token = Token::try_create("pt-9749250e-d157-4f80-9ee2-359ce8524あ");
    if let Err(error::Error::LocalError(err)) = token {
        assert_eq!(err.as_str(), "token str should be ascii");
    } else {
        unreachable!();
    }
}

struct TokenVisitor;

impl<'de> Visitor<'de> for TokenVisitor {
    type Value = Token;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 39 length str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let token = Token::try_create(value);
        if let Err(error::Error::LocalError(err)) = token {
            return Err(E::custom(format!("fail to deserialize Token: {}", err)));
        } else if let Err(_) = token {
            return Err(E::custom(format!("fail to deserialize Token")));
        }

        Ok(token.unwrap())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let token = Token::try_create(value);
        if let Err(error::Error::LocalError(err)) = token {
            return Err(E::custom(format!("fail to deserialize Token: {}", err)));
        } else if let Err(_) = token {
            return Err(E::custom(format!("fail to deserialize Token")));
        }

        Ok(token.unwrap())
    }
}

impl<'de> Deserialize<'de> for Token {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(TokenVisitor)
    }
}

#[cfg(test)]
mod deserialize_token {
    use super::*;

    #[derive(Deserialize)]
    struct TokenWrapper {
        pub token: Token,
    }

    #[test]
    fn deserialize_ok() {
        // this test is executed with 38 chars
        let wrapper = serde_json::from_str::<TokenWrapper>(
            r#"{"token": "pt-9749250e-d157-4f80-9ee2-359ce8524308"}"#,
        )
        .unwrap();
        assert_eq!(
            wrapper.token.as_str(),
            "pt-9749250e-d157-4f80-9ee2-359ce8524308"
        );
    }

    #[test]
    fn deserialize_err() {
        // this test is executed with 38 chars
        let result = serde_json::from_str::<TokenWrapper>(
            r#"{"token": "pt-9749250e-d157-4f80-9ee2-359ce852430"}"#,
        );
        assert!(result.is_err());
    }
}

/// Pair of PeerId and Token.
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct PeerInfo {
    peer_id: PeerId,
    token: Token,
}

impl PeerInfo {
    pub fn new(peer_id: PeerId, token: Token) -> Self {
        Self { peer_id, token }
    }

    pub fn try_create(
        peer_id: impl Into<String>,
        token: impl Into<String>,
    ) -> Result<Self, error::Error> {
        Ok(PeerInfo {
            peer_id: PeerId::new(peer_id),
            token: Token::try_create(token)?,
        })
    }

    pub fn peer_id(&self) -> PeerId {
        return self.peer_id.clone();
    }

    pub fn token(&self) -> Token {
        return self.token.clone();
    }
}

/// Query for POST /peers.
///
/// [API](http://35.200.46.204/#/1.peers/peer)
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreatePeerQuery {
    /// SkyWay's API Key.
    pub key: String,
    /// Registered domain of the API Key
    pub domain: String,
    /// Peer Id that user want to use.
    pub peer_id: PeerId,
    /// Whether does user want to use TURN server or not.
    pub turn: bool,
}

/// Response from POST /peers
///
/// [API](http://35.200.46.204/#/1.peers/peer)
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreatedResponse {
    /// Fixed value as `"PEERS_CREATE"`.
    pub command_type: String,
    /// Pair of PeerId and Token. PeerId is not allocated in the server in this timing.
    pub params: PeerInfo,
}

/// Events from GET /peer/events
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
#[serde(tag = "event")]
pub(crate) enum EventEnum {
    OPEN(PeerOpenEvent),
    CLOSE(PeerCloseEvent),
    CONNECTION(PeerConnectionEvent),
    CALL(PeerCallEvent),
    ERROR(PeerErrorEvent),
    TIMEOUT,
}

/// Response from GET /peers/{peer_id}/events
///
/// [API](http://35.200.46.204/#/1.peers/peer_event)
///
/// Indicates peer object is registered to SkyWay Server
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerOpenEvent {
    /// Pair of PeerId and Token. PeerId has been allocated in the server.
    pub params: PeerInfo,
}

/// Response from GET /peers/{peer_id}/events
///
/// [API](http://35.200.46.204/#/1.peers/peer_event)
///
/// Indicates peer object is deleted
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerCloseEvent {
    /// Pair of PeerId and Token. Just for indicating which Peer Object is deleted.
    pub params: PeerInfo,
}

/// Response from GET /peers/{peer_id}/events
///
/// [API](http://35.200.46.204/#/1.peers/peer_event)
///
/// Shows Error about PeerObject
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerErrorEvent {
    /// Pair of PeerId and Token. Indicate which Peer Object is regarded.
    pub params: PeerInfo,
    /// Shows detail of the error.
    pub error_message: String,
}

/// Shows that the Peer Object receives a request to establish DataConnection with neighbour.
///
/// [API](http://35.200.46.204/#/1.peers/peer_event)
///
/// DataConnection is automatically established when the request comes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerConnectionEvent {
    /// Pair of PeerId and Token. Indicate which Peer Object is regarded.
    pub params: PeerInfo,
    /// Id to identify the DataConnection
    pub data_params: DataConnectionIdWrapper,
}

/// Shows that the Peer Object receives a request to establish MediaConnection with neighbour.
///
/// [API](http://35.200.46.204/#/1.peers/peer_event)
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerCallEvent {
    pub params: PeerInfo,
    pub call_params: MediaConnectionIdWrapper,
}

/// Response from GET /peer/{peer_id}/status
///
/// [API](http://35.200.46.204/#/1.peers/peer_status)
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerStatusMessage {
    pub peer_id: PeerId,
    pub disconnected: bool,
}
