mod db;

use db::*;

use anyhow::{Context, Result};
use std::net::SocketAddr;

use axum::Json;
use axum::response::IntoResponse;
use axum::{Router, http::StatusCode, routing::get};
use serde_json::json;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub jwt_secret: String,
    pub domain: String,
    pub store_domain: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = connect_database()
        .await
        .context("could not establish database connection pool")?;
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let domain = std::env::var("DOMAIN").expect("DOMAIN must be set");
    let store_domain = std::env::var("STORE_DOMAIN").expect("STORE_DOMAIN must be set");

    let _state = AppState {
        db: pool,
        jwt_secret,
        domain,
        store_domain,
    };

    let app = Router::new()
        .nest_service("/uploads", ServeDir::new("uploads"))
        .route("/health", get(health));

    let addr = SocketAddr::from(([0, 0, 0, 0], 7780));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("🚀 Server running at http://127.0.0.1:{}", addr.port());
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "message": "System is live!" })),
    )
}
