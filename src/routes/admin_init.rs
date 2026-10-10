use crate::AppState;

use crate::admin::users::*;

use anyhow::Result;
use axum::{Router, routing::get};

pub async fn init_admin_routes(state: AppState) -> Result<Router> {
    let users = Router::new().route("/health", get(health));

    let routes = Router::new().nest("/users", users).with_state(state);
    Ok(routes)
}
