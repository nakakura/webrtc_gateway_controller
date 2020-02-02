use serde::{Deserialize, Serialize};

use crate::common::{PhantomId, SerializableId, SocketInfo};
use crate::prelude::{PeerId, Token};

/// Identifier for source socket of media
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct MediaId(pub String);

impl MediaId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn new(media_id: impl Into<String>) -> Self {
        MediaId(media_id.into())
    }
}

impl SerializableId for MediaId {
    fn try_create(id: Option<String>) -> Option<Self>
    where
        Self: Sized,
    {
        id.map(|id| MediaId(id))
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn id(&self) -> String {
        self.0.clone()
    }

    fn key(&self) -> &'static str {
        "media_id"
    }
}

/// Identifier for source socket of rtcp
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct RtcpId(pub String);

impl RtcpId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn new(rtcp_id: impl Into<String>) -> Self {
        RtcpId(rtcp_id.into())
    }
}

impl SerializableId for RtcpId {
    fn try_create(id: Option<String>) -> Option<Self>
    where
        Self: Sized,
    {
        id.map(|id| RtcpId(id))
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn id(&self) -> String {
        self.0.clone()
    }

    fn key(&self) -> &'static str {
        "rtcp_id"
    }
}

/// Identifier for MediaConnection
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct MediaConnectionId(pub String);

impl MediaConnectionId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn new(media_connection_id: impl Into<String>) -> Self {
        MediaConnectionId(media_connection_id.into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct CreateMediaOptions {
    pub is_video: bool,
}

/// Query parameter for POST /media/connections
///
/// [API](http://35.200.46.204/#/3.media/media_connection_create)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CallQuery {
    /// to identify which PeerObject calls to neighbour
    pub peer_id: PeerId,
    /// to show that this program has permission to control PeerObject
    pub token: Token,
    /// connect to the neighbour which has this PeerId
    pub target_id: PeerId,
    /// Parameters for MediaConnection
    /// It contains source socket. If the field is None, this MediaConnection works as RecvOnly.
    pub constraints: Option<Constraints>,
    /// Shows destiation socket to which received data is redirected
    /// If this field is not set, DataConnection works as SendOnly.
    pub redirect_params: Option<RedirectParameters>,
}

/// Parameters for MediaConnection
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct Constraints {
    /// Shows whether this connection sends video or not
    pub video: bool,
    /// Shows whether this connection receives video or not
    pub videoReceiveEnabled: Option<bool>,
    /// Shows whether this connection sends audio or not
    pub audio: bool,
    /// Shows whether this connection receives audio or not
    pub audioReceiveEnabled: Option<bool>,
    /// Parameters for sending video
    pub video_params: Option<MediaParams>,
    /// Parameters for sending audio
    pub audio_params: Option<MediaParams>,
}

/// Parameters for sending media
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MediaParams {
    /// band width between Peers
    pub band_width: usize,
    /// Codec which caller side want to use. Video: `"H264"` or `"VP8"`, Audio: `"OPUS"` or `"G711"`. It will be used in SDP.
    pub codec: String,
    /// Identify which media should be redirected
    pub media_id: MediaId,
    /// Identify which rtcp should be redirected
    pub rtcp_id: Option<RtcpId>,
    /// Payload type which caller side want to use. It will be used in SDP.
    pub payload_type: Option<u16>,
    /// Sampling rate which media uses
    pub sampling_rate: Option<usize>,
}

/// Shows to which socket media should be redirected.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RedirectParameters {
    /// video is redirected to this socket
    pub video: Option<SocketInfo<PhantomId>>,
    /// video rtcp is redirected to this socket
    pub video_rtcp: Option<SocketInfo<PhantomId>>,
    /// audio is redirected to this socket
    pub audio: Option<SocketInfo<PhantomId>>,
    /// audio rtcp is redirected to this socket
    pub audio_rtcp: Option<SocketInfo<PhantomId>>,
}

/// Response from POST /media/connections
///
/// [API](http://35.200.46.204/#/3.media/media_connection_create)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CallResponse {
    /// Fixed value as `"PEERS_CALL"`.
    pub command_type: String,
    /// Identifier for MediaConnection
    pub params: MediaConnectionIdWrapper,
}

/// Wrapper for serializing JSON
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct MediaConnectionIdWrapper {
    /// Identifier for MediaConnection
    pub media_connection_id: MediaConnectionId,
}

/// Query parameter for POST /media/connections
///
/// [API](http://35.200.46.204/#/3.media/media_connection_answer)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerQuery {
    /// Parameters for MediaConnection
    /// It contains source socket. If the field is None, this MediaConnection works as RecvOnly.
    pub constraints: Constraints,
    /// Shows destiation socket to which received data is redirected
    /// If this field is not set, DataConnection works as SendOnly.
    pub redirect_params: Option<RedirectParameters>,
}

/// Response from POST /media/connections
///
/// [API](http://35.200.46.204/#/3.media/media_connection_answer)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerResponse {
    /// Fixed value as `"MEDIA_CONNECTION_ANSWER"`.
    pub command_type: String,
    /// Shows media_ids used in this MediaConnection
    pub params: AnswerResponseParams,
}

/// Shows media_ids used in this MediaConnection
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerResponseParams {
    pub video_id: Option<MediaId>,
    pub audio_id: Option<MediaId>,
}

/// Events from GET /media/events API.
/// It includes TIMEOUT, but the event is not needed for end-user-programs.
/// So it's used internally.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "event")]
pub(crate) enum EventEnum {
    READY,
    STREAM,
    CLOSE,
    ERROR { error_message: String },
    TIMEOUT,
}

/// Status of MediaConnection
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MediaConnectionStatus {
    /// Metadata associated with the connection, passed in by whoever initiated the connection.
    pub metadata: String,
    /// Shows whether this MediaConnection is working or not.
    pub open: bool,
    /// Shows neighbour id
    pub remote_id: PeerId,
    /// Shows ssrc(Synchrozination Source) information
    pub ssrc: Vec<SsrcPair>,
}

/// Shows ssrc(Synchrozination Source) information
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SsrcPair {
    /// Identify Media
    pub media_id: MediaId,
    /// SSRC
    pub ssrc: usize,
}
