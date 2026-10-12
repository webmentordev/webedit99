use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct StoreSummary {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub custom_domain: Option<String>,
    pub currency_id: Uuid,
    pub timezone: String,
    pub default_locale: String,
    pub is_active: bool,
    pub is_banned: bool,
    pub domain_verified_at: Option<DateTime<Utc>>,
    pub contact_email: Option<String>,
    pub contact_phone_number: Option<String>,
    pub whatsapp: Option<String>,
    pub store_address: Option<String>,
    pub favicon_id: Option<Uuid>,
    pub white_logo_id: Option<Uuid>,
    pub dark_logo_id: Option<Uuid>,
    pub members_count: i64,
    pub customers_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_stores(
    State(state): State<AppState>,
) -> Result<Json<Vec<StoreSummary>>, StatusCode> {
    let stores = sqlx::query_as!(
        StoreSummary,
        r#"SELECT
               s.id,
               s.title,
               s.slug,
               s.custom_domain,
               s.currency_id,
               s.timezone,
               s.default_locale,
               s.is_active,
               s.is_banned,
               s.domain_verified_at,
               s.contact_email,
               s.contact_phone_number,
               s.whatsapp,
               s.store_address,
               s.favicon_id,
               s.white_logo_id,
               s.dark_logo_id,
               (SELECT COUNT(*) FROM store_members sm WHERE sm.store_id = s.id) AS "members_count!",
               (SELECT COUNT(*) FROM customers c WHERE c.store_id = s.id AND c.deleted_at IS NULL) AS "customers_count!",
               s.created_at,
               s.updated_at
           FROM stores s
           ORDER BY s.created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(stores))
}
