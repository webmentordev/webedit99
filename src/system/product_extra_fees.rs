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
pub struct ListExtraFeesQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub is_active: Option<bool>,
    pub product_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct ExtraFeeListItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_title: String,
    pub title: String,
    pub price: i64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateExtraFee {
    pub product_id: Uuid,
    pub title: String,
    pub price: i64,
    pub is_active: Option<bool>,
}

pub async fn list_extra_fees(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Query(q): Query<ListExtraFeesQuery>,
) -> Result<Json<Paginated<ExtraFeeListItem>>, StatusCode> {
    authorize(
        &state,
        claims.sub,
        store_id,
        "product_extra_fees",
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
        r#"SELECT f.id, f.product_id, p.title AS product_title,
                  f.title, f.price, f.is_active,
                  f.created_at, f.updated_at,
                  COUNT(*) OVER() AS "total!"
           FROM product_extra_fees f
           JOIN products p ON p.id = f.product_id
           WHERE p.store_id = $1
             AND p.deleted_at IS NULL
             AND ($2::text IS NULL OR f.title ILIKE '%' || $2 || '%')
             AND ($3::bool IS NULL OR f.is_active = $3)
             AND ($4::uuid IS NULL OR f.product_id = $4)
           ORDER BY f.created_at DESC
           LIMIT $5 OFFSET $6"#,
        store_id,
        search,
        q.is_active,
        q.product_id,
        per_page,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = rows.first().map(|r| r.total).unwrap_or(0);

    let data = rows
        .into_iter()
        .map(|r| ExtraFeeListItem {
            id: r.id,
            product_id: r.product_id,
            product_title: r.product_title,
            title: r.title,
            price: r.price,
            is_active: r.is_active,
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

pub async fn create_extra_fee(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Json(payload): Json<CreateExtraFee>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    authorize(
        &state,
        claims.sub,
        store_id,
        "product_extra_fees",
        Action::Create,
    )
    .await?;

    let title = payload
        .title
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if title.is_empty() || title.chars().count() > 255 || payload.price < 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let product_exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(
               SELECT 1 FROM products
               WHERE id = $1 AND store_id = $2 AND deleted_at IS NULL
           ) AS "exists!""#,
        payload.product_id,
        store_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !product_exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    let row = sqlx::query!(
        r#"INSERT INTO product_extra_fees (id, product_id, title, price, is_active)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, title, price"#,
        Uuid::new_v4(),
        payload.product_id,
        title,
        payload.price,
        payload.is_active.unwrap_or(true)
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "id": row.id,
            "title": row.title,
            "price": row.price
        })),
    ))
}
