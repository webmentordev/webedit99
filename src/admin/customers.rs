use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CustomerSummary {
    pub id: Uuid,
    pub store_id: Uuid,
    pub store_title: String,
    pub store_slug: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub is_active: bool,
    pub is_banned: bool,
    pub ip_address: Option<String>,
    pub last_login: Option<DateTime<Utc>>,
    pub user_agent: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_customers(
    State(state): State<AppState>,
) -> Result<Json<Vec<CustomerSummary>>, StatusCode> {
    let customers = sqlx::query_as!(
        CustomerSummary,
        r#"SELECT
               c.id,
               c.store_id,
               s.title AS store_title,
               s.slug AS store_slug,
               c.name,
               c.username,
               c.email,
               c.phone,
               c.is_active,
               c.is_banned,
               c.ip_address,
               c.last_login,
               c.user_agent,
               c.verified_at,
               c.created_at,
               c.updated_at
           FROM customers c
           INNER JOIN stores s ON s.id = c.store_id
           WHERE c.deleted_at IS NULL
           ORDER BY c.created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(customers))
}
