use thiserror::Error;

#[derive(Error, Debug)]
pub enum MineruError {
    #[error("HTTP error: {0}")]
    Http(#[from] hpx::Error),
    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API error: {0}")]
    Api(String),
}
