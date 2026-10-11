use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Currency {
    pub id: Uuid,
    pub currency_code: String,
    pub symbol: String,
    pub decimal_places: i16,
    pub value: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCurrency {
    pub currency_code: String,
    pub symbol: String,
    pub decimal_places: Option<i16>,
    pub value: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCurrency {
    pub currency_code: Option<String>,
    pub symbol: Option<String>,
    pub decimal_places: Option<i16>,
    pub value: Option<Decimal>,
}

fn valid_code(code: &str) -> bool {
    !code.is_empty() && code.chars().count() <= 10
}

fn valid_symbol(symbol: &str) -> bool {
    !symbol.is_empty() && symbol.chars().count() <= 10
}

fn valid_decimals(places: i16) -> bool {
    (0..=8).contains(&places)
}

pub async fn get_currencies(
    State(state): State<AppState>,
) -> Result<Json<Vec<Currency>>, StatusCode> {
    let currencies = sqlx::query_as!(
        Currency,
        r#"SELECT id, currency_code, symbol, decimal_places, value, created_at, updated_at
           FROM currencies
           ORDER BY currency_code ASC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(currencies))
}

pub async fn create_currency(
    State(state): State<AppState>,
    Json(payload): Json<CreateCurrency>,
) -> Result<(StatusCode, Json<Currency>), StatusCode> {
    let code = payload.currency_code.trim().to_uppercase();
    let symbol = payload.symbol.trim().to_string();
    let decimal_places = payload.decimal_places.unwrap_or(2);

    if !valid_code(&code)
        || !valid_symbol(&symbol)
        || !valid_decimals(decimal_places)
        || payload.value <= Decimal::ZERO
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let currency = sqlx::query_as!(
        Currency,
        r#"INSERT INTO currencies (id, currency_code, symbol, decimal_places, value)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, currency_code, symbol, decimal_places, value, created_at, updated_at"#,
        Uuid::new_v4(),
        code,
        symbol,
        decimal_places,
        payload.value
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23514") => {
            StatusCode::BAD_REQUEST
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::CREATED, Json(currency)))
}

pub async fn update_currency(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCurrency>,
) -> Result<Json<Currency>, StatusCode> {
    let code = payload.currency_code.map(|c| c.trim().to_uppercase());
    let symbol = payload.symbol.map(|s| s.trim().to_string());

    if code.as_deref().is_some_and(|c| !valid_code(c))
        || symbol.as_deref().is_some_and(|s| !valid_symbol(s))
        || payload.decimal_places.is_some_and(|p| !valid_decimals(p))
        || payload.value.is_some_and(|v| v <= Decimal::ZERO)
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let currency = sqlx::query_as!(
        Currency,
        r#"UPDATE currencies
           SET currency_code = COALESCE($1, currency_code),
               symbol = COALESCE($2, symbol),
               decimal_places = COALESCE($3, decimal_places),
               value = COALESCE($4, value),
               updated_at = NOW()
           WHERE id = $5
           RETURNING id, currency_code, symbol, decimal_places, value, created_at, updated_at"#,
        code,
        symbol,
        payload.decimal_places,
        payload.value,
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23514") => {
            StatusCode::BAD_REQUEST
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(currency))
}

pub async fn get_currency(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Currency>, StatusCode> {
    let currency = sqlx::query_as!(
        Currency,
        r#"SELECT id, currency_code, symbol, decimal_places, value, created_at, updated_at
           FROM currencies
           WHERE id = $1"#,
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(currency))
}
