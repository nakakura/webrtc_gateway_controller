use futures::*;
use reqwest;
use serde::{Deserialize, Serialize};

use crate::error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerId(pub String);

impl PeerId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct Token(pub String);

impl Token {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub token: Token,
}

pub async fn api_access<A: Sized, T: Sized, R: Sized>(
    success_code: reqwest::StatusCode,
    is_404_captable: bool,
    api_call: impl Fn() -> A,
    f: impl Fn(reqwest::Response) -> R,
) -> Result<T, error::ErrorEnum>
where
    A: Future<Output = Result<reqwest::Response, reqwest::Error>>,
    R: Future<Output = Result<T, error::ErrorEnum>>,
{
    let res = api_call().await?;
    match res.status() {
        code if code == success_code => f(res).await,
        reqwest::StatusCode::BAD_REQUEST => res
            .json::<error::ErrorResponse>()
            .await
            .map_err(Into::into)
            .and_then(|response: error::ErrorResponse| {
                let message = response
                    .params
                    .errors
                    .iter()
                    .fold("recv message".to_string(), |sum, acc| {
                        format!("{}\n{}", sum, acc.message)
                    });
                Err(error::ErrorEnum::create_myerror(&message))
            }),
        reqwest::StatusCode::FORBIDDEN => Err(error::ErrorEnum::create_myerror("recv Forbidden")),
        reqwest::StatusCode::NOT_FOUND if is_404_captable => {
            Err(error::ErrorEnum::create_myerror("recv Not Found"))
        }
        reqwest::StatusCode::METHOD_NOT_ALLOWED => {
            Err(error::ErrorEnum::create_myerror("recv Method Not Allowed"))
        }
        reqwest::StatusCode::NOT_ACCEPTABLE => {
            Err(error::ErrorEnum::create_myerror("recv Not Acceptable"))
        }
        reqwest::StatusCode::REQUEST_TIMEOUT => {
            Err(error::ErrorEnum::create_myerror("recv RequestTimeout"))
        }
        _ => {
            unreachable!();
        }
    }
}
