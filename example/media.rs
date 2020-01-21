mod terminal;

use std::collections::hash_set::Iter;
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use either;
use either::Either;
use futures::channel::mpsc;
use futures::future::FutureExt;
use futures::prelude::*;
use futures::*;
use log::{info, warn};
use media::formats::*;
use peer::formats::PeerEventEnum;
use serde_derive::Deserialize;
use std::collections::hash_map::Keys;
use std::net::SocketAddr;
use webrtc_gateway_controller::common::{DataConnectionId, PeerId, PeerInfo};
use webrtc_gateway_controller::*;

// Wrap user input strings with New-Type pattern
#[derive(Debug)]
struct ControlMessage(String);

// This program reacts only with two streams, user control from stdin and peer events.
// This struct holds previous state to process the events properly.
#[derive(Debug)]
struct PeerFoldState(
    (
        Option<PeerInfo>,
        Vec<MediaConfig>,
        Vec<mpsc::Sender<ControlMessage>>,
    ),
);

// PeerFoldState has only setter and getter.
impl PeerFoldState {
    pub fn peer_info(&self) -> &Option<PeerInfo> {
        &(self.0).0
    }

    pub fn set_peer_info(self, peer_info: Option<PeerInfo>) -> Self {
        let PeerFoldState((_, redirects, sender)) = self;
        PeerFoldState((peer_info, redirects, sender))
    }

    pub fn pop_media_config(&mut self) -> Option<MediaConfig> {
        ((self.0).1).pop()
    }

    pub fn control_message_notifier(&mut self) -> &mut Vec<mpsc::Sender<ControlMessage>> {
        &mut (self.0).2
    }

    pub fn set_control_message_notifier(&mut self, tx: mpsc::Sender<ControlMessage>) {
        (&mut (self.0).2).push(tx);
    }
}

// It shows config toml formats
#[derive(Clone, Debug, Deserialize)]
struct Config {
    peer: PeerConfig,
    gateway: SocketConfig,
    media: Vec<MediaConfig>,
}

// It is internal format for config toml
#[derive(Clone, Debug, Deserialize)]
struct PeerConfig {
    pub peer_id: String,
    pub domain: String,
}

// It is internal format for config toml
#[derive(Clone, Debug, Deserialize)]
struct MediaConfig {
    pub video: bool,
    pub audio: bool,
    pub video_redirect: Option<RedirectSocketConfig>,
    pub video_params: Option<MediaParamConfig>,
    pub audio_redirect: Option<RedirectSocketConfig>,
    pub audio_params: Option<MediaParamConfig>,
}

// It is internal format for config toml
#[derive(Clone, Debug, Deserialize)]
struct SocketConfig {
    pub ip: String,
    pub port: u16,
}

// It is internal format for config toml
#[derive(Clone, Debug, Deserialize)]
struct RedirectSocketConfig {
    pub media_ip: String,
    pub media_port: u16,
    pub rtcp_ip: String,
    pub rtcp_port: u16,
}

// It is internal format for config toml
#[derive(Clone, Debug, Deserialize)]
struct MediaParamConfig {
    pub band_width: usize,
    pub codec: String,
    pub payload_type: u16,
    pub sampling_rate: usize,
}

