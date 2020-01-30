mod terminal;

use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use either;
use either::Either;
use futures::channel::mpsc;
use futures::future::FutureExt;
use futures::prelude::*;
use log::{error, info, warn};
use serde_derive::Deserialize;

use media::*;
use peer::PeerEventEnum;
use webrtc_gateway_controller::peer::PeerCallEvent;
use webrtc_gateway_controller::prelude::*;
use webrtc_gateway_controller::*;

//==================== for parsing media.toml ====================
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

//==================== main ====================

// Main function creates a peer object and start listening peer events and keyboard events.
// Further processes are triggered by these events.
#[tokio::main]
async fn main() {
    // initialize logger
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
        info!("RUST_LOG is not set. So it works as info mode.")
    }
    env_logger::init();

    //load and set parameters
    let api_key = ::std::env::var("API_KEY").expect("API_KEY is not set in environment variables");
    let config = read_config("example/media.toml");
    let domain = config.peer.domain;
    let peer_id = PeerId::new(config.peer.peer_id);
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
            // So None is set for Option<PeerInfo>.
            PeerFoldState((None, config.media, vec![])),
            |state, event| async move {
                match event {
                    Either::Left(api_events) => on_peer_api_events(state, api_events)
                        .await
                        .expect("error in on_peer_api_events"),
                    Either::Right(key_events) => on_peer_key_events(state, key_events)
                        .await
                        .expect("error in on_peer_key_events"),
                }
            },
        )
        .map(|_| futures::future::ok::<(), error::Error>(()));

    //execute all the futures
    let (fold_fut_result, event_fut_result, key_fut_reult) =
        futures::future::join3(fold_fut, event_future, key_fut).await;
    info!("All the futures are finished. They stopped with these status\nfold: {:?}\nevent: {:?}\nkey:{:?}", fold_fut_result, event_fut_result, key_fut_reult);
}

//==================== materials for PeerFold ====================

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

#[derive(Clone, Debug, Deserialize)]
struct MediaSrcSockets {
    media_socket: SocketInfo<MediaId>,
    rtcp_socket: SocketInfo<RtcpId>,
}

//==================== process events for peer ====================

// This function process events from Peer Object
async fn on_peer_api_events(
    status: PeerFoldState,
    event: PeerEventEnum,
) -> Result<PeerFoldState, error::Error> {
    // CONNECTION is not needed for this program.
    let status = match event {
        PeerEventEnum::OPEN(event) => {
            // PeerObject notify that it has been successfully created.
            // Hold PeerInfo for further process.
            info!(
                "====================\nPeerObject is created. Its PeerId is {} and Token is {}",
                event.params.peer_id.as_str(),
                event.params.token.as_str()
            );
            let params = status.set_peer_info(Some(event.params));
            Ok(params)
        }
        PeerEventEnum::CLOSE(event) => {
            // PeerObject notify that it has already been deleted.
            // Erase old PeerInfo.
            info!(
                "====================\nPeerObject of {} is closed",
                event.params.peer_id.as_str()
            );
            let params = status.set_peer_info(None);
            Ok(params)
        }
        PeerEventEnum::CALL(event) => {
            info!(
                "====================\nreceive call request. Its MediaConnectionId is {}",
                event.call_params.media_connection_id.as_str()
            );
            let status = answer(status, event).await?;
            Ok(status)
        }
        PeerEventEnum::ERROR(event) => {
            error!("error {:?} occurs in on_peer_api_events", event);
            Ok(status)
        }
        _ => Ok(status),
    };
    print_commands();
    status
}

