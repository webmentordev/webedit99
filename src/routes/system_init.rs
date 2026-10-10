use crate::AppState;

use crate::system::stores::*;

use anyhow::Result;
use axum::{Router, routing::get};

pub async fn init_system_routes(state: AppState) -> Result<Router> {
    let store = Router::new().route("/health", get(health));

    let routes = Router::new().nest("/stores", store).with_state(state);
    Ok(routes)
}
