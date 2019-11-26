mod error;
mod peer;

use lazy_static::*;

lazy_static! {
    static ref API_KEY: String =
        ::std::env::var("API_KEY").expect("API_KEY is not set in environment variables");
    static ref DOMAIN: String = "localhost".to_string();
    static ref BASE_URL: String = "http://localhost:8000".to_string();
}

#[tokio::main]
async fn main() {
    let hoge = peer::create_peer("hoge", true);
    let x = hoge.await;
    println!("{:?}", x);
}