// This function works according to User Keyboard Input
async fn on_peer_key_events(
    mut status: PeerFoldState,
    message: String,
) -> Result<PeerFoldState, error::Error> {
    let status = match message.as_str() {
        "exit" => {
            // When an user wants to close this program, it needs to close P2P links and delete Peer Object.
            // Content Socket will be automaticall released, so it is not necessary to release them manually.
            // https://github.com/skyway/skyway-webrtc-gateway/blob/master/docs/release_process.md
            let notifiers = status.control_message_notifier();
            for notifier in notifiers {
                let _ = notifier
                    .send(ControlMessage(String::from("disconnect_all")))
                    .await;
            }
            if let Some(peer_info) = status.peer_info() {
                let _unit = peer::delete(&peer_info).await?;
            }
            let params = status.set_peer_info(None);
            Ok(params)
        }
        "status" => {
            // Show status of PeerObject and MediaConnection
            if let Some(ref peer_info) = status.peer_info() {
                let status = peer::status(peer_info).await?;
                let mut message = String::from("====================\nShow Status");
                message = format!(
                    "{}\nMy PeerId is {} and Token is {}. It is {} now.",
                    message,
                    peer_info.peer_id.as_str(),
                    peer_info.token.as_str(),
                    if !status.disconnected {
                        "connected to Server."
                    } else {
                        "not connected to Server."
                    }
                );
                info!("{}", message);
            }
            let notifiers = status.control_message_notifier();
            for notifier in notifiers {
                let _ = notifier.send(ControlMessage(String::from("status"))).await;
            }
            Ok(status)
        }
        message if message.starts_with("pli ") => {
            // Send a Pli Packet
            // This function expects "pli MEDIA_CONNECTION_ID".
            let notifiers = status.control_message_notifier();
            for notifier in notifiers {
                let _ = notifier.send(ControlMessage(String::from(message))).await;
            }
            Ok(status)
        }
        message if message.starts_with("disconnect ") => {
            // Disconnect P2P link
            // This function expects "connect MEDIA_CONNECTION_ID".
            let notifiers = status.control_message_notifier();
            for notifier in notifiers {
                let _ = notifier.send(ControlMessage(String::from(message))).await;
            }
            Ok(status)
        }
        message if message.starts_with("call ") => {
            // Establish P2P datachannel to an neighbour.
            // This function expects "connect TARGET_ID".
            let mut args = message.split_whitespace();
            let _ = args.next();
            if let Some(target_id) = args.next() {
                let target_id = PeerId::new(target_id);
                Ok(call(status, target_id).await.expect("error at line 163"))
            } else {
                warn!("input \"call TARGET_PEER_ID\"");
                Ok(status)
            }
        }
        _ => Ok(status),
    };
    print_commands();
    status
}

//==================== call and answer to establish media connection ====================
// start establishing MediaConnection to an neighbour
async fn call(mut params: PeerFoldState, target_id: PeerId) -> Result<PeerFoldState, error::Error> {
    // To show which peer object needs to establish P2P link, it will be sent to WebRTC Gateway.
    let peer_info = params
        .peer_info()
        .clone()
        .expect("peer has not been created");

    let media_params = params.pop_media_config();
    if media_params.is_none() {
        info!("Call request with no media and no redirect is ignored");
        return Ok(params);
    }
    let media_config = media_params.unwrap();
    // In creating constraints process, source sockets of media and rtcp are opened.
    // It is necessary to store the sockets' information.
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

    // This channel is for redirecting keyboard inputs from PeerFold future to MediaFold future
    let (control_message_notifier, control_message_observer) = mpsc::channel::<ControlMessage>(0);

    // listen MediaConnection events and send them with this channel
    let (mc_event_notifier, mc_event_observer) = mpsc::channel::<media::MediaConnectionEvents>(0);
    let event_listen_fut = media::listen_events(media_connection_id.clone(), mc_event_notifier);
    tokio::spawn(event_listen_fut);

    // MediaConnection process will work according to MediaConnection events and keyboard inputs
    let stream = futures::stream::select(
        mc_event_observer.map(|event| Either::Left(event)),
        control_message_observer.map(|event| Either::Right(event)),
    );
    // Create initial state of fold future of media events
    let mut state = MediaConnectionState::default();
    state.insert_media_connection_id(
        media_connection_id,
        (video_src_socket, audio_src_socket, Some(redirect_params)),
    );
    let fold_fut = stream.fold(state, |state, event| async move {
        match event {
            Either::Left(event) => on_media_api_events(state, event)
                .await
                .expect("error in on_media_and_api_events"),
            Either::Right(event) => on_media_key_events(state, event)
                .await
                .expect("error in on_media_key_events"),
        }
    });
    tokio::spawn(fold_fut);

    // hold keybord events notifier
    params.set_control_message_notifier(control_message_notifier);
    Ok(params)
}

