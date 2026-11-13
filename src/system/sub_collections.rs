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
pub struct ListSubCollectionsQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub is_active: Option<bool>,
    pub collection_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct SubCollectionListItem {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub collection_title: String,
    pub title: String,
    pub slug: String,
    pub is_featured: bool,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubCollection {
    pub collection_id: Uuid,
    pub title: String,
    pub slug: Option<String>,
    pub is_featured: Option<bool>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
}

pub async fn list_sub_collections(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Query(q): Query<ListSubCollectionsQuery>,
) -> Result<Json<Paginated<SubCollectionListItem>>, StatusCode> {
    authorize(
        &state,
        claims.sub,
        store_id,
        "sub_collections",
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
        r#"SELECT sc.id, sc.collection_id, c.title AS collection_title,
                  sc.title, sc.slug, sc.is_featured, sc.is_active, sc.sort_order,
                  sc.created_at, sc.updated_at,
                  COUNT(*) OVER() AS "total!"
           FROM sub_collections sc
           JOIN collections c ON c.id = sc.collection_id
           WHERE sc.store_id = $1
             AND ($2::text IS NULL OR sc.title ILIKE '%' || $2 || '%')
             AND ($3::bool IS NULL OR sc.is_active = $3)
             AND ($4::uuid IS NULL OR sc.collection_id = $4)
           ORDER BY sc.sort_order ASC, sc.created_at DESC
           LIMIT $5 OFFSET $6"#,
        store_id,
        search,
        q.is_active,
        q.collection_id,
        per_page,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = rows.first().map(|r| r.total).unwrap_or(0);

    let data = rows
        .into_iter()
        .map(|r| SubCollectionListItem {
            id: r.id,
            collection_id: r.collection_id,
            collection_title: r.collection_title,
            title: r.title,
            slug: r.slug,
            is_featured: r.is_featured,
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

pub async fn create_sub_collection(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Json(payload): Json<CreateSubCollection>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    authorize(
        &state,
        claims.sub,
        store_id,
        "sub_collections",
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

    let slug = match payload
        .slug
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    {
        Some(s) => slugify(&s, false),
        None => slugify(&title, false),
    };

    if slug.is_empty() || slug.chars().count() > 255 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let collection_exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(
               SELECT 1 FROM collections
               WHERE id = $1 AND store_id = $2 AND deleted_at IS NULL
           ) AS "exists!""#,
        payload.collection_id,
        store_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !collection_exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    let row = sqlx::query!(
        r#"INSERT INTO sub_collections
               (id, store_id, collection_id, title, slug, is_featured, is_active, sort_order)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           ON CONFLICT (store_id, slug) DO NOTHING
           RETURNING id, title, slug"#,
        Uuid::new_v4(),
        store_id,
        payload.collection_id,
        title,
        slug,
        payload.is_featured.unwrap_or(false),
        payload.is_active.unwrap_or(true),
        payload.sort_order.unwrap_or(1)
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