// read config from toml file
fn read_config(path: &'static str) -> Config {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .expect("file open error");

    fr.read_to_string(&mut file_content)
        .expect("file read error");
    toml::from_str(&file_content).expect("toml parse error")
}

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    //load and set parameters
    let config = read_config("example/media.toml");
    let api_key = ::std::env::var("API_KEY").expect("API_KEY is not set in environment variables");
    let domain = config.peer.domain;
    let peer_id = common::PeerId::new(config.peer.peer_id);
    let base_url: String = format!("http://{}:{}", config.gateway.ip, config.gateway.port);
    webrtc_gateway_controller::initialize(base_url);

    //observe keyboard events
    let (key_notifier, key_observer) = mpsc::channel::<String>(0);
    let key_fut = terminal::read(key_notifier);

    //create peer and observe peer events
    let create_peer_future = peer::create(api_key, domain, peer_id, true);
    let peer_info = create_peer_future.await.expect("create peer failed");
    let (peer_event_notifier, peer_event_observer) = mpsc::channel::<PeerEventEnum>(0);
    let event_future = peer::listen_events(&peer_info, peer_event_notifier);

    //this program reacts only to user input and peer events.
    //merge these two event streams, and fold current status
    let peer_event_stream = futures::stream::select(
        peer_event_observer.map(|e| Either::Left(e)),
        key_observer.map(|e| Either::Right(e)),
    );
    let fold_fut = peer_event_stream
        .fold(
            // before receiving Peer::OPEN event, peer object might not be created.
            // So I set None for PeerInfo.
            PeerFoldState((None, config.media, vec![])),
            |sum, acc| async move {
                let sum = on_peer_events(sum, acc).await.expect("error");
                sum
            },
        )
        .map(|_| futures::future::ok::<(), error::ErrorEnum>(()));

    //execute all the futures
    let (fold_fut_result, event_fut_result, key_fut_reult) =
        futures::future::join3(fold_fut, event_future, key_fut).await;
    info!("All the futures are finished. They stopped with these status\nfold: {:?}\nevent: {:?}\nkey:{:?}", fold_fut_result, event_fut_result, key_fut_reult);
}

// This function is called in a fold of User Input and Peer Event streams.
// It parse the stream and process them with its internal functions
async fn on_peer_events(
    status: PeerFoldState,
    event: Either<PeerEventEnum, String>,
) -> Result<PeerFoldState, error::ErrorEnum> {
    match event {
        Either::Left(api_events) => on_peer_api_events(status, api_events).await,
        Either::Right(key_events) => on_peer_key_events(status, key_events).await,
    }
}

// This function process events from Peer Object
async fn on_peer_api_events(
    params: PeerFoldState,
    event: PeerEventEnum,
) -> Result<PeerFoldState, error::ErrorEnum> {
    match event {
        PeerEventEnum::OPEN(event) => {
            // PeerObject notify that it has been successfully created.
            // Hold PeerInfo for further process.
            info!("Peer {:?} is created", event.params);
            let params = params.set_peer_info(Some(event.params));
            Ok(params)
        }
        PeerEventEnum::CLOSE(event) => {
            // PeerObject notify that it has already been deleted.
            // Erase old PeerInfo.
            info!("Peer {:?} is closed", event.params);
            let params = params.set_peer_info(None);
            Ok(params)
        }
        _ => Ok(params),
    }
}

// This function works according to User Keyboard Input
async fn on_peer_key_events(
    mut params: PeerFoldState,
    message: String,
) -> Result<PeerFoldState, error::ErrorEnum> {
    println!("recv {} in peer key events", message);
    match message.as_str() {
        "exit" => {
            // FIXME: not yet
            // When an user wants to close this program, it needs to close P2P links and delete Peer Object.
            // Content Socket will be automaticall released, so it is not necessary to release them manually.
            // https://github.com/skyway/skyway-webrtc-gateway/blob/master/docs/release_process.md
            let mut notifiers = params.control_message_notifier();
            for notifier in notifiers {
                notifier
                    .send(ControlMessage(String::from("disconnect_all")))
                    .await;
            }
            if let Some(peer_info) = params.peer_info() {
                let _unit = peer::delete(&peer_info).await?;
            }
            let params = params.set_peer_info(None);
            Ok(params)
        }
        "status" => {
            // Show status of PeerObject and DataConnection
            if let Some(ref peer_info) = params.peer_info() {
                let status = peer::status(peer_info).await?;
                info!("Peer {:?} is now {:?}", peer_info, status);
            }
            let mut notifiers = params.control_message_notifier();
            for notifier in notifiers {
                notifier.send(ControlMessage(String::from("status"))).await;
            }
            Ok(params)
        }
        message if message.starts_with("disconnect ") => {
            // Disconnect P2P link
            // This function expects "connect DATA_CONNECTION_ID".
            let mut notifiers = params.control_message_notifier();
            for notifier in notifiers {
                notifier.send(ControlMessage(String::from(message))).await;
            }
            Ok(params)
        }
        message if message.starts_with("connect ") => {
            // Establish P2P datachannel to an neighbour.
            // This function expects "connect TARGET_ID".
            let mut args = message.split_whitespace();
            let _ = args.next();
            if let Some(target_id) = args.next() {
                let target_id = PeerId::new(target_id);
                Ok(call(params, target_id).await.expect("error at line 163"))
            } else {
                warn!("input \"connect TARGET_PEER_ID\"");
                Ok(params)
            }
        }
        _ => Ok(params),
    }
}

