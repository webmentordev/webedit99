mod admin;
mod apis;
mod assets;
mod auth;
mod db;
mod domain;
mod middleware;
mod routes;
mod store;
mod system;
mod utils;

use assets::*;
use db::*;
use routes::accounts_init::*;
use routes::admin_init::*;
use routes::apis_init::*;
use routes::system_init::*;
use store::{storefront, tls_check};
use utils::*;

use anyhow::{Context, Result};
use std::net::SocketAddr;
use std::time::Duration;

use axum::Json;
use axum::http::{HeaderName, HeaderValue, Method, header};
use axum::response::IntoResponse;
use axum::{Router, http::StatusCode, routing::get};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub jwt_secret: String,
    pub domain: String,
    pub store_domain: String,
    pub ip_address: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = connect_database()
        .await
        .context("could not establish database connection pool")?;
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let domain = std::env::var("DOMAIN").expect("DOMAIN must be set");
    let store_domain = std::env::var("STORE_DOMAIN").expect("STORE_DOMAIN must be set");
    let ip_address = std::env::var("IP_ADDRESS").expect("IP_ADDRESS must be set");

    let state = AppState {
        db: pool,
        jwt_secret,
        domain,
        store_domain,
        ip_address,
    };

    let private = private_cors(&state.domain);

    let system = init_system_routes(state.clone())
        .await?
        .layer(private.clone());
    let admin = init_admin_routes(state.clone())
        .await?
        .layer(private.clone());
    let account = init_account_route(state.clone())
        .await?
        .layer(private.clone());
    let apis = init_api_routes(state.clone()).await?.layer(public_cors());

    let health_router = Router::new().route("/health", get(health)).layer(private);

    let public_files = Router::new()
        .nest_service("/uploads", ServeDir::new("uploads"))
        .route("/assets/{store}/{file}", get(get_asset))
        .layer(public_cors());

    let storefront_router = Router::new()
        .route("/tls-check", get(tls_check))
        .fallback(storefront)
        .with_state(state.clone());

    let app = Router::new()
        .nest("/admin", admin)
        .nest("/api", apis)
        .nest("/system", system)
        .nest("/account", account)
        .merge(health_router)
        .merge(public_files)
        .merge(storefront_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7780));
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

fn private_cors(frontend: &str) -> CorsLayer {
    let base = frontend.trim_end_matches('/').to_string();
    let mut origins: Vec<HeaderValue> = vec![base.parse().unwrap()];
    if let Some(rest) = base.strip_prefix("https://") {
        origins.push(format!("https://www.{rest}").parse().unwrap());
    }
    if cfg!(debug_assertions) {
        origins.push("http://localhost:3000".parse().unwrap());
        origins.push("http://127.0.0.1:3000".parse().unwrap());
    }

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600))
}

fn public_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            HeaderName::from_static("x-store"),
        ])
        .max_age(Duration::from_secs(86400))
}
