use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{
    Extension, Json,
    extract::State,
    http::{HeaderMap, StatusCode, header},
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub is_admin: bool,
    #[serde(default)]
    pub ver: i32,
    pub exp: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct UserPublic {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    name: String,
    username: String,
    email: String,
    password_hash: String,
    is_admin: bool,
    is_active: bool,
    is_banned: bool,
    token_version: i32,
    last_login: Option<DateTime<Utc>>,
    verified_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

fn client_info(headers: &HeaderMap) -> (Option<String>, Option<String>) {
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|v| v.trim().chars().take(45).collect::<String>())
        .filter(|v| !v.is_empty());
    let agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.chars().take(512).collect::<String>());
    (ip, agent)
}

pub async fn register_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RegisterUser>,
) -> Result<(StatusCode, Json<UserPublic>), StatusCode> {
    let name = payload.name.trim().to_string();
    let email = payload.email.trim().to_lowercase();

    if name.is_empty()
        || name.len() > 255
        || !email.contains('@')
        || email.len() > 255
        || payload.password.len() < 8
        || payload.password.len() > 128
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let username = payload
        .username
        .as_deref()
        .map(|u| u.trim().to_lowercase())
        .filter(|u| !u.is_empty())
        .unwrap_or_else(|| email.split('@').next().unwrap_or("").to_string());

    if username.is_empty() || username.len() > 255 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            eprintln!("error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .to_string();

    let (ip, agent) = client_info(&headers);

    let mut tx = state.db.begin().await.map_err(|e| {
        eprintln!("error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let existing: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let user = sqlx::query_as::<_, UserPublic>(
        r#"INSERT INTO users (id, name, username, email, password_hash, is_admin, ip_address, user_agent)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           RETURNING id, name, username, email, is_admin, is_active, last_login, verified_at, created_at, updated_at"#,
    )
    .bind(Uuid::new_v4())
    .bind(&name)
    .bind(&username)
    .bind(&email)
    .bind(&password_hash)
    .bind(existing == 0)
    .bind(&ip)
    .bind(&agent)
    .fetch_one(&mut *tx)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        _ => {
            eprintln!("error: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    })?;

    tx.commit().await.map_err(|e| {
        eprintln!("error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginUser>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let email = payload.email.trim().to_lowercase();

    let user = sqlx::query_as::<_, UserRow>(
        r#"SELECT id, name, username, email, password_hash, is_admin, is_active, is_banned,
                  token_version, last_login, verified_at, created_at, updated_at
           FROM users WHERE lower(email) = $1"#,
    )
    .bind(&email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let parsed = PasswordHash::new(&user.password_hash).map_err(|e| {
        eprintln!("error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !user.is_active || user.is_banned {
        return Err(StatusCode::FORBIDDEN);
    }

    let (ip, agent) = client_info(&headers);

    sqlx::query(
        "UPDATE users SET last_login = now(), ip_address = $2, user_agent = $3, updated_at = now() WHERE id = $1",
    )
    .bind(user.id)
    .bind(&ip)
    .bind(&agent)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let claims = Claims {
        sub: user.id,
        is_admin: user.is_admin,
        ver: user.token_version,
        exp: (Utc::now() + Duration::hours(24 * 30)).timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|e| {
        eprintln!("error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let public = UserPublic {
        id: user.id,
        name: user.name,
        username: user.username,
        email: user.email,
        is_admin: user.is_admin,
        is_active: user.is_active,
        last_login: user.last_login,
        verified_at: user.verified_at,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    Ok(Json(serde_json::json!({ "token": token, "user": public })))
}

pub async fn logout_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        "UPDATE users SET token_version = token_version + 1, updated_at = now() WHERE id = $1",
    )
    .bind(claims.sub)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::NO_CONTENT)
}
