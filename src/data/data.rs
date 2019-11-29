use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreatedResponse {
    pub data_id: String,
    pub port: u16,
    pub ip_v4: Option<String>,
    pub ip_v6: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreateDataConnectionQuery {
    pub peer_id: String,
    pub token: String,
    pub options: Option<DataConnectionParameters>,
    pub target_id: String,
    pub params: DataId,
    pub redirect_params: Option<RedirectParams>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
#[allow(non_snake_case)]
pub struct DataConnectionParameters {
    pub metadata: String,
    pub serialization: String,
    pub dcInit: DcInit,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
#[allow(non_snake_case)]
pub struct DcInit {
    pub ordered: bool,
    pub maxPacketLifeTime: usize,
    pub maxRetransmits: usize,
    pub protocol: String,
    pub negotiated: bool,
    pub id: usize,
    pub priority: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct DataId {
    pub data_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreateDataConnectionResponse {
    pub command_type: String,
    pub params: DataConnectionId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct DataConnectionId {
    pub data_connection_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct RedirectParams {
    pub ip_v4: Option<String>,
    pub ip_v6: Option<String>,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct RedirectDataParams {
    pub feed_params: DataId,
    pub redirect_params: RedirectParams,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct RedirectDataResponse {
    pub command_type: String,
    pub data_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct DataConnectionStatus {
    pub remote_id: String,
    pub buffersize: usize,
    pub label: String,
    pub metadata: String,
    pub open: bool,
    pub reliable: bool,
    pub serialization: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
#[serde(tag = "event")]
pub enum DataConnectionEventEnum {
    OPEN,
    CLOSE,
    ERROR { error_message: String },
    TIMEOUT,
}
