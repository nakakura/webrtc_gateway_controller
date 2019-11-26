use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerInfo {
    pub peer_id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreatedResponse {
    pub command_type: String,
    pub params: PeerInfo,
}
