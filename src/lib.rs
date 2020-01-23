mod common;
pub mod data;
pub mod error;
pub mod media;
pub mod peer;

use std::sync::Once;

static mut BASE_URL: String = String::new();
static INIT: Once = Once::new();
static INIT_CHECK: Once = Once::new();

pub fn initialize(base_url: impl Into<String>) {
    unsafe {
        INIT.call_once(|| {
            BASE_URL = base_url.into();
        });
    }
}

//use crate::common::{MyId, MySocket, PhantomId};
pub(crate) fn base_url() -> &'static str {
    unsafe {
        INIT_CHECK.call_once(|| {
            if BASE_URL.len() == 0 {
                panic!("not initialized");
            }
        });
        &BASE_URL
    }
}

pub use common::{PhantomId, SerializableSocket, SocketInfo};
pub use data::formats::{DataConnectionId, DataId};
pub use media::formats::{MediaConnectionId, MediaId, RtcpId};
pub use peer::formats::{PeerEventEnum, PeerId, PeerInfo, Token};
