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
use crate::utils::generate_sku;

const DEFAULT_PER_PAGE: i64 = 20;
const MAX_PER_PAGE: i64 = 100;
const MAX_SKU_ATTEMPTS: usize = 5;

#[derive(Debug, Serialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListSubVariantsQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub is_active: Option<bool>,
    pub product_variant_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct SubVariantListItem {
    pub id: Uuid,
    pub product_variant_id: Uuid,
    pub variant_title: String,
    pub title: String,
    pub sku: String,
    pub price: i64,
    pub compare_at_price: Option<i64>,
    pub cost_price: Option<i64>,
    pub quantity: Option<i32>,
    pub low_stock_threshold: Option<i32>,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct VariantOption {
    pub id: Uuid,
    pub title: String,
    pub product_title: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubVariant {
    pub product_variant_id: Uuid,
    pub title: String,
    pub sku: Option<String>,
    pub price: i64,
    pub compare_at_price: Option<i64>,
    pub cost_price: Option<i64>,
    pub quantity: Option<i32>,
    pub low_stock_threshold: Option<i32>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
}

pub async fn list_sub_variants(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Query(q): Query<ListSubVariantsQuery>,
) -> Result<Json<Paginated<SubVariantListItem>>, StatusCode> {
    authorize(
        &state,
        claims.sub,
        store_id,
        "product_sub_variants",
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
        r#"SELECT sv.id, sv.product_variant_id, pv.title AS variant_title,
                  sv.title, sv.sku, sv.price, sv.compare_at_price, sv.cost_price,
                  sv.quantity, sv.low_stock_threshold, sv.is_active, sv.sort_order,
                  sv.created_at, sv.updated_at,
                  COUNT(*) OVER() AS "total!"
           FROM product_sub_variants sv
           JOIN product_variants pv ON pv.id = sv.product_variant_id
           WHERE sv.store_id = $1
             AND ($2::text IS NULL OR sv.title ILIKE '%' || $2 || '%' OR sv.sku ILIKE '%' || $2 || '%')
             AND ($3::bool IS NULL OR sv.is_active = $3)
             AND ($4::uuid IS NULL OR sv.product_variant_id = $4)
           ORDER BY sv.sort_order ASC, sv.created_at DESC
           LIMIT $5 OFFSET $6"#,
        store_id,
        search,
        q.is_active,
        q.product_variant_id,
        per_page,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = rows.first().map(|r| r.total).unwrap_or(0);

    let data = rows
        .into_iter()
        .map(|r| SubVariantListItem {
            id: r.id,
            product_variant_id: r.product_variant_id,
            variant_title: r.variant_title,
            title: r.title,
            sku: r.sku,
            price: r.price,
            compare_at_price: r.compare_at_price,
            cost_price: r.cost_price,
            quantity: r.quantity,
            low_stock_threshold: r.low_stock_threshold,
            is_active: r.is_active,
            sort_order: r.sort_order,
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

pub async fn list_variant_options(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Vec<VariantOption>>, StatusCode> {
    authorize(
        &state,
        claims.sub,
        store_id,
        "product_sub_variants",
        Action::View,
    )
    .await?;

    let rows = sqlx::query!(
        r#"SELECT pv.id, pv.title, p.title AS product_title
           FROM product_variants pv
           JOIN products p ON p.id = pv.product_id
           WHERE p.store_id = $1
             AND p.deleted_at IS NULL
           ORDER BY p.title ASC, pv.title ASC
           LIMIT 500"#,
        store_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let data = rows
        .into_iter()
        .map(|r| VariantOption {
            id: r.id,
            title: r.title,
            product_title: r.product_title,
        })
        .collect();

    Ok(Json(data))
}

pub async fn create_sub_variant(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Json(payload): Json<CreateSubVariant>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    authorize(
        &state,
        claims.sub,
        store_id,
        "product_sub_variants",
        Action::Create,
    )
    .await?;

    let title = payload
        .title
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if title.is_empty() || title.chars().count() > 255 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let sku_input = payload
        .sku
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    if sku_input.as_ref().is_some_and(|s| s.chars().count() > 100) {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.price < 0
        || payload.compare_at_price.is_some_and(|v| v < 0)
        || payload.cost_price.is_some_and(|v| v < 0)
        || payload.quantity.is_some_and(|v| v < 0)
        || payload.low_stock_threshold.is_some_and(|v| v < 0)
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let variant_exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(
               SELECT 1 FROM product_variants pv
               JOIN products p ON p.id = pv.product_id
               WHERE pv.id = $1
                 AND p.store_id = $2
                 AND p.deleted_at IS NULL
           ) AS "exists!""#,
        payload.product_variant_id,
        store_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !variant_exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    let attempts = if sku_input.is_some() {
        1
    } else {
        MAX_SKU_ATTEMPTS
    };

    let mut created = None;
    for _ in 0..attempts {
        let sku = match &sku_input {
            Some(s) => s.clone(),
            None => generate_sku(&title),
        };

        let row = sqlx::query!(
            r#"INSERT INTO product_sub_variants
                   (id, store_id, product_variant_id, title, sku, price, compare_at_price,
                    cost_price, quantity, low_stock_threshold, is_active, sort_order)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
               ON CONFLICT (store_id, sku) DO NOTHING
               RETURNING id, title, sku"#,
            Uuid::new_v4(),
            store_id,
            payload.product_variant_id,
            title,
            sku,
            payload.price,
            payload.compare_at_price,
            payload.cost_price,
            payload.quantity,
            payload.low_stock_threshold,
            payload.is_active.unwrap_or(true),
            payload.sort_order.unwrap_or(1)
        )
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if row.is_some() {
            created = row;
            break;
        }
    }

    let row = created.ok_or(StatusCode::CONFLICT)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "id": row.id,
            "title": row.title,
            "sku": row.sku
        })),
    ))
}
