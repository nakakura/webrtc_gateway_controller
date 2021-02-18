use futures::*;
use reqwest;

use crate::new_error;

/// It's a high-order function as a template of API access.
pub(crate) async fn api_access<A: Sized, T: Sized, R: Sized>(
    success_code: reqwest::StatusCode,
    is_404_captable: bool,
    api_call: impl Fn() -> A,
    f: impl Fn(reqwest::Response) -> R,
) -> Result<T, new_error::Error>
where
    A: Future<Output = Result<reqwest::Response, new_error::Error>>,
    R: Future<Output = Result<T, new_error::Error>>,
{
    let res = api_call().await?;
    match res.status() {
        code if code == success_code => f(res).await,
        reqwest::StatusCode::BAD_REQUEST => res
            .json::<new_error::ErrorResponse>()
            .await
            .map_err(Into::into)
            .and_then(|response: new_error::ErrorResponse| {
                let message = response
                    .params
                    .errors
                    .iter()
                    .fold("recv message".to_string(), |sum, acc| {
                        format!("{}\n{}", sum, acc.message)
                    });
                Err(new_error::Error::create_local_error(&message))
            }),
        reqwest::StatusCode::FORBIDDEN => {
            Err(new_error::Error::create_local_error("recv Forbidden"))
        }
        reqwest::StatusCode::NOT_FOUND if is_404_captable => {
            Err(new_error::Error::create_local_error("recv Not Found"))
        }
        reqwest::StatusCode::METHOD_NOT_ALLOWED => Err(new_error::Error::create_local_error(
            "recv Method Not Allowed",
        )),
        reqwest::StatusCode::NOT_ACCEPTABLE => {
            Err(new_error::Error::create_local_error("recv Not Acceptable"))
        }
        reqwest::StatusCode::REQUEST_TIMEOUT => {
            Err(new_error::Error::create_local_error("recv RequestTimeout"))
        }
        _ => {
            unreachable!();
        }
    }
}