async fn create_constraints(media_params: &MediaConfig) -> Result<Constraints, error::ErrorEnum> {
    let video_constraints = if media_params.video {
        let video_response = media::open_media_socket(true).await?;
        let rtcp_response = media::open_rtcp_socket().await?;
        Some((video_response, rtcp_response)).map(|(media_response, rtcp_response)| {
            let params = media_params
                .clone()
                .video_params
                .expect("video params should be set");
            MediaParams {
                band_width: params.band_width,
                codec: params.codec,
                media_id: media_response.media_id,
                rtcp_id: Some(rtcp_response.rtcp_id),
                payload_type: Some(params.payload_type),
                sampling_rate: Some(params.sampling_rate),
            }
        })
    } else {
        None
    };

    let audio_constraints = if media_params.audio {
        let audio_response = media::open_media_socket(false).await?;
        let rtcp_response = media::open_rtcp_socket().await?;
        Some((audio_response, rtcp_response)).map(|(media_response, rtcp_response)| {
            let params = media_params
                .clone()
                .audio_params
                .expect("video params should be set");
            MediaParams {
                band_width: params.band_width,
                codec: params.codec,
                media_id: media_response.media_id,
                rtcp_id: Some(rtcp_response.rtcp_id),
                payload_type: Some(params.payload_type),
                sampling_rate: Some(params.sampling_rate),
            }
        })
    } else {
        None
    };

    Ok(Constraints {
        video: media_params.video,
        videoReceiveEnabled: Some(media_params.video_redirect.is_some()),
        audio: media_params.audio,
        audioReceiveEnabled: Some(media_params.audio_redirect.is_some()),
        video_params: video_constraints,
        audio_params: audio_constraints,
    })
}

fn create_redirect(media_params: MediaConfig) -> RedirectParameters {
    let mut redirect_params = RedirectParameters {
        video: None,
        video_rtcp: None,
        audio: None,
        audio_rtcp: None,
    };
    if media_params.video_redirect.is_some() {
        let params = media_params.video_redirect.unwrap();
        redirect_params.video = Some(RedirectParams {
            ip_v4: Some(params.media_ip),
            ip_v6: None,
            port: params.media_port,
        });
        redirect_params.video_rtcp = Some(RedirectParams {
            ip_v4: Some(params.rtcp_ip),
            ip_v6: None,
            port: params.rtcp_port,
        });
    }
    if media_params.audio_redirect.is_some() {
        let params = media_params.audio_redirect.unwrap();
        redirect_params.audio = Some(RedirectParams {
            ip_v4: Some(params.media_ip),
            ip_v6: None,
            port: params.media_port,
        });
        redirect_params.audio_rtcp = Some(RedirectParams {
            ip_v4: Some(params.rtcp_ip),
            ip_v6: None,
            port: params.rtcp_port,
        });
    }
    redirect_params
}

// start establishing MediaConnection to an neighbour
async fn call(
    mut params: PeerFoldState,
    target_id: PeerId,
) -> Result<PeerFoldState, error::ErrorEnum> {
    // Notify which peer object needs to establish P2P link to WebRTC Gateway.
    let peer_info = params
        .peer_info()
        .clone()
        .expect("peer has not been created");

    let media_params = params.pop_media_config();
    if media_params.is_none() {
        return Ok(params);
    }
    let media_config = media_params.unwrap();
    let constraints = create_constraints(&media_config).await?;
    let redirect_params = create_redirect(media_config);

    let call_params = CallParameters {
        peer_id: peer_info.peer_id,
        token: peer_info.token,
        target_id: target_id,
        constraints: Some(constraints),
        redirect_params: Some(redirect_params),
    };

    let call_response = media::call(&call_params).await?;
    Ok(params)
}
