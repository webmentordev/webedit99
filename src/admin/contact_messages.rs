use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ContactMessageSummary {
    pub id: Uuid,
    pub store_id: Uuid,
    pub store_title: String,
    pub store_slug: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub subject: Option<String>,
    pub message: String,
    pub is_read: bool,
    pub ip_address: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_contact_messages(
    State(state): State<AppState>,
) -> Result<Json<Vec<ContactMessageSummary>>, StatusCode> {
    let messages = sqlx::query_as!(
        ContactMessageSummary,
        r#"SELECT
               cm.id,
               cm.store_id,
               s.title AS store_title,
               s.slug AS store_slug,
               cm.name,
               cm.email,
               cm.phone,
               cm.subject,
               cm.message,
               cm.is_read,
               cm.ip_address,
               cm.created_at,
               cm.updated_at
           FROM contact_messages cm
           INNER JOIN stores s ON s.id = cm.store_id
           ORDER BY cm.created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(messages))
}
