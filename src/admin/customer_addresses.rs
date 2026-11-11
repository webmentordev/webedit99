use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CustomerAddressSummary {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub customer_name: String,
    pub customer_email: String,
    pub store_id: Uuid,
    pub store_title: String,
    pub store_slug: String,
    pub label: Option<String>,
    pub name: String,
    pub phone: Option<String>,
    pub address_1: String,
    pub address_2: Option<String>,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub is_default_shipping: bool,
    pub is_default_billing: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_customer_addresses(
    State(state): State<AppState>,
) -> Result<Json<Vec<CustomerAddressSummary>>, StatusCode> {
    let addresses = sqlx::query_as!(
        CustomerAddressSummary,
        r#"SELECT
               ca.id,
               ca.customer_id,
               c.name AS customer_name,
               c.email AS customer_email,
               c.store_id,
               s.title AS store_title,
               s.slug AS store_slug,
               ca.label,
               ca.name,
               ca.phone,
               ca.address_1,
               ca.address_2,
               ca.postal_code,
               ca.city,
               ca.country,
               ca.is_default_shipping,
               ca.is_default_billing,
               ca.created_at,
               ca.updated_at
           FROM customer_addresses ca
           INNER JOIN customers c ON c.id = ca.customer_id
           INNER JOIN stores s ON s.id = c.store_id
           WHERE c.deleted_at IS NULL
           ORDER BY ca.created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(addresses))
}
