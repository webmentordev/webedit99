use crate::AppState;
use crate::middleware::require_auth;

use crate::system::domain_routes::*;
use crate::system::stores::*;

use anyhow::Result;
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub async fn init_system_routes(state: AppState) -> Result<Router> {
    let public = Router::new().route("/health", get(health));

    let protected = Router::new()
        .route("/{id}/verify-domain", post(verify_domain_handler))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let store = Router::new().merge(public).merge(protected);

    let routes = Router::new().nest("/stores", store).with_state(state);
    Ok(routes)
}
