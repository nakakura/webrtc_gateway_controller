use crate::{PeerId, SocketInfo, Token};
use serde::{Deserialize, Serialize};

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
pub struct CreateMediaOptions {
    pub is_video: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateMediaResponse {
    pub media_id: MediaId,
    pub port: u16,
    pub ip_v4: Option<String>,
    pub ip_v6: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateRtcpResponse {
    pub rtcp_id: RtcpId,
    pub port: u16,
    pub ip_v4: Option<String>,
    pub ip_v6: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CallParameters {
    pub peer_id: PeerId,
    pub token: Token,
    pub target_id: PeerId,
    pub constraints: Option<Constraints>,
    pub redirect_params: Option<RedirectParameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct Constraints {
    pub video: bool,
    pub videoReceiveEnabled: Option<bool>,
    pub audio: bool,
    pub audioReceiveEnabled: Option<bool>,
    pub video_params: Option<MediaParams>,
    pub audio_params: Option<MediaParams>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MediaParams {
    pub band_width: usize,
    pub codec: String,
    pub media_id: MediaId,
    pub rtcp_id: Option<RtcpId>,
    pub payload_type: Option<u16>,
    pub sampling_rate: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RedirectParameters {
    pub video: Option<SocketInfo>,
    pub video_rtcp: Option<SocketInfo>,
    pub audio: Option<SocketInfo>,
    pub audio_rtcp: Option<SocketInfo>,
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
    pub video_port: Option<u16>,
    pub video_id: Option<MediaId>,
    pub audio_port: Option<u16>,
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
