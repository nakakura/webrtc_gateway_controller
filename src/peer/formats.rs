use serde::{Deserialize, Serialize};

use crate::DataConnectionId;

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Token(pub String);

impl Token {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn new(token: impl Into<String>) -> Self {
        Token(token.into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub token: Token,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerOptions {
    pub key: String,
    pub domain: String,
    pub peer_id: PeerId,
    pub turn: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreatedResponse {
    pub command_type: String,
    pub params: PeerInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
#[serde(tag = "event")]
pub enum PeerEventEnum {
    OPEN(PeerOpenEvent),
    CLOSE(PeerCloseEvent),
    CONNECTION(PeerConnectionEvent),
    CALL(PeerCallEvent),
    ERROR(PeerErrorEvent),
    TIMEOUT,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerOpenEvent {
    pub params: PeerInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerCloseEvent {
    pub params: PeerInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerErrorEvent {
    pub params: PeerInfo,
    pub error_message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerConnectionEvent {
    pub params: PeerInfo,
    pub data_params: PeerConnectionEventDataParams,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerConnectionEventDataParams {
    pub data_connection_id: DataConnectionId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerCallEvent {
    pub params: PeerInfo,
    pub call_params: PeerCallEventMediaParams,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerCallEventMediaParams {
    pub media_connection_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerStatusMessage {
    pub peer_id: PeerId,
    pub disconnected: bool,
}
