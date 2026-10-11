use crate::AppState;
use crate::apis::customers::*;
use crate::apis::products::*;

use anyhow::Result;
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub async fn init_api_routes(state: AppState) -> Result<Router> {
    let products = Router::new().route("/health", get(health));

    let customers_public = Router::new()
        .route("/register", post(register_customer))
        .route("/login", post(login_customer));

    let customers_protected =
        Router::new()
            .route("/me", get(customer_me))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                require_customer,
            ));

    let customers = Router::new()
        .merge(customers_public)
        .merge(customers_protected);

    let routes = Router::new()
        .nest("/products", products)
        .nest("/customers", customers)
        .with_state(state);
    Ok(routes)
}
