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
use crate::utils::slugify;

const WARRANTY_TYPES: [&str; 4] = ["day", "hour", "month", "year"];

const DEFAULT_PER_PAGE: i64 = 20;
const MAX_PER_PAGE: i64 = 100;

#[derive(Debug, Deserialize)]
pub struct ListProductsQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ProductListItem {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub brand_id: Option<Uuid>,
    pub sort_order: i32,
    pub is_digital: bool,
    pub is_active: bool,
    pub is_featured: bool,
    pub has_inventory: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

pub async fn list_products(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Query(q): Query<ListProductsQuery>,
) -> Result<Json<Paginated<ProductListItem>>, StatusCode> {
    authorize(&state, claims.sub, store_id, "products", Action::View).await?;

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
        r#"SELECT id, title, slug, brand_id, sort_order, is_digital, is_active,
                  is_featured, has_inventory, created_at, updated_at,
                  COUNT(*) OVER() AS "total!"
           FROM products
           WHERE store_id = $1
             AND deleted_at IS NULL
             AND ($2::text IS NULL OR title ILIKE '%' || $2 || '%')
             AND ($3::bool IS NULL OR is_active = $3)
           ORDER BY sort_order ASC, created_at DESC
           LIMIT $4 OFFSET $5"#,
        store_id,
        search,
        q.is_active,
        per_page,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = rows.first().map(|r| r.total).unwrap_or(0);

    let data = rows
        .into_iter()
        .map(|r| ProductListItem {
            id: r.id,
            title: r.title,
            slug: r.slug,
            brand_id: r.brand_id,
            sort_order: r.sort_order,
            is_digital: r.is_digital,
            is_active: r.is_active,
            is_featured: r.is_featured,
            has_inventory: r.has_inventory,
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

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub title: String,
    pub slug: Option<String>,
    pub brand_id: Option<Uuid>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub sort_order: Option<i32>,
    pub is_digital: Option<bool>,
    pub is_active: Option<bool>,
    pub is_featured: Option<bool>,
    pub min_order_qty: Option<i32>,
    pub max_order_qty: Option<i32>,
    pub has_warranty: Option<bool>,
    pub warranty_type: Option<String>,
    pub warranty_value: Option<i32>,
    pub has_inventory: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
}

fn clean(value: Option<String>) -> Option<String> {
    value
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub async fn create_product(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Json(payload): Json<CreateProduct>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    authorize(&state, claims.sub, store_id, "products", Action::Create).await?;

    let title = payload
        .title
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if title.is_empty() || title.chars().count() > 255 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let slug = match clean(payload.slug) {
        Some(s) => slugify(&s, false),
        None => slugify(&title, false),
    };

    if slug.is_empty() || slug.chars().count() > 255 {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.min_order_qty.is_some_and(|v| v < 1) || payload.max_order_qty.is_some_and(|v| v < 1)
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    if let (Some(min), Some(max)) = (payload.min_order_qty, payload.max_order_qty) {
        if min > max {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let has_warranty = payload.has_warranty.unwrap_or(false);
    let (warranty_type, warranty_value) = if has_warranty {
        let kind = payload.warranty_type.ok_or(StatusCode::BAD_REQUEST)?;
        let value = payload
            .warranty_value
            .filter(|v| *v > 0)
            .ok_or(StatusCode::BAD_REQUEST)?;
        if !WARRANTY_TYPES.contains(&kind.as_str()) {
            return Err(StatusCode::BAD_REQUEST);
        }
        (Some(kind), Some(value))
    } else {
        (None, None)
    };

    let row = sqlx::query!(
        r#"INSERT INTO products
               (id, store_id, title, slug, brand_id, description, content, sort_order,
                is_digital, is_active, is_featured, min_order_qty, max_order_qty,
                has_warranty, warranty_type, warranty_value, has_inventory,
                seo_title, seo_description)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
           ON CONFLICT (store_id, slug) DO NOTHING
           RETURNING id, title, slug"#,
        Uuid::new_v4(),
        store_id,
        title,
        slug,
        payload.brand_id,
        payload.description.unwrap_or_default(),
        payload.content.unwrap_or_default(),
        payload.sort_order.unwrap_or(1),
        payload.is_digital.unwrap_or(false),
        payload.is_active.unwrap_or(true),
        payload.is_featured.unwrap_or(false),
        payload.min_order_qty,
        payload.max_order_qty,
        has_warranty,
        warranty_type,
        warranty_value,
        payload.has_inventory.unwrap_or(false),
        clean(payload.seo_title),
        clean(payload.seo_description)
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::CONFLICT)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "id": row.id,
            "title": row.title,
            "slug": row.slug
        })),
    ))
}
