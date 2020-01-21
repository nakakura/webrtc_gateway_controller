mod terminal;

use std::collections::hash_set::Iter;
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

use either;
use futures::channel::mpsc;
use futures::future::FutureExt;
use futures::prelude::*;
use futures::*;
use log::{info, warn};
use serde_derive::Deserialize;

use either::Either;
use peer::formats::PeerEventEnum;
use std::collections::hash_map::Keys;
use std::net::SocketAddr;
use webrtc_gateway_controller::common::{DataConnectionId, PeerId, PeerInfo};
use webrtc_gateway_controller::data::formats::{
    CreatedResponse, DataId, DataIdWrapper, RedirectDataParams, RedirectParams,
};
use webrtc_gateway_controller::*;

// It shows config toml formats
#[derive(Debug, Deserialize)]
struct Config {
    peer: PeerConfig,
    gateway: SocketConfig,
    media: Vec<MediaConfig>,
}

// It is internal format for config toml
#[derive(Debug, Deserialize)]
struct PeerConfig {
    peer_id: String,
    domain: String,
}

// It is internal format for config toml
#[derive(Debug, Deserialize)]
struct MediaConfig {
    video: bool,
    audio: bool,
    video_redirect: Option<SocketConfig>,
    video_params: Option<MediaParams>,
    audio_redirect: Option<SocketConfig>,
    audio_params: Option<MediaParams>,
}

// It is internal format for config toml
#[derive(Debug, Deserialize)]
struct SocketConfig {
    ip: String,
    port: u16,
}

// It is internal format for config toml
#[derive(Debug, Deserialize)]
struct MediaParams {
    bandwidth: u16,
    codec: String,
    payload_type: u16,
    sampling_rate: usize,
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
    println!("{:?}", config);
}
