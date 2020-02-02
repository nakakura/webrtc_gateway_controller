use serde::{Deserialize, Serialize};

use crate::data::DataConnectionIdWrapper;
use crate::prelude::{DataConnectionId, MediaConnectionId};

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

/// Token to avoid misuse of Peer.
///
/// It is used with PeerId.
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

/// Pair of PeerId and Token.
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub token: Token,
}

/// Query for POST /peers.
///
/// See [API](http://35.200.46.204/#/1.peers/peer)
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
/// See [API](http://35.200.46.204/#/1.peers/peer)
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

/// Indicates peer object is registered to SkyWay Server
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerOpenEvent {
    /// Pair of PeerId and Token. PeerId has been allocated in the server.
    pub params: PeerInfo,
}

/// Indicates peer object is deleted
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerCloseEvent {
    /// Pair of PeerId and Token. Just for indicating which Peer Object is deleted.
    pub params: PeerInfo,
}

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
/// DataConnection is automatically established when the request comes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerConnectionEvent {
    /// Pair of PeerId and Token. Indicate which Peer Object is regarded.
    pub params: PeerInfo,
    /// Id to identify the DataConnection
    pub data_params: DataConnectionIdWrapper,
}

/// Shows that the Peer Object receives a request to establish MediaConnection with neighbour.
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerCallEvent {
    pub params: PeerInfo,
    pub call_params: PeerCallEventMediaParams,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerCallEventMediaParams {
    pub media_connection_id: MediaConnectionId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerStatusMessage {
    pub peer_id: PeerId,
    pub disconnected: bool,
}
