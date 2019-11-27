use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerOptions {
    pub key: String,
    pub domain: String,
    pub peer_id: String,
    pub turn: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerInfo {
    pub peer_id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
#[serde(untagged)]
pub enum CreatedResponse {
    Success(CreatedResponseSuccess),
    Error(PeerErrorResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreatedResponseSuccess {
    pub command_type: String,
    pub params: PeerInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerErrorResponse {
    pub command_type: String,
    pub params: PeerErrors,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerErrors {
    pub errors: Vec<PeerError>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerError {
    pub field: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
#[serde(tag = "event")]
pub enum PeerEventEnum {
    OPEN(PeerOpenEvent),
    CLOSE(PeerCloseEvent),
    CONNECTION(PeerConnectionEvent),
    CALL(PeerCallEvent),
    ERROR(PeerErrorEvent),
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
    pub data_connection_id: String,
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
