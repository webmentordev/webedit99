use crate::AppState;
use crate::middleware::require_auth;

use crate::system::brands::*;
use crate::system::collections::*;
use crate::system::domain_routes::*;
use crate::system::product_extra_fees::*;
use crate::system::product_specifications::*;
use crate::system::product_sub_variants::*;
use crate::system::product_variants::*;
use crate::system::products::*;
use crate::system::stores::*;
use crate::system::sub_collections::*;

use anyhow::Result;
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub async fn init_system_routes(state: AppState) -> Result<Router> {
    let public = Router::new().route("/health", get(health));

    let protected = Router::new()
        .route("/", get(list_my_stores).post(create_store))
        .route("/{id}", get(get_my_store))
        .route("/{id}/verify-domain", post(verify_domain_handler))
        .route("/{id}/products", get(list_products).post(create_product))
        .route("/{id}/brands", get(list_brands).post(create_brand))
        .route(
            "/{id}/collections",
            get(list_collections).post(create_collection),
        )
        .route(
            "/{id}/sub-collections",
            get(list_sub_collections).post(create_sub_collection),
        )
        .route(
            "/{id}/product-variants",
            get(list_variants).post(create_variant),
        )
        .route(
            "/{id}/product-sub-variants",
            get(list_sub_variants).post(create_sub_variant),
        )
        .route(
            "/{id}/product-specifications",
            get(list_specifications).post(create_specification),
        )
        .route(
            "/{id}/product-extra-fees",
            get(list_extra_fees).post(create_extra_fee),
        )
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let store = Router::new().merge(public).merge(protected);

    let routes = Router::new().nest("/stores", store).with_state(state);
    Ok(routes)
}
