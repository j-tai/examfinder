//! A common error type.

use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use thiserror::Error;
use tracing::error;

/// An application error, which could be a user error or an internal error.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Custom(String),
    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Custom(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("{self}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
