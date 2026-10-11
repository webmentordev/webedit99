use crate::AppState;
use crate::auth::*;
use crate::middleware::require_auth;

use anyhow::Result;
use axum::middleware as axum_middleware;
use axum::{Router, routing::post};

pub async fn init_account_route(state: AppState) -> Result<Router> {
    let public = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user));

    let protected = Router::new()
        .route("/logout", post(logout_user))
        .route_layer(axum_middleware::from_fn_with_state(
            state.clone(),
            require_auth,
        ));

    let routes = Router::new()
        .merge(public)
        .merge(protected)
        .with_state(state);

    Ok(routes)
}
