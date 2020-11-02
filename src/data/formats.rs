use serde::{Deserialize, Serialize};

use crate::common::{PhantomId, SerializableId, SocketInfo};
use crate::prelude::{PeerId, Token};

/// Query for POST /data/connections
///
/// It will send as JSON body
///
/// [API](http://35.200.46.204/#/2.data/data_connections_create)
///
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConnectQuery {
    /// to identify which PeerObject calls to neighbour
    pub peer_id: PeerId,
    /// to show that this program has permission to control PeerObject
    pub token: Token,
    /// parameters for DataChannel
    pub options: Option<ConnectionQueryOption>,
    /// connect to the neighbour which has this PeerId
    pub target_id: PeerId,
    /// Shows source Data Object which feeds data redirected to nighbour.
    /// If this field is not set, DataConnection works as RecvOnly.
    pub params: Option<DataIdWrapper>,
    /// Shows destiation socket to which received data is redirected
    /// If this field is not set, DataConnection works as SendOnly.
    pub redirect_params: Option<SocketInfo<PhantomId>>,
}

/// Query parameter for POST /data/connections
///
/// Shows DataConnection parameters in SkyWay layer
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct ConnectionQueryOption {
    /// Metadata associated with the connection, passed in by whoever initiated the connection.
    pub metadata: Option<String>,
    /// Can be BINARY (default), BINARY_UTF8, JSON, or NONE.
    pub serialization: Option<String>,
    /// Detail option for DataConnection
    pub dcInit: Option<DcInit>,
}

/// Query parameter for POST /data/connections
///
/// Shows DataConnection parameters in Browser layer.
/// It's almost same as browser parameters.
///
/// [https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct DcInit {
    /// Indicates whether or not the data channel guarantees in-order delivery of messages; the default is true, which indicates that the data channel is indeed ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "crate::helper::deserialize_maybe_nan")]
    pub ordered: Option<bool>,
    /// The amount of time, in milliseconds, the browser is allowed to take to attempt to transmit a message.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "crate::helper::deserialize_maybe_nan")]
    pub maxPacketLifeTime: Option<usize>,
    /// The maximum number of times the WebRTC Gateway should try to retransmit a message before giving up.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "crate::helper::deserialize_maybe_nan")]
    pub maxRetransmits: Option<usize>,
    /// containing the name of the sub-protocol in use. If no protocol was specified when the data channel was created, then this property's value is "".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "crate::helper::deserialize_maybe_nan")]
    pub protocol: Option<String>,
    /// Indicates whether the RTCDataChannel's connection was negotiated by the Web app (true) or by the WebRTC layer (false).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "crate::helper::deserialize_maybe_nan")]
    pub negotiated: Option<bool>,
    /// ID number (between 0 and 65,534) which uniquely identifies the RTCDataChannel.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "crate::helper::deserialize_maybe_nan")]
    pub id: Option<usize>,
    /// Show priority of this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "crate::helper::deserialize_maybe_nan")]
    pub priority: Option<String>,
}

/// Identifier for source socket of data
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

impl SerializableId for DataId {
    fn try_create(id: Option<String>) -> Option<Self>
        where
            Self: Sized,
    {
        id.map(|id| DataId(id))
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn id(&self) -> String {
        self.0.clone()
    }

    fn key(&self) -> &'static str {
        "data_id"
    }
}

/// Struct just for adapter to adjust JSON format
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DataIdWrapper {
    pub data_id: DataId,
}

/// Identifier for DataConnection
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

/// Response JSON from POST /data/connections
///
/// [API](http://35.200.46.204/#/2.data/data_connections_create)
///
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConnectionResponse {
    /// Fixed value as `"PEERS_CONNECT"`
    pub command_type: String,
    /// Id to identify this DataConnection
    pub params: DataConnectionIdWrapper,
}

/// Struct just for adapter to adjust JSON format
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct DataConnectionIdWrapper {
    pub data_connection_id: DataConnectionId,
}

/// Query for PUT /data/connections
///
/// It will send as JSON body
///
/// [API](http://35.200.46.204/#/2.data/data_connection_put)
///
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RedirectDataParams {
    /// Data source for the DataConnection
    pub feed_params: Option<DataIdWrapper>,
    /// Data destination for the DataConnection. A WebRTC Gateway will redirect received data to this socket.
    pub redirect_params: Option<SocketInfo<PhantomId>>,
}

/// Response from PUT /data/connections
///
/// It will send as JSON body
///
/// [API](http://35.200.46.204/#/2.data/data_connection_put)
///
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RedirectDataResponse {
    /// Fixed value as `"DATA_CONNECTION_PUT"`
    pub command_type: String,
    /// Identify which data will be redirected.
    pub data_id: DataId,
}

/// Response from GET /data/connections/{data_cnnection_id}/status
///
/// [API](http://35.200.46.204/#/2.data/data_connection_status)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DataConnectionStatus {
    /// Identifies a peer connected with this DataConnection
    pub remote_id: String,
    /// Shows size of buffer
    pub buffersize: usize,
    /// The optional label passed in or assigned by SkyWay when the connection was initiated.
    pub label: String,
    /// Any type of metadata associated with the connection, passed in by whoever initiated the connection.
    pub metadata: String,
    /// This is true if the connection is open and ready for read/write.
    pub open: bool,
    /// Whether the underlying data channels are reliable; defined when the connection was initiated.
    pub reliable: bool,
    /// The serialization format of the data sent over the connection. Can be BINARY (default), BINARY_UTF8, JSON, or NONE.
    pub serialization: String,
    /// Fixed value as `"data"`
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "event")]
pub(crate) enum EventEnum {
    OPEN,
    CLOSE,
    ERROR { error_message: String },
    TIMEOUT,
}
