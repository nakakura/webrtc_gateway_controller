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
    Error(CreatedResponseError),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreatedResponseSuccess {
    pub command_type: String,
    pub params: PeerInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreatedResponseError {
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
