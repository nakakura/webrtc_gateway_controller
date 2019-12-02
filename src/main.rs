mod common;
mod data;
mod error;
mod media;
mod peer;

#[cfg(test)]
mod test_helper;

use lazy_static::*;

#[cfg(not(test))]
lazy_static! {
    static ref API_KEY: String =
        ::std::env::var("API_KEY").expect("API_KEY is not set in environment variables");
    static ref BASE_URL: String =
        ::std::env::var("BASE_URL").unwrap_or("http://localhost:8000".to_string());
    static ref DOMAIN: String =
        ::std::env::var("DOMAIN").expect("API_KEY is not set in environment variables");
    static ref PEER_ID: String =
        ::std::env::var("PEER_ID").expect("API_KEY is not set in environment variables");
}

#[cfg(test)]
lazy_static! {
    static ref API_KEY: String = "test-key".to_string();
    static ref DOMAIN: String = "localhost".to_string();
}

#[cfg(not(test))]
#[tokio::main]
async fn main() {
    let hoge = peer::create_peer(&*BASE_URL, &*PEER_ID, true);
    let x = hoge.await;
    println!("{:?}", x);
}
