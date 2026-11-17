use argon2::Argon2;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHasher, SaltString};
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
pub struct ListCustomersQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub is_active: Option<bool>,
    pub is_banned: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CustomerListItem {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub is_active: bool,
    pub is_banned: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCustomer {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
}

pub async fn list_customers(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Query(q): Query<ListCustomersQuery>,
) -> Result<Json<Paginated<CustomerListItem>>, StatusCode> {
    authorize(&state, claims.sub, store_id, "customers", Action::View).await?;

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
        r#"SELECT id, name, username, email, phone, is_active, is_banned,
                  last_login, verified_at, created_at, updated_at,
                  COUNT(*) OVER() AS "total!"
           FROM customers
           WHERE store_id = $1
             AND deleted_at IS NULL
             AND ($2::text IS NULL
                  OR name ILIKE '%' || $2 || '%'
                  OR email ILIKE '%' || $2 || '%'
                  OR username ILIKE '%' || $2 || '%'
                  OR phone ILIKE '%' || $2 || '%')
             AND ($3::bool IS NULL OR is_active = $3)
             AND ($4::bool IS NULL OR is_banned = $4)
           ORDER BY created_at DESC
           LIMIT $5 OFFSET $6"#,
        store_id,
        search,
        q.is_active,
        q.is_banned,
        per_page,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = rows.first().map(|r| r.total).unwrap_or(0);

    let data = rows
        .into_iter()
        .map(|r| CustomerListItem {
            id: r.id,
            name: r.name,
            username: r.username,
            email: r.email,
            phone: r.phone,
            is_active: r.is_active,
            is_banned: r.is_banned,
            last_login: r.last_login,
            verified_at: r.verified_at,
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

pub async fn create_customer(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(store_id): Path<Uuid>,
    Json(payload): Json<CreateCustomer>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    authorize(&state, claims.sub, store_id, "customers", Action::Create).await?;

    let name = payload
        .name
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let username = payload.username.trim().to_string();
    let email = payload.email.trim().to_lowercase();
    let phone = payload
        .phone
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    if name.is_empty()
        || name.chars().count() > 255
        || username.is_empty()
        || username.chars().count() > 255
        || email.chars().count() > 255
        || phone.as_ref().is_some_and(|p| p.chars().count() > 50)
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let valid_email = match email.split_once('@') {
        Some((local, domain)) => {
            !local.is_empty() && domain.contains('.') && !email.contains(char::is_whitespace)
        }
        None => false,
    };

    if !valid_email {
        return Err(StatusCode::BAD_REQUEST);
    }

    let password = payload.password;
    if password.chars().count() < 8 || password.chars().count() > 128 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let password_hash = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row = sqlx::query!(
        r#"INSERT INTO customers
               (id, store_id, name, username, email, password_hash, phone, is_active, verified_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8,
                   CASE WHEN $9::bool THEN now() END)
           ON CONFLICT (store_id, email) DO NOTHING
           RETURNING id, name, email"#,
        Uuid::new_v4(),
        store_id,
        name,
        username,
        email,
        password_hash,
        phone,
        payload.is_active.unwrap_or(true),
        payload.is_verified.unwrap_or(false)
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::CONFLICT)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "id": row.id,
            "name": row.name,
            "email": row.email
        })),
    ))
}
