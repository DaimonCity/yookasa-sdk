use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidRequest,
    InvalidCredentials,
    Forbidden,
    NotFound,
    Gone,
    TooManyRequests,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorBody {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub code: Option<ErrorCode>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parameter: Option<String>,
    #[serde(default)]
    pub retry_after: Option<u64>,
}

#[derive(Debug, Error)]
pub enum YookassaError {
    #[error("http transport error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid idempotence key")]
    InvalidIdempotenceKey,
    #[error("this endpoint requires OAuth authentication")]
    OAuthRequired,
    #[error("api error: status={status}, body={body:?}")]
    Api {
        status: StatusCode,
        body: ApiErrorBody,
    },
    #[error("api returned unexpected status {status}: {body}")]
    UnexpectedStatus { status: StatusCode, body: String },
}
