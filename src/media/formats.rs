use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreateMediaOptions {
    pub is_video: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreateMediaResponse {
    pub media_id: String,
    pub port: u16,
    pub ip_v4: Option<String>,
    pub ip_v6: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CreateRtcpResponse {
    pub rtcp_id: String,
    pub port: u16,
    pub ip_v4: Option<String>,
    pub ip_v6: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CallParameters {
    pub peer_id: String,
    pub token: String,
    pub target_id: String,
    pub constraints: Option<Constraints>,
    pub redirect_params: Option<RedirectParameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
#[allow(non_snake_case)]
pub struct Constraints {
    pub video: bool,
    videoReceiveEnabled: bool,
    audio: bool,
    audioReceiveEnabled: bool,
    video_params: MediaParams,
    audio_params: MediaParams,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct MediaParams {
    pub band_width: usize,
    pub codec: String,
    pub media_id: String,
    pub rtcp_id: String,
    pub payload_type: u16,
    pub sampling_rate: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct RedirectParams {
    pub ip_v4: Option<String>,
    pub ip_v6: Option<String>,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct RedirectParameters {
    video: Option<RedirectParams>,
    video_rtcp: Option<RedirectParams>,
    audio: Option<RedirectParams>,
    audio_rtcp: Option<RedirectParams>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct CallResponse {
    pub command_type: String,
    pub params: MediaConnectionId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct MediaConnectionId {
    pub media_connection_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct AnswerParameters {
    pub constraints: Option<Constraints>,
    pub redirect_params: Option<RedirectParameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct AnswerResponse {
    pub command_type: String,
    pub params: AnswerResponseParams,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct AnswerResponseParams {
    pub video_port: Option<u16>,
    pub video_id: Option<String>,
    pub audio_port: Option<u16>,
    pub audio_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
#[serde(tag = "event")]
pub enum MediaConnectionEventEnum {
    READY,
    STREAM,
    CLOSE,
    ERROR { error_message: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct MediaConnectionStatus {
    pub metadata: String,
    pub open: bool,
    pub remote_id: String,
    pub ssrc: Vec<SsrcPair>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct SsrcPair {
    pub media_id: String,
    pub ssrc: usize,
}
