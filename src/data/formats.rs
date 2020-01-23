use serde::{Deserialize, Serialize};

use crate::common::{PhantomId, SocketInfo};
use crate::{PeerId, Token};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreatedResponse {
    pub data_id: DataId,
    pub port: u16,
    pub ip_v4: Option<String>,
    pub ip_v6: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateDataConnectionQuery {
    pub peer_id: PeerId,
    pub token: Token,
    pub options: Option<DataConnectionParameters>,
    pub target_id: PeerId,
    pub params: Option<DataIdWrapper>,
    pub redirect_params: Option<SocketInfo<PhantomId>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct DataConnectionParameters {
    pub metadata: String,
    pub serialization: String,
    pub dcInit: DcInit,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct DataId(pub String);

impl DataId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn new(data_id: impl Into<String>) -> Self {
        DataId(data_id.into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DataIdWrapper {
    pub data_id: DataId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct DataConnectionId(pub String);

impl DataConnectionId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn new(data_connection_id: impl Into<String>) -> Self {
        DataConnectionId(data_connection_id.into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateDataConnectionResponse {
    pub command_type: String,
    pub params: DataConnectionIdWrapper,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DataConnectionIdWrapper {
    pub data_connection_id: DataConnectionId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RedirectDataParams {
    pub feed_params: Option<DataIdWrapper>,
    pub redirect_params: Option<SocketInfo<PhantomId>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RedirectDataResponse {
    pub command_type: String,
    pub data_id: DataId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "event")]
pub enum DataConnectionEventEnum {
    OPEN,
    CLOSE,
    ERROR { error_message: String },
    TIMEOUT,
}

#[derive(Debug, PartialEq)]
pub struct OnOpenTxParameters(pub DataConnectionId);
#[derive(Debug, PartialEq)]
pub struct OnCloseTxParameters(pub DataConnectionId);
#[derive(Debug, PartialEq)]
pub struct OnErrorTxParameters(pub DataConnectionId, pub String);
