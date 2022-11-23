use std::env;

use axum::extract::Query;
use axum::response::Html;
use axum::routing::{get, get_service};
use axum::{middleware, Extension, Json, Router, Server};
use db::Redis;
use error::AppResult;
use exam::Exam;
use serde::{Deserialize, Serialize};
use tokio::signal::unix::SignalKind;
use tokio::{fs, select, signal};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tracing::info;

use crate::error::AppError;

mod db;
mod error;
mod exam;
mod headers;
mod scraper;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let redis_url = get_env("REDIS_URL");
    let dist_dir = {
        #[cfg(feature = "packaged")]
        {
            "/usr/local/share/examfinder/client".to_string()
        }
        #[cfg(not(feature = "packaged"))]
        {
            get_env("CLIENT_DIST_DIR")
        }
    };

    let redis = db::connect(&redis_url).await;
    let assets_dir = format!("{dist_dir}/assets");

    let app = Router::new()
        .route("/api/v1/get", get(get_exams))
        .route("/", get(move || async move { serve_html(&dist_dir).await }))
        .nest(
            "/assets",
            get_service(ServeDir::new(&assets_dir))
                .handle_error(|e| async move { AppError::from(e) })
                .layer(middleware::from_fn(headers::immutable)),
        )
        .layer(
            ServiceBuilder::new()
                .layer(Extension(redis))
                .layer(middleware::from_fn(headers::middleware)),
        );

    info!("Starting server");

    Server::bind(&"0.0.0.0:3843".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            // Tell the server to shut down when either SIGINT or SIGTERM is
            // received
            let sig_int = async { signal::ctrl_c().await.unwrap() };
            let sig_term = async {
                signal::unix::signal(SignalKind::terminate())
                    .unwrap()
                    .recv()
                    .await;
            };
            select! {
                _ = sig_int => {}
                _ = sig_term => {}
            }
        })
        .await
        .unwrap();
}

/// The query parameters for [`get_exams`].
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
struct AppQuery {
    course: String,
}

/// Serves the HTML web app.
async fn serve_html(dist_dir: &str) -> AppResult<Html<String>> {
    let path = format!("{dist_dir}/index.html");
    let html = fs::read_to_string(&path).await?;
    Ok(Html(html))
}

/// Returns a list of exams in JSON format.
async fn get_exams(
    Redis(mut redis): Redis,
    Query(query): Query<AppQuery>,
) -> AppResult<Json<Vec<Exam>>> {
    let results = scraper::fetch(&mut redis, &query.course).await?;
    Ok(Json(results))
}

/// Get the specified environment variable, or panic if it is not found.
fn get_env<'a>(name: &str) -> String {
    env::var(name).expect("variable should be in .env")
}
