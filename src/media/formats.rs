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
    fn new(id: Option<String>) -> Option<Self>
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
    fn new(id: Option<String>) -> Option<Self>
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

/// Parameter for call
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CallParameters {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CallResponse {
    pub command_type: String,
    pub params: MediaConnectionIdWrapper,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MediaConnectionIdWrapper {
    pub media_connection_id: MediaConnectionId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerParameters {
    pub constraints: Constraints,
    pub redirect_params: Option<RedirectParameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerResponse {
    pub command_type: String,
    pub params: AnswerResponseParams,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerResponseParams {
    pub video_id: Option<MediaId>,
    pub audio_id: Option<MediaId>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "event")]
pub enum MediaConnectionEventEnum {
    READY,
    STREAM,
    CLOSE,
    ERROR { error_message: String },
    TIMEOUT,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MediaConnectionStatus {
    pub metadata: String,
    pub open: bool,
    pub remote_id: PeerId,
    pub ssrc: Vec<SsrcPair>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SsrcPair {
    pub media_id: MediaId,
    pub ssrc: usize,
}
