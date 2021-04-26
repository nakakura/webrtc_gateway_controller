use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::common::formats::{PhantomId, SerializableId, SocketInfo};
use crate::error;
use crate::prelude::{PeerId, Token};

/// Identifier for source socket of media
#[derive(Serialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct MediaId(String);

impl SerializableId for MediaId {
    fn try_create(media_id: impl Into<String>) -> Result<Self, error::Error>
    where
        Self: Sized,
    {
        // peer token's prefix is composed of a UUID and a prefix "pt-".
        let media_id = media_id.into();
        if !(media_id.starts_with("vi-") || media_id.starts_with("au-")) {
            return Err(error::Error::create_local_error(
                "media_id\'s prefix is \"vi-\" or \"au-\"",
            ));
        }
        if media_id.len() != 39 {
            // It's length is 39(UUID: 36 + prefix: 3).
            return Err(error::Error::create_local_error(
                "token str's length should be 39",
            ));
        }
        if !media_id.is_ascii() {
            return Err(error::Error::create_local_error(
                "token str should be ascii",
            ));
        }

        Ok(MediaId(media_id))
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

struct MediaIdVisitor;

impl<'de> Visitor<'de> for MediaIdVisitor {
    type Value = MediaId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 39 length str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let media_id = MediaId::try_create(value);
        if let Err(error::Error::LocalError(err)) = media_id {
            return Err(E::custom(format!("fail to deserialize MediaId: {}", err)));
        } else if let Err(_) = media_id {
            return Err(E::custom(format!("fail to deserialize MediaId")));
        }

        Ok(media_id.unwrap())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let media_id = MediaId::try_create(value);
        if let Err(error::Error::LocalError(err)) = media_id {
            return Err(E::custom(format!("fail to deserialize MediaId: {}", err)));
        } else if let Err(_) = media_id {
            return Err(E::custom(format!("fail to deserialize MediaId")));
        }

        Ok(media_id.unwrap())
    }
}

impl<'de> Deserialize<'de> for MediaId {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(MediaIdVisitor)
    }
}

/// Identifier for source socket of rtcp
#[derive(Serialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct RtcpId(String);

impl SerializableId for RtcpId {
    fn try_create(rtcp_id: impl Into<String>) -> Result<Self, error::Error>
    where
        Self: Sized,
    {
        // peer token's prefix is composed of a UUID and a prefix "pt-".
        let rtcp_id = rtcp_id.into();
        if !rtcp_id.starts_with("rc-") {
            return Err(error::Error::create_local_error(
                "rtcp_id\'s prefix is \"rc-\"",
            ));
        }
        if rtcp_id.len() != 39 {
            // It's length is 39(UUID: 36 + prefix: 3).
            return Err(error::Error::create_local_error(
                "token str's length should be 39",
            ));
        }
        if !rtcp_id.is_ascii() {
            return Err(error::Error::create_local_error(
                "token str should be ascii",
            ));
        }

        Ok(RtcpId(rtcp_id))
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

struct RtcpIdVisitor;

impl<'de> Visitor<'de> for RtcpIdVisitor {
    type Value = RtcpId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 39 length str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let media_id = RtcpId::try_create(value);
        if let Err(error::Error::LocalError(err)) = media_id {
            return Err(E::custom(format!("fail to deserialize RtcpId: {}", err)));
        } else if let Err(_) = media_id {
            return Err(E::custom(format!("fail to deserialize RtcpId")));
        }

        Ok(media_id.unwrap())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let media_id = RtcpId::try_create(value);
        if let Err(error::Error::LocalError(err)) = media_id {
            return Err(E::custom(format!("fail to deserialize RtcpId: {}", err)));
        } else if let Err(_) = media_id {
            return Err(E::custom(format!("fail to deserialize RtcpId")));
        }

        Ok(media_id.unwrap())
    }
}

impl<'de> Deserialize<'de> for RtcpId {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(RtcpIdVisitor)
    }
}

/// Identifier for MediaConnection
#[derive(Serialize, Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct MediaConnectionId(String);

impl MediaConnectionId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn try_create(media_connection_id: impl Into<String>) -> Result<Self, error::Error>
    where
        Self: Sized,
    {
        // peer token's prefix is composed of a UUID and a prefix "pt-".
        let media_connection_id = media_connection_id.into();
        if !media_connection_id.starts_with("mc-") {
            return Err(error::Error::create_local_error(
                "media_connection_id\'s prefix is \"rc-\"",
            ));
        }
        if media_connection_id.len() != 39 {
            // It's length is 39(UUID: 36 + prefix: 3).
            return Err(error::Error::create_local_error(
                "token str's length should be 39",
            ));
        }
        if !media_connection_id.is_ascii() {
            return Err(error::Error::create_local_error(
                "token str should be ascii",
            ));
        }

        Ok(MediaConnectionId(media_connection_id))
    }
}

struct MediaConnectionIdVisitor;

impl<'de> Visitor<'de> for MediaConnectionIdVisitor {
    type Value = MediaConnectionId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 39 length str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let media_connection_id = MediaConnectionId::try_create(value);
        if let Err(error::Error::LocalError(err)) = media_connection_id {
            return Err(E::custom(format!("fail to deserialize MediaId: {}", err)));
        } else if let Err(_) = media_connection_id {
            return Err(E::custom(format!("fail to deserialize MediaId")));
        }

        Ok(media_connection_id.unwrap())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let media_connection_id = MediaConnectionId::try_create(value);
        if let Err(error::Error::LocalError(err)) = media_connection_id {
            return Err(E::custom(format!("fail to deserialize MediaId: {}", err)));
        } else if let Err(_) = media_connection_id {
            return Err(E::custom(format!("fail to deserialize MediaId")));
        }

        Ok(media_connection_id.unwrap())
    }
}

impl<'de> Deserialize<'de> for MediaConnectionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(MediaConnectionIdVisitor)
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
    /// metadata sent to a neighbour.
    pub metadata: Option<String>,
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
    pub ssrc: Option<Vec<SsrcPair>>,
}

/// Shows ssrc(Synchrozination Source) information
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SsrcPair {
    /// Identify Media
    pub media_id: MediaId,
    /// SSRC
    pub ssrc: usize,
}
