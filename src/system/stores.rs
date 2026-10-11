use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::AppState;
use crate::auth::Claims;
use crate::system::access::{Action, authorize};
use crate::utils::*;

const RESERVED_SLUGS: [&str; 12] = [
    "www", "api", "admin", "system", "assets", "uploads", "health", "static", "cdn", "app", "mail",
    "shop",
];

const MAX_OWNED_STORES: i64 = 2;

pub async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "message": "Store is healthy!" })),
    )
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MyStore {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub is_active: bool,
    pub is_owner: bool,
    pub is_worker: bool,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StoreDetail {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub custom_domain: Option<String>,
    pub domain_verified_at: Option<DateTime<Utc>>,
    pub domain_verification_token: String,
    pub is_active: bool,
    pub currency_id: Uuid,
    pub timezone: String,
    pub default_locale: String,
    pub cloudflare_turnstile_site_key: Option<String>,
    pub has_turnstile_secret: bool,
    pub contact_email: Option<String>,
    pub contact_phone_number: Option<String>,
    pub whatsapp: Option<String>,
    pub store_address: Option<String>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image_id: Option<Uuid>,
    pub jsonld_title: Option<String>,
    pub jsonld_description: Option<String>,
    pub jsonld_image_id: Option<Uuid>,
    pub keywords: Option<String>,
    pub twitter: Option<String>,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub linkedin: Option<String>,
    pub pinterest: Option<String>,
    pub favicon_id: Option<Uuid>,
    pub white_logo_id: Option<Uuid>,
    pub dark_logo_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateStore {
    pub title: String,
}

pub async fn list_my_stores(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<MyStore>>, StatusCode> {
    let stores = sqlx::query_as!(
        MyStore,
        r#"SELECT s.id, s.title, s.slug, s.is_active,
                  m.is_owner AS "is_owner!", m.is_worker AS "is_worker!"
           FROM store_members m
           JOIN stores s ON s.id = m.store_id
           WHERE m.user_id = $1
           ORDER BY s.created_at DESC"#,
        claims.sub
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(stores))
}

pub async fn get_my_store(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<StoreDetail>, StatusCode> {
    authorize(&state, claims.sub, id, "settings", Action::View).await?;

    let store = sqlx::query_as!(
        StoreDetail,
        r#"SELECT id, title, slug, custom_domain, domain_verified_at, domain_verification_token,
                  is_active, currency_id, timezone, default_locale,
                  cloudflare_turnstile_site_key,
                  (cloudflare_turnstile_secret_key IS NOT NULL) AS "has_turnstile_secret!",
                  contact_email, contact_phone_number, whatsapp, store_address,
                  seo_title, seo_description, og_title, og_description, og_image_id,
                  jsonld_title, jsonld_description, jsonld_image_id, keywords,
                  twitter, facebook, instagram, linkedin, pinterest,
                  favicon_id, white_logo_id, dark_logo_id, created_at, updated_at
           FROM stores
           WHERE id = $1"#,
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(store))
}

pub async fn create_store(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStore>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let title = payload
        .title
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if title.is_empty() || title.chars().count() > 255 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let base = slugify_store(&title);
    let slug = if RESERVED_SLUGS.contains(&base.as_str()) {
        format!("{base}-{}", &Uuid::new_v4().simple().to_string()[..6])
    } else {
        base
    };
    let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());

    let row = sqlx::query!(
        r#"WITH owned AS (
               SELECT COUNT(*) AS c
               FROM store_members
               WHERE user_id = $1 AND is_owner
           ),
           new_store AS (
               INSERT INTO stores
                   (id, title, slug, currency_id, timezone, default_locale, domain_verification_token)
               SELECT $2::uuid, $3::text, $4::text, (SELECT id FROM currencies LIMIT 1),
                      'UTC', 'en', $5::text
               FROM owned
               WHERE c < $6::bigint
               ON CONFLICT DO NOTHING
               RETURNING id, title, slug
           ),
           new_member AS (
               INSERT INTO store_members (id, store_id, user_id, is_owner)
               SELECT $7::uuid, id, $1, TRUE FROM new_store
           )
           SELECT (SELECT c FROM owned) AS "owned!",
                  ns.id AS "id?", ns.title AS "title?", ns.slug AS "slug?"
           FROM (SELECT 1) AS one
           LEFT JOIN new_store ns ON TRUE"#,
        claims.sub,
        Uuid::new_v4(),
        title,
        slug,
        token,
        MAX_OWNED_STORES,
        Uuid::new_v4()
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if row.owned >= MAX_OWNED_STORES {
        return Err(StatusCode::FORBIDDEN);
    }

    match (row.id, row.title, row.slug) {
        (Some(id), Some(title), Some(slug)) => Ok((
            StatusCode::CREATED,
            Json(json!({
                "id": id,
                "title": title,
                "slug": slug,
                "is_owner": true,
                "is_worker": false
            })),
        )),
        _ => Err(StatusCode::CONFLICT),
    }
}
