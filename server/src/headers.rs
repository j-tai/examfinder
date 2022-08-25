//! Middleware for setting headers.

use axum::headers::HeaderName;
use axum::middleware::Next;
use axum::response::Response;
use hyper::header::{CACHE_CONTROL, REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS};
use hyper::Request;

/// Adds a header only if it's not already present.
fn add_header(response: &mut Response, header: HeaderName, value: &str) {
    if !response.headers().contains_key(&header) {
        response
            .headers_mut()
            .insert(header, value.try_into().unwrap());
    }
}

/// Main middleware for configuring headers
pub async fn middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(req).await;
    if response.status().is_success() {
        add_header(&mut response, CACHE_CONTROL, "public, max-age=86400");
    }
    add_header(&mut response, X_FRAME_OPTIONS, "DENY");
    add_header(&mut response, X_CONTENT_TYPE_OPTIONS, "nosniff");
    add_header(&mut response, REFERRER_POLICY, "no-referrer");
    response
}

/// Middleware for immutable static resources.
pub async fn immutable<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(req).await;
    if response.status().is_success() {
        add_header(
            &mut response,
            CACHE_CONTROL,
            "public, max-age=604800, immutable",
        );
    }
    response
}
