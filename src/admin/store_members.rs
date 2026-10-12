use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct StoreMember {
    pub id: Uuid,
    pub store_id: Uuid,
    pub store_title: String,
    pub store_slug: String,
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub is_owner: bool,
    pub is_worker: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_store_members(
    State(state): State<AppState>,
) -> Result<Json<Vec<StoreMember>>, StatusCode> {
    let members = sqlx::query_as!(
        StoreMember,
        r#"SELECT
               sm.id,
               sm.store_id,
               s.title AS store_title,
               s.slug AS store_slug,
               sm.user_id,
               u.name,
               u.email,
               sm.is_owner,
               sm.is_worker,
               sm.created_at,
               sm.updated_at
           FROM store_members sm
           JOIN stores s ON s.id = sm.store_id
           JOIN users u ON u.id = sm.user_id
           ORDER BY s.title ASC, sm.is_owner DESC, sm.created_at ASC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(members))
}
