use crate::AppState;

use crate::admin::contact_messages::*;
use crate::admin::currency::*;
use crate::admin::customer_addresses::*;
use crate::admin::customers::*;
use crate::admin::store::*;
use crate::admin::store_members::*;
use crate::admin::users::*;

use anyhow::Result;
use axum::{Router, routing::get};

pub async fn init_admin_routes(state: AppState) -> Result<Router> {
    let users = Router::new().route("/health", get(health));

    let stores = Router::new().route("/", get(get_stores));
    let stores_members = Router::new().route("/", get(get_store_members));
    let customers = Router::new().route("/", get(get_customers));
    let customer_addresses = Router::new().route("/", get(get_customer_addresses));
    let contact_messages = Router::new().route("/", get(get_contact_messages));

    let currencies = Router::new()
        .route("/", get(get_currencies).post(create_currency))
        .route("/{id}", get(get_currency).patch(update_currency));

    let routes = Router::new()
        .nest("/users", users)
        .nest("/currencies", currencies)
        .nest("/stores", stores)
        .nest("/store-members", stores_members)
        .nest("/customers", customers)
        .nest("/customer-addresses", customer_addresses)
        .nest("/contact-messages", contact_messages)
        .with_state(state);
    Ok(routes)
}
