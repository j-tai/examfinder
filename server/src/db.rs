//! Database utilities.

use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::Extension;

use crate::error::AppError;

pub type RedisClient = redis::Client;

pub async fn connect(url: &str) -> RedisClient {
    RedisClient::open(url).unwrap()
}

/// A Redis connection.
pub type RedisConn = redis::aio::Connection;

/// A Redis connection extractor.
pub struct Redis(pub RedisConn);

#[async_trait]
impl<B: Send> FromRequest<B> for Redis {
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(client) = Extension::<RedisClient>::from_request(req).await.unwrap();
        let conn = client.get_async_connection().await?;
        Ok(Redis(conn))
    }
}
