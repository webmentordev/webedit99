use crate::AppState;

use crate::apis::products::*;

use anyhow::Result;
use axum::{Router, routing::get};

pub async fn init_api_routes(state: AppState) -> Result<Router> {
    let products = Router::new().route("/health", get(health));

    let routes = Router::new().nest("/products", products).with_state(state);
    Ok(routes)
}