async fn answer(
    mut params: PeerFoldState,
    event: PeerCallEvent,
) -> Result<PeerFoldState, error::Error> {
    let media_params = params.pop_media_config();
    if media_params.is_none() {
        info!("Call request with no media and no redirect is ignored");
        return Ok(params);
    }
    let media_connection_id = event.call_params.media_connection_id;
    let media_config = media_params.unwrap();

    // In creating constraints process, source sockets of media and rtcp are opened.
    // It is necessary to store the sockets' information.
    let (video_src_socket, audio_src_socket, constraints) =
        create_media_connect_options(&media_config).await?;
    let redirect_params = create_redirect(media_config);

    let answer_params = AnswerQuery {
        constraints: constraints,
        redirect_params: Some(redirect_params.clone()),
    };

    // answer to call from neighbour
    let _answer = media::answer(&media_connection_id, &answer_params).await?;

    // This channel is for redirecting keyboard inputs from PeerFold future to MediaFold future
    let (control_message_notifier, control_message_observer) = mpsc::channel::<ControlMessage>(0);

    // listen MediaConnection events and send them with this channel
    let (mc_event_notifier, mc_event_observer) = mpsc::channel::<media::MediaConnectionEvents>(0);
    let event_listen_fut = media::listen_events(media_connection_id.clone(), mc_event_notifier);
    tokio::spawn(event_listen_fut);

    // MediaConnection process will work according to MediaConnection events and keyboard inputs
    let stream = futures::stream::select(
        mc_event_observer.map(|event| Either::Left(event)),
        control_message_observer.map(|event| Either::Right(event)),
    );
    // Create initial state of fold future of media events
    let mut state = MediaConnectionState::default();
    state.insert_media_connection_id(
        media_connection_id,
        (video_src_socket, audio_src_socket, Some(redirect_params)),
    );
    let fold_fut = stream.fold(state, |state, event| async move {
        match event {
            Either::Left(event) => on_media_api_events(state, event)
                .await
                .expect("error in on_media_and_api_events"),
            Either::Right(event) => on_media_key_events(state, event)
                .await
                .expect("error in on_media_key_events"),
        }
    });
    tokio::spawn(fold_fut);

    // hold keybord events notifier
    params.set_control_message_notifier(control_message_notifier);
    Ok(params)
}

//==================== create query for call and answer ====================

