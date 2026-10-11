use crate::AppState;

use crate::admin::currency::*;
use crate::admin::users::*;

use anyhow::Result;
use axum::routing::patch;
use axum::{
    Router,
    routing::{Route, get},
};

pub async fn init_admin_routes(state: AppState) -> Result<Router> {
    let users = Router::new().route("/health", get(health));

    let currencies = Router::new()
        .route("/", get(get_currencies).post(create_currency))
        .route("/{id}", get(get_currency).patch(update_currency));

    let routes = Router::new()
        .nest("/users", users)
        .nest("/currencies", currencies)
        .with_state(state);
    Ok(routes)
}
