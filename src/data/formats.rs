use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::common::formats::{PhantomId, SerializableId, SocketInfo};
use crate::error;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ConnectQueryOption>,
    /// connect to the neighbour which has this PeerId
    pub target_id: PeerId,
    /// Shows source Data Object which feeds data redirected to nighbour.
    /// If this field is not set, DataConnection works as RecvOnly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<DataIdWrapper>,
    /// Shows destiation socket to which received data is redirected
    /// If this field is not set, DataConnection works as SendOnly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_params: Option<SocketInfo<PhantomId>>,
}

/// Query parameter for POST /data/connections
///
/// Shows DataConnection parameters in SkyWay layer
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct ConnectQueryOption {
    /// Metadata associated with the connection, passed in by whoever initiated the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// Can be BINARY (default), BINARY_UTF8, JSON, or NONE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization: Option<String>,
    /// Detail option for DataConnection
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Serialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct DataId(String);

impl SerializableId for DataId {
    fn try_create(data_id: impl Into<String>) -> Result<Self, error::Error>
    where
        Self: Sized,
    {
        // peer token's prefix is composed of a UUID and a prefix "pt-".
        let data_id = data_id.into();
        if !data_id.starts_with("da-") {
            return Err(error::Error::create_local_error(
                "token str\'s prefix is \"da-\"",
            ));
        }
        if data_id.len() != 39 {
            // It's length is 39(UUID: 36 + prefix: 3).
            return Err(error::Error::create_local_error(
                "token str's length should be 39",
            ));
        }
        if !data_id.is_ascii() {
            return Err(error::Error::create_local_error(
                "token str should be ascii",
            ));
        }

        Ok(DataId(data_id))
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

struct DataVisitor;

impl<'de> Visitor<'de> for DataVisitor {
    type Value = DataId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 39 length str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let data_id = DataId::try_create(value);
        if let Err(error::Error::LocalError(err)) = data_id {
            return Err(E::custom(format!("fail to deserialize DataId: {}", err)));
        } else if let Err(_) = data_id {
            return Err(E::custom(format!("fail to deserialize DataId")));
        }

        Ok(data_id.unwrap())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let data_id = DataId::try_create(value);
        if let Err(error::Error::LocalError(err)) = data_id {
            return Err(E::custom(format!("fail to deserialize Token: {}", err)));
        } else if let Err(_) = data_id {
            return Err(E::custom(format!("fail to deserialize Token")));
        }

        Ok(data_id.unwrap())
    }
}

impl<'de> Deserialize<'de> for DataId {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(DataVisitor)
    }
}

/// Struct just for adapter to adjust JSON format
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DataIdWrapper {
    pub data_id: DataId,
}

/// Identifier for DataConnection
#[derive(Serialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct DataConnectionId(String);

impl DataConnectionId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn try_create(data_connection_id: impl Into<String>) -> Result<Self, error::Error>
    where
        Self: Sized,
    {
        // peer token's prefix is composed of a UUID and a prefix "pt-".
        let data_connection_id = data_connection_id.into();
        if !data_connection_id.starts_with("dc-") {
            return Err(error::Error::create_local_error(
                "data_connection_id\'s prefix is \"dc-\"",
            ));
        }
        if data_connection_id.len() != 39 {
            // It's length is 39(UUID: 36 + prefix: 3).
            return Err(error::Error::create_local_error(
                "token str's length should be 39",
            ));
        }
        if !data_connection_id.is_ascii() {
            return Err(error::Error::create_local_error(
                "token str should be ascii",
            ));
        }

        Ok(DataConnectionId(data_connection_id))
    }
}

struct DataConnectionIdVisitor;

impl<'de> Visitor<'de> for DataConnectionIdVisitor {
    type Value = DataConnectionId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 39 length str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let data_connection_id = DataConnectionId::try_create(value);
        if let Err(error::Error::LocalError(err)) = data_connection_id {
            return Err(E::custom(format!("fail to deserialize DataId: {}", err)));
        } else if let Err(_) = data_connection_id {
            return Err(E::custom(format!("fail to deserialize DataId")));
        }

        Ok(data_connection_id.unwrap())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let data_connection_id = DataConnectionId::try_create(value);
        if let Err(error::Error::LocalError(err)) = data_connection_id {
            return Err(E::custom(format!("fail to deserialize Token: {}", err)));
        } else if let Err(_) = data_connection_id {
            return Err(E::custom(format!("fail to deserialize Token")));
        }

        Ok(data_connection_id.unwrap())
    }
}

impl<'de> Deserialize<'de> for DataConnectionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(DataConnectionIdVisitor)
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_params: Option<DataIdWrapper>,
    /// Data destination for the DataConnection. A WebRTC Gateway will redirect received data to this socket.
    #[serde(skip_serializing_if = "Option::is_none")]
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