// Open some sockets to create constraints object, and return these information for call and answer
async fn create_media_connect_options(
    media_params: &MediaConfig,
) -> Result<
    (
        Option<MediaSrcSockets>,
        Option<MediaSrcSockets>,
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
            Some(MediaSrcSockets {
                media_socket: video_src_socket.clone(),
                rtcp_socket: rtcp_src_socket.clone(),
            }),
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
            Some(MediaSrcSockets {
                media_socket: audio_src_socket.clone(),
                rtcp_socket: rtcp_src_socket.clone(),
            }),
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

//==================== process events for media ====================

fn create_socket_state_message(
    value: &(
        Option<MediaSrcSockets>,
        Option<MediaSrcSockets>,
        Option<RedirectParameters>,
    ),
) -> String {
    let mut message = String::from("");
    if let Some(ref socket_info) = value.0 {
        message = format!(
            "{}Its video src socket is {}:{}",
            message,
            socket_info.media_socket.ip(),
            socket_info.media_socket.port()
        );
        message = format!(
            "{}\nAlso, rtcp socket is ready for {}:{}",
            message,
            socket_info.rtcp_socket.ip(),
            socket_info.rtcp_socket.port()
        );
    }
    if let Some(ref socket_info) = value.1 {
        message = format!(
            "{}\nIts Audio src socket is {}:{}",
            message,
            socket_info.media_socket.ip(),
            socket_info.media_socket.port()
        );
        message = format!(
            "{}\nAlso, rtcp socket is ready for {}:{}",
            message,
            socket_info.rtcp_socket.ip(),
            socket_info.rtcp_socket.port()
        );
    }
    if let Some(ref redirect) = value.2 {
        if let Some(ref video_redirect) = redirect.video {
            message = format!(
                "{}\nReceived Video is redirect to {}:{}",
                message,
                video_redirect.ip(),
                video_redirect.port()
            );
        }
        if let Some(ref rtcp_redirect) = redirect.video_rtcp {
            message = format!(
                "{}\nReceived RTCP for Video is redirect to {}:{}",
                message,
                rtcp_redirect.ip(),
                rtcp_redirect.port()
            );
        }
        if let Some(ref audio_redirect) = redirect.audio {
            message = format!(
                "{}\nReceived Audio is redirect to {}:{}",
                message,
                audio_redirect.ip(),
                audio_redirect.port()
            );
        }
        if let Some(ref rtcp_redirect) = redirect.audio_rtcp {
            message = format!(
                "{}\nReceived RTCP for Audio is redirect to {}:{}",
                message,
                rtcp_redirect.ip(),
                rtcp_redirect.port()
            );
        }
    }

    message
}

// This function process MediaConnection events
async fn on_media_api_events(
    state: MediaConnectionState,
    event: media::MediaConnectionEvents,
) -> Result<MediaConnectionState, error::Error> {
    let status = match event {
        media::MediaConnectionEvents::READY(media_connection_id) => {
            let mut message = format!(
                "====================\nMediaConnection {} is ready to send-recv media",
                media_connection_id.as_str()
            );
            let value = state
                .get(&media_connection_id)
                .expect("socket info not set");
            message = format!("{}\n{}", message, create_socket_state_message(value));

            info!("{}", message);
            Ok(state)
        }
        media::MediaConnectionEvents::CLOSE(media_connection_id) => {
            info!(
                "====================\nMediaConnection {} is closed",
                media_connection_id.as_str()
            );
            Ok(state)
        }
        media::MediaConnectionEvents::STREAM(media_connection_id) => {
            let message = format!(
                "====================\nRecv stream from MediaConnection {}",
                media_connection_id.as_str()
            );
            info!("{}", message);
            Ok(state)
        }
        media::MediaConnectionEvents::ERROR((media_connection_id, message)) => {
            error!(
                "error {:?} in MediaConnection {}",
                message,
                media_connection_id.as_str()
            );
            Ok(state)
        }
    };
    print_commands();
    status
}

// This function process Keyboard Inputs
async fn on_media_key_events(
    mut state: MediaConnectionState,
    ControlMessage(message): ControlMessage,
) -> Result<MediaConnectionState, error::Error> {
    let status = match message.as_str() {
        "status" => {
            // prinnts all MediaConnection status
            for media_connection_id in state.media_connection_id_iter() {
                let status = media::status(&media_connection_id).await?;
                let mut message = format!(
                    "MediaConnection {} is as follows",
                    media_connection_id.as_str()
                );
                message = format!("{}\nis_open: {}", message, status.open);
                message = format!(
                    "{}\nneighbour PeerId: {}",
                    message,
                    status.remote_id.as_str()
                );
                message = format!("{}\nIts ssrc is: {:?}", message, status.ssrc);

                let value = state
                    .get(&media_connection_id)
                    .expect("socket info not set");
                let tmp_message = create_socket_state_message(value);
                message = format!(
                    "{}\nIt can redirect media as follows\n {}",
                    message, tmp_message
                );
                info!("{}", message);
            }
            Ok(state)
        }
        message if message.starts_with("pli ") => {
            // close P2P link
            let mut args = message.split_whitespace();
            let _ = args.next();
            if let Some(media_connection_id) = args.next() {
                let media_connection_id = MediaConnectionId::new(media_connection_id);
                if state.contains(&media_connection_id) {
                    //FIXME: always send pli for video
                    if let Some((_, _, Some(redirect))) = state.get(&media_connection_id) {
                        let socket = redirect.video.clone().unwrap();
                        let result = media::send_pli(&media_connection_id, &socket).await;
                        if result.is_ok() {
                            info!("====================\npli send OK");
                        } else {
                            info!("====================\npli send Error {:?}", result.err());
                        }
                    } else {
                        warn!(
                            "{:?} is not a valid Media Connection Id",
                            media_connection_id
                        );
                    }
                } else {
                    warn!(
                        "{:?} is not a valid Media Connection Id",
                        media_connection_id
                    );
                }
            } else {
                warn!("input \"pli MEDIA_CONNECTION_ID\"");
            }
            Ok(state)
        }
        message if message.starts_with("disconnect ") => {
            // close P2P link
            let mut args = message.split_whitespace();
            let _ = args.next();
            if let Some(media_connection_id) = args.next() {
                let media_connection_id = MediaConnectionId::new(media_connection_id);
                if state.contains(&media_connection_id) {
                    let _ = media::disconnect(&media_connection_id).await?;
                    state.remove_media_connection_id(&media_connection_id);
                } else {
                    warn!(
                        "{:?} is not a valid Media Connection Id",
                        media_connection_id
                    );
                }
            } else {
                warn!("input \"disconnect MEDIA_CONNECTION_ID\"");
            }
            Ok(state)
        }
        "disconnect_all" => {
            for media_connection_id in state.clone().media_connection_id_iter() {
                let _ = media::disconnect(media_connection_id).await?;
                state.remove_media_connection_id(media_connection_id);
            }

            Ok(state)
        }
        _ => Ok::<_, error::Error>(state),
    };
    print_commands();
    status
}

// Process for MediaConnection reacts to fold stream of MediaConnection events and UserInput streams.
// This struct shows the previous state.
#[derive(Clone, Default)]
struct MediaConnectionState(
    HashMap<
        MediaConnectionId,
        (
            // video src sockets
            Option<MediaSrcSockets>,
            // audio src sockets
            Option<MediaSrcSockets>,
            // redirect information
            Option<RedirectParameters>,
        ),
    >,
);

// This struct has only setter and getter.
impl MediaConnectionState {
    pub fn media_connection_id_iter(
        &self,
    ) -> Keys<
        MediaConnectionId,
        (
            Option<MediaSrcSockets>,
            Option<MediaSrcSockets>,
            Option<RedirectParameters>,
        ),
    > {
        self.0.keys()
    }

    pub fn insert_media_connection_id(
        &mut self,
        media_connection_id: MediaConnectionId,
        value: (
            Option<MediaSrcSockets>,
            Option<MediaSrcSockets>,
            Option<RedirectParameters>,
        ),
    ) {
        let _ = self.0.insert(media_connection_id, value);
    }

    pub fn remove_media_connection_id(&mut self, media_connection_id: &MediaConnectionId) {
        let _ = self.0.remove(&media_connection_id);
    }

    pub fn contains(&self, media_connection_id: &MediaConnectionId) -> bool {
        self.0.contains_key(media_connection_id)
    }

    pub fn get(
        &self,
        media_connection_id: &MediaConnectionId,
    ) -> Option<&(
        Option<MediaSrcSockets>,
        Option<MediaSrcSockets>,
        Option<RedirectParameters>,
    )> {
        self.0.get(media_connection_id)
    }
}

//==================== helper ====================

fn print_commands() {
    let message = "exit\n\
    call PEER_ID\n\
    status\n\
    disconnect MEDIA_CONNECTION_ID";

    println!(
        "====================\ninput following commands\n{}\n====================",
        message
    );
}
