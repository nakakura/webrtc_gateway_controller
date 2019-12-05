mod common;
mod data;
mod error;
mod media;
pub mod peer;

use lazy_static::*;

#[cfg(not(test))]
lazy_static! {
    pub static ref API_KEY: String =
        ::std::env::var("API_KEY").expect("API_KEY is not set in environment variables");
    pub static ref BASE_URL: String =
        ::std::env::var("BASE_URL").unwrap_or("http://localhost:8000".to_string());
    pub static ref DOMAIN: String =
        ::std::env::var("DOMAIN").expect("API_KEY is not set in environment variables");
    pub static ref PEER_ID: String =
        ::std::env::var("PEER_ID").expect("API_KEY is not set in environment variables");
}

#[cfg(test)]
lazy_static! {
    static ref API_KEY: String = "test-key".to_string();
    static ref DOMAIN: String = "localhost".to_string();
}
