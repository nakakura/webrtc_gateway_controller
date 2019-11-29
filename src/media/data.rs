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

use crate::data::data::RedirectParams;

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
