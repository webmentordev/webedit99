use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::AppState;
use crate::auth::Claims;
use crate::system::access::{Action, authorize};

const DEFAULT_PER_PAGE: i64 = 20;
const MAX_PER_PAGE: i64 = 100;

#[derive(Debug, Serialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListAddressesQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub customer_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct AddressListItem {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub customer_name: String,
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

#[derive(Debug, Deserialize)]
pub struct CreateAddress {
    pub customer_id: Uuid,
    pub label: Option<String>,
    pub name: String,
    pub phone: Option<String>,
    pub address_1: String,
    pub address_2: Option<String>,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub is_default_shipping: Option<bool>,
    pub is_default_billing: Option<bool>,
}

fn clean(value: Option<String>, max: usize) -> Result<Option<String>, StatusCode> {
    let value = value
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    if value.as_ref().is_some_and(|v| v.chars().count() > max) {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(value)
}

fn required(value: String, max: usize) -> Result<String, StatusCode> {
    let value = value.trim().to_string();
    if value.is_empty() || value.chars().count() > max {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(value)
}

pub async fn list_addresses(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Query(q): Query<ListAddressesQuery>,
) -> Result<Json<Paginated<AddressListItem>>, StatusCode> {
    authorize(
        &state,
        claims.sub,
        store_id,
        "customer_addresses",
        Action::View,
    )
    .await?;

    let page = q.page.unwrap_or(1).max(1);
    let per_page = q
        .per_page
        .unwrap_or(DEFAULT_PER_PAGE)
        .clamp(1, MAX_PER_PAGE);
    let offset = (page - 1) * per_page;

    let search = q
        .search
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let rows = sqlx::query!(
        r#"SELECT a.id, a.customer_id, c.name AS customer_name, a.label, a.name, a.phone,
                  a.address_1, a.address_2, a.postal_code, a.city, a.country,
                  a.is_default_shipping, a.is_default_billing,
                  a.created_at, a.updated_at,
                  COUNT(*) OVER() AS "total!"
           FROM customer_addresses a
           JOIN customers c ON c.id = a.customer_id
           WHERE c.store_id = $1
             AND c.deleted_at IS NULL
             AND ($2::text IS NULL
                  OR a.name ILIKE '%' || $2 || '%'
                  OR a.address_1 ILIKE '%' || $2 || '%'
                  OR a.city ILIKE '%' || $2 || '%'
                  OR a.postal_code ILIKE '%' || $2 || '%')
             AND ($3::uuid IS NULL OR a.customer_id = $3)
           ORDER BY a.created_at DESC
           LIMIT $4 OFFSET $5"#,
        store_id,
        search,
        q.customer_id,
        per_page,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = rows.first().map(|r| r.total).unwrap_or(0);

    let data = rows
        .into_iter()
        .map(|r| AddressListItem {
            id: r.id,
            customer_id: r.customer_id,
            customer_name: r.customer_name,
            label: r.label,
            name: r.name,
            phone: r.phone,
            address_1: r.address_1,
            address_2: r.address_2,
            postal_code: r.postal_code,
            city: r.city,
            country: r.country,
            is_default_shipping: r.is_default_shipping,
            is_default_billing: r.is_default_billing,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect();

    Ok(Json(Paginated {
        data,
        page,
        per_page,
        total,
    }))
}

pub async fn create_address(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Json(payload): Json<CreateAddress>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    authorize(
        &state,
        claims.sub,
        store_id,
        "customer_addresses",
        Action::Create,
    )
    .await?;

    let label = clean(payload.label, 100)?;
    let name = required(payload.name, 255)?;
    let phone = clean(payload.phone, 50)?;
    let address_1 = required(payload.address_1, 255)?;
    let address_2 = clean(payload.address_2, 255)?;
    let postal_code = required(payload.postal_code, 20)?;
    let city = required(payload.city, 100)?;
    let country = required(payload.country, 100)?;

    let customer_exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(
               SELECT 1 FROM customers
               WHERE id = $1 AND store_id = $2 AND deleted_at IS NULL
           ) AS "exists!""#,
        payload.customer_id,
        store_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !customer_exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    let is_default_shipping = payload.is_default_shipping.unwrap_or(false);
    let is_default_billing = payload.is_default_billing.unwrap_or(false);

    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if is_default_shipping {
        sqlx::query!(
            r#"UPDATE customer_addresses
               SET is_default_shipping = FALSE, updated_at = now()
               WHERE customer_id = $1 AND is_default_shipping"#,
            payload.customer_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    if is_default_billing {
        sqlx::query!(
            r#"UPDATE customer_addresses
               SET is_default_billing = FALSE, updated_at = now()
               WHERE customer_id = $1 AND is_default_billing"#,
            payload.customer_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let row = sqlx::query!(
        r#"INSERT INTO customer_addresses
               (id, customer_id, label, name, phone, address_1, address_2, postal_code,
                city, country, is_default_shipping, is_default_billing)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
           RETURNING id, name"#,
        Uuid::new_v4(),
        payload.customer_id,
        label,
        name,
        phone,
        address_1,
        address_2,
        postal_code,
        city,
        country,
        is_default_shipping,
        is_default_billing
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "id": row.id,
            "name": row.name
        })),
    ))
}
