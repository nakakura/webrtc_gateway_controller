mod terminal;

use std::env;
use std::fs;
use std::io::{BufReader, Read};
use std::sync::Mutex;

use futures::channel::mpsc;
use futures::prelude::*;
use futures::prelude::*;
use futures::*;
use log::{info, warn};
use once_cell::sync::OnceCell;
use serde_derive::Deserialize;
use skyway_webrtc_gateway_api::media::*;
use skyway_webrtc_gateway_api::peer::*;
use skyway_webrtc_gateway_api::prelude::*;
use skyway_webrtc_gateway_api::*;

// Config of media used in call and answer.
static CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();

//==================== for parsing media.toml ====================
// It shows config toml formats
#[derive(Clone, Debug, Deserialize)]
struct Config {
    media: Vec<MediaConfig>,
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

//==================== main ====================

#[tokio::main]
async fn main() {
    // set log level
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Initialize values for create peer
    // SkyWay Service API Key
    let api_key = env::var("API_KEY").expect("API_KEY is not set in environment variables");
    // a domain already registered with skyway
    let domain = env::var("DOMAIN").unwrap_or("localhost".into());
    // Identifier of your peer
    let peer_id = env::var("PEER_ID").unwrap_or("peer_id".into());
    let peer_id = PeerId::new(peer_id);
    // URL to access your WebRTC GW.
    let base_url = env::var("BASE_URL").unwrap_or("http://localhost:8000".to_string());

    // read config file
    let media_config = read_config("example/media.toml");
    CONFIG.set(Mutex::new(media_config)).unwrap();

    // initialize crate
    skyway_webrtc_gateway_api::initialize(base_url);

    // call create peer api
    let create_peer_future = peer::create(api_key, domain, peer_id, true);
    // When create api is called, the WebRTC GW first creates a PeerObject internally.
    // Next, it start registering the PeerObject with the SkyWay server.
    // When WebRTC GW starts the sequence, this crate returns Ok(PeerInfo).
    // The PeerInfo contains the Peer ID and the token needed to control the PeerObject.
    let peer_info = create_peer_future.await.expect("create peer failed");
    info!("peer_info is {:?}", peer_info);

    // The WebRTC GW interacts with SkyWay and notifies the end user of the result as an event.
    // Generate a future for event monitoring here.
    let (peer_event_notifier, peer_event_observer) = mpsc::channel::<PeerEventEnum>(10);
    let event_future = peer::listen_events(peer_info.clone(), peer_event_notifier);
    tokio::spawn(event_future);

    // Listen Keyboard Inputs
    let (keyboard_notifier, keyboard_observer) = tokio::sync::mpsc::channel::<String>(10);
    let key_observe_fut = terminal::read_stdin(keyboard_notifier);
    tokio::spawn(key_observe_fut);

    // Routes Keyboard Events
    let key_events_fut = on_keyboard_events(peer_info.clone(), keyboard_observer);

    // Routes Peer Events
    let peer_events_fut = on_peer_events(peer_info, peer_event_observer);

    // run futures
    join!(key_events_fut, peer_events_fut);
}

async fn on_peer_events(peer_info: PeerInfo, mut observer: mpsc::Receiver<PeerEventEnum>) {
    while let Some(result) = observer.next().await {
        match result {
            PeerEventEnum::OPEN(open) => {
                info!(
                    "Peer({}) is created. Now you can CALL/CONNECT.\n{:?}",
                    peer_info.peer_id.as_str(),
                    open
                );
            }
            PeerEventEnum::CALL(call) => {
                info!(
                    "Peer({}) received call as {:?}",
                    peer_info.peer_id.as_str(),
                    call
                );
                let config = &mut *CONFIG.get().unwrap().lock().unwrap();
                let val = config.media.pop().unwrap();
                tokio::spawn(answer(val, call));
            }
            PeerEventEnum::CLOSE(_close) => {
                info!("Peer({}) is deleted", peer_info.peer_id.as_str());
                break;
            }
            _ => {
                info!(
                    "Peer({}) notifies an Event \n{:?}",
                    peer_info.peer_id.as_str(),
                    result
                );
            }
        }
    }
}

async fn on_keyboard_events(
    peer_info: PeerInfo,
    mut observer: tokio::sync::mpsc::Receiver<String>,
) {
    println!(
        r#"print COMMAND
    exit
    status
    call TARGET_ID
    "#
    );
    while let Some(message) = observer.recv().await {
        match message.as_str() {
            "exit" => {
                info!("start closing Peer({})", peer_info.peer_id.as_str());
                // when a PeerObject is closed, MediaConnections associated with it will be automatically closed.
                // So it's not necessary to close MediaConnections here.
                let _ = peer::delete(&peer_info).await;
                break;
            }
            "status" => {
                let peer_status = peer::status(&peer_info).await;
                info!(
                    "Peer({})'s status is \n{:?}",
                    peer_info.peer_id.as_str(),
                    peer_status
                );
            }
            x if x.starts_with("call") => {
                let params: Vec<&str> = x.split(' ').collect();
                if params.len() < 2 {
                    warn!("input \"call TARGET_ID\"");
                    continue;
                }

                let config = &mut *CONFIG.get().unwrap().lock().unwrap();
                let val = config.media.pop().unwrap();
                tokio::spawn(call(peer_info.clone(), val, PeerId(params[1].into())));
            }
            _ => {
                info!("please type valid commands.");
            }
        }

        println!(
            r#"print COMMAND
    exit
    status
    call TARGET_ID
    "#
        );
    }
}

// make a call to neighbour
async fn call(
    peer_info: PeerInfo,
    media_config: MediaConfig,
    target_id: PeerId,
) -> Result<(), error::Error> {
    // Open video and audio port to feed media.
    // Also, create constraints parameter
    let (video_src_socket, audio_src_socket, constraints) =
        create_media_connect_options(&media_config).await?;
    let redirect_params = create_redirect(media_config);

    let call_params = CallQuery {
        peer_id: peer_info.peer_id,
        token: peer_info.token,
        target_id: target_id,
        constraints: Some(constraints),
        redirect_params: Some(redirect_params.clone()),
    };

    // call to neighbour
    let media_connection_id = media::call(&call_params).await?.params.media_connection_id;

    // Listen media connection events
    let (event_notifier, mut event_observer) = mpsc::channel::<media::MediaConnectionEventEnum>(0);
    let event_listen_fut = media::listen_events(media_connection_id.clone(), event_notifier);
    tokio::spawn(event_listen_fut);

    while let Some(media_event) = event_observer.next().await {
        match media_event {
            MediaConnectionEventEnum::READY(ready) => {
                info!(
                    "MediaConnection({}) is ready {:?}",
                    media_connection_id.as_str(),
                    ready
                );
                video_src_socket.clone().map(|sock| {
                    info!("Now you can send video with {:?}", sock);
                });
                audio_src_socket.clone().map(|sock| {
                    info!("Now you can send audio with {:?}", sock);
                });
            }
            MediaConnectionEventEnum::STREAM(stream) => {
                info!(
                    "MediaConnection({}) recv stream {:?}",
                    media_connection_id.as_str(),
                    stream
                );
            }
            MediaConnectionEventEnum::CLOSE(close) => {
                info!(
                    "MediaConnection({}) has closed {:?}",
                    media_connection_id.as_str(),
                    close
                );
            }
            MediaConnectionEventEnum::ERROR(error) => {
                info!(
                    "MediaConnection({}) gets error {:?}",
                    media_connection_id.as_str(),
                    error
                );
            }
        }
    }

    Ok(())
}

// accept a call from neighbour
async fn answer(media_config: MediaConfig, call_event: PeerCallEvent) -> Result<(), error::Error> {
    // Open video and audio port to feed media.
    // Also, create constraints parameter
    let (video_src_socket, audio_src_socket, constraints) =
        create_media_connect_options(&media_config).await?;
    let redirect_params = create_redirect(media_config);
    let answer_params = AnswerQuery {
        constraints: constraints,
        redirect_params: Some(redirect_params.clone()),
    };

    // answer to call from neighbour
    let _answer =
        media::answer(&call_event.call_params.media_connection_id, &answer_params).await?;

    // Listen media connection events
    let (event_notifier, mut event_observer) = mpsc::channel::<media::MediaConnectionEventEnum>(0);
    let event_listen_fut = media::listen_events(
        call_event.call_params.media_connection_id.clone(),
        event_notifier,
    );
    tokio::spawn(event_listen_fut);

    while let Some(media_event) = event_observer.next().await {
        match media_event {
            MediaConnectionEventEnum::READY(ready) => {
                info!(
                    "MediaConnection({}) is ready {:?}",
                    call_event.call_params.media_connection_id.as_str(),
                    ready
                );
                video_src_socket.clone().map(|sock| {
                    info!("Now you can send video with {:?}", sock);
                });
                audio_src_socket.clone().map(|sock| {
                    info!("Now you can send audio with {:?}", sock);
                });
            }
            MediaConnectionEventEnum::STREAM(stream) => {
                info!(
                    "MediaConnection({}) recv stream {:?}",
                    call_event.call_params.media_connection_id.as_str(),
                    stream
                );
            }
            MediaConnectionEventEnum::CLOSE(close) => {
                info!(
                    "MediaConnection({}) has closed {:?}",
                    call_event.call_params.media_connection_id.as_str(),
                    close
                );
            }
            MediaConnectionEventEnum::ERROR(error) => {
                info!(
                    "MediaConnection({}) gets error {:?}",
                    call_event.call_params.media_connection_id.as_str(),
                    error
                );
            }
        }
    }
    Ok(())
}

//==================== create query for call and answer ====================

// Open some sockets to create constraints object,
// and return these information for call and answer
async fn create_media_connect_options(
    media_params: &MediaConfig,
) -> Result<
    (
        Option<(SocketInfo<MediaId>, SocketInfo<RtcpId>)>,
        Option<(SocketInfo<MediaId>, SocketInfo<RtcpId>)>,
        Constraints,
    ),
    error::Error,
> {
    let (video_src_sockets, video_constraints) = if media_params.video
        && media_params.video_params.is_some()
    {
        // open video and video rtcp port
        let video_src_socket = media::open_media_socket(true).await?;
        let rtcp_src_socket = media::open_rtcp_socket().await?;
        //return tuple of socket information and constraints
        (
            Some((video_src_socket.clone(), rtcp_src_socket.clone())),
            Some((video_src_socket, rtcp_src_socket)).map(|(video_src_socket, rtcp_src_socket)| {
                let params = media_params.clone().video_params.unwrap();
                MediaParams {
                    band_width: params.band_width,
                    codec: params.codec,
                    media_id: video_src_socket.get_id().expect("no media_id"),
                    rtcp_id: Some(rtcp_src_socket.get_id().expect("no rtcp_id")),
                    payload_type: Some(params.payload_type),
                    sampling_rate: Some(params.sampling_rate),
                }
            }),
        )
    } else {
        // if video is not needed or config toml is not properly written,
        // these fields are not created
        (None, None)
    };

    let (audio_src_sockets, audio_constraints) = if media_params.audio
        && media_params.audio_params.is_some()
    {
        // open audio and audio rtcp port
        let audio_src_socket = media::open_media_socket(false).await?;
        let rtcp_src_socket = media::open_rtcp_socket().await?;
        //return tuple of socket information and constraints
        (
            Some((audio_src_socket.clone(), rtcp_src_socket.clone())),
            Some((audio_src_socket, rtcp_src_socket)).map(|(audio_src_socket, rtcp_src_socket)| {
                let params = media_params
                    .clone()
                    .audio_params
                    .expect("video params should be set");
                MediaParams {
                    band_width: params.band_width,
                    codec: params.codec,
                    media_id: audio_src_socket.get_id().expect("no media_id"),
                    rtcp_id: Some(rtcp_src_socket.get_id().expect("no id")),
                    payload_type: Some(params.payload_type),
                    sampling_rate: Some(params.sampling_rate),
                }
            }),
        )
    } else {
        // if audio is not needed or config toml is not properly written,
        // these fields are not created
        (None, None)
    };

    Ok((
        video_src_sockets,
        audio_src_sockets,
        Constraints {
            video: media_params.video,
            videoReceiveEnabled: Some(media_params.video_redirect.is_some()),
            audio: media_params.audio,
            audioReceiveEnabled: Some(media_params.audio_redirect.is_some()),
            video_params: video_constraints,
            audio_params: audio_constraints,
            metadata: None,
        },
    ))
}

// create redirect information for call and answer
fn create_redirect(media_params: MediaConfig) -> RedirectParameters {
    let mut redirect_params = RedirectParameters {
        video: None,
        video_rtcp: None,
        audio: None,
        audio_rtcp: None,
    };
    if let Some(ref params) = media_params.video_redirect {
        redirect_params.video = Some(
            SocketInfo::try_create(None, &params.media_ip, params.media_port)
                .expect("invalid video redirect parameter"),
        );
        redirect_params.video_rtcp = Some(
            SocketInfo::try_create(None, &params.rtcp_ip, params.rtcp_port)
                .expect("invalid video_rtcp redirect parameter"),
        );
    }
    if let Some(params) = media_params.audio_redirect {
        redirect_params.audio = Some(
            SocketInfo::try_create(None, &params.media_ip, params.media_port)
                .expect("invalid video redirect parameter"),
        );
        redirect_params.audio_rtcp = Some(
            SocketInfo::try_create(None, &params.rtcp_ip, params.rtcp_port)
                .expect("invalid video_rtcp redirect parameter"),
        );
    }
    redirect_params
}
