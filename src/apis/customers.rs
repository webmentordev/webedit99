use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{
    Extension, Json,
    extract::{FromRequestParts, Request, State},
    http::{HeaderMap, StatusCode, header},
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::store::CurrentStore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerClaims {
    pub sub: Uuid,
    pub store_id: Uuid,
    pub typ: String,
    pub exp: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CustomerPublic {
    pub id: Uuid,
    pub store_id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct CustomerLogin {
    id: Uuid,
    store_id: Uuid,
    name: String,
    username: String,
    email: String,
    phone: Option<String>,
    verified_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    password_hash: String,
    is_active: bool,
    is_banned: bool,
}

#[derive(Debug, Deserialize)]
pub struct RegisterCustomer {
    pub name: String,
    pub email: String,
    pub password: String,
    pub username: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginCustomer {
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

pub async fn register_customer(
    State(state): State<AppState>,
    CurrentStore(store): CurrentStore,
    headers: HeaderMap,
    Json(payload): Json<RegisterCustomer>,
) -> Result<(StatusCode, Json<CustomerPublic>), StatusCode> {
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
        .map(|u| u.trim().to_string())
        .filter(|u| !u.is_empty())
        .unwrap_or_else(|| email.split('@').next().unwrap_or("").to_string());

    if username.is_empty() || username.len() > 255 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let phone = payload
        .phone
        .as_deref()
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty());

    if phone.as_deref().is_some_and(|p| p.len() > 50) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let (ip, agent) = client_info(&headers);

    let customer = sqlx::query_as::<_, CustomerPublic>(
        r#"INSERT INTO customers (id, store_id, name, username, email, password_hash, phone, ip_address, user_agent)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           RETURNING id, store_id, name, username, email, phone, verified_at, created_at"#,
    )
    .bind(Uuid::new_v4())
    .bind(store.id)
    .bind(&name)
    .bind(&username)
    .bind(&email)
    .bind(&password_hash)
    .bind(&phone)
    .bind(&ip)
    .bind(&agent)
    .fetch_one(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::CREATED, Json(customer)))
}

pub async fn login_customer(
    State(state): State<AppState>,
    CurrentStore(store): CurrentStore,
    headers: HeaderMap,
    Json(payload): Json<LoginCustomer>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let email = payload.email.trim().to_lowercase();

    let customer = sqlx::query_as::<_, CustomerLogin>(
        r#"SELECT id, store_id, name, username, email, phone, verified_at, created_at,
                  password_hash, is_active, is_banned
           FROM customers
           WHERE store_id = $1 AND lower(email) = $2 AND deleted_at IS NULL"#,
    )
    .bind(store.id)
    .bind(&email)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let parsed = PasswordHash::new(&customer.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !customer.is_active || customer.is_banned {
        return Err(StatusCode::FORBIDDEN);
    }

    let (ip, agent) = client_info(&headers);

    sqlx::query(
        "UPDATE customers SET last_login = now(), ip_address = $2, user_agent = $3, updated_at = now() WHERE id = $1",
    )
    .bind(customer.id)
    .bind(&ip)
    .bind(&agent)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let claims = CustomerClaims {
        sub: customer.id,
        store_id: customer.store_id,
        typ: "customer".to_string(),
        exp: (Utc::now() + Duration::hours(24 * 30)).timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let public = CustomerPublic {
        id: customer.id,
        store_id: customer.store_id,
        name: customer.name,
        username: customer.username,
        email: customer.email,
        phone: customer.phone,
        verified_at: customer.verified_at,
        created_at: customer.created_at,
    };

    Ok(Json(
        serde_json::json!({ "token": token, "customer": public }),
    ))
}

pub async fn require_customer(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    let CurrentStore(store) = CurrentStore::from_request_parts(&mut parts, &state)
        .await
        .map_err(|(code, _)| code)?;

    let token = parts
        .headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = decode::<CustomerClaims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?
    .claims;

    if claims.typ != "customer" || claims.store_id != store.id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let valid: bool = sqlx::query_scalar(
        r#"SELECT EXISTS (
               SELECT 1 FROM customers
               WHERE id = $1 AND store_id = $2
                 AND is_active AND NOT is_banned AND deleted_at IS NULL
           )"#,
    )
    .bind(claims.sub)
    .bind(store.id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut req = Request::from_parts(parts, body);
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

pub async fn customer_me(
    State(state): State<AppState>,
    Extension(claims): Extension<CustomerClaims>,
) -> Result<Json<CustomerPublic>, StatusCode> {
    let customer = sqlx::query_as::<_, CustomerPublic>(
        r#"SELECT id, store_id, name, username, email, phone, verified_at, created_at
           FROM customers WHERE id = $1 AND store_id = $2 AND deleted_at IS NULL"#,
    )
    .bind(claims.sub)
    .bind(claims.store_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(customer))
}
