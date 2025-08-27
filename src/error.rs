use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("url: {0}")]
    Url(#[from] url::ParseError),
    #[error("decode: {0}")]
    Decode(#[from] serde_json::Error),
    #[error("api status {status}: {body}")]
    Status { status: u16, body: String },
}

pub(crate) async fn map_json<T: serde::de::DeserializeOwned>(
    res: reqwest::Response,
) -> Result<T, ApiError> {
    let status = res.status();
    let text = res.text().await?;
    if status.is_success() {
        Ok(serde_json::from_str(&text)?)
    } else {
        Err(ApiError::Status { status: status.as_u16(), body: text })
    }
}
