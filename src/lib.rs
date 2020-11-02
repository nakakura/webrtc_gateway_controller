mod common;
/// /data api bindings
pub mod data;
/// Definition of errors occur in this crate
pub mod error;
/// /media api bindings
pub mod media;
/// /peers api bindings
pub mod peer;
/// A "prelude" for users of this crate.
pub mod prelude;
/// helper to load yaml
pub(crate) mod helper;

use std::sync::Once;

static mut BASE_URL: String = String::new();
static INIT: Once = Once::new();
static INIT_CHECK: Once = Once::new();

/// Initialize this crate with base url of WebRTC Gateway.
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
