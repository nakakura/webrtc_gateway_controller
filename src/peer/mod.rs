mod data;

use reqwest;
use reqwest::Client;
use serde_json::json;

use crate::error;

pub async fn create_peer(
    peer_id: &str,
    turn: bool,
) -> Result<data::CreatedResponse, error::ErrorEnum> {
    let key = &*crate::API_KEY;
    let json = json!({
        "key": key,
        "domain": *crate::DOMAIN,
        "turn": turn,
        "peer_id": peer_id,
    });

    let base_url = format!("{}/peers", &*crate::BASE_URL);
    let res = Client::new().post(&base_url).json(&json).send().await?;
    res.json::<data::CreatedResponse>()
        .await
        .map_err(Into::into)
}
