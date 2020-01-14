mod common;
pub mod data;
pub mod error;
pub mod media;
pub mod peer;
pub mod terminal;

use lazy_static::*;

lazy_static! {
    pub static ref BASE_URL: String =
        ::std::env::var("BASE_URL").unwrap_or("http://localhost:8000".to_string());
}

pub fn base_url() -> &'static str {
    &*BASE_URL
}
