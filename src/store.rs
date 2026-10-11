use axum::extract::{FromRequestParts, Query, State};
use axum::http::{StatusCode, header, request::Parts};
use axum::response::Html;
use serde::Deserialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::AppState;

#[derive(Clone, FromRow)]
pub struct Store {
    pub id: Uuid,
    pub slug: String,
    pub currency_id: Uuid,
    pub timezone: String,
    pub default_locale: String,
}

pub struct CurrentStore(pub Store);

pub fn host_of(value: &str) -> Option<String> {
    let host = value
        .split("://")
        .last()?
        .split('/')
        .next()?
        .split(':')
        .next()?
        .trim()
        .to_lowercase();
    if host.is_empty() { None } else { Some(host) }
}

async fn by_slug(db: &PgPool, slug: &str) -> Result<Option<Store>, sqlx::Error> {
    sqlx::query_as::<_, Store>(
        "SELECT id, slug, currency_id, timezone, default_locale FROM stores WHERE is_active AND lower(slug) = $1 LIMIT 1",
    )
    .bind(slug)
    .fetch_optional(db)
    .await
}

async fn by_domain(db: &PgPool, host: &str) -> Result<Option<Store>, sqlx::Error> {
    sqlx::query_as::<_, Store>(
        "SELECT id, slug, currency_id, timezone, default_locale FROM stores WHERE is_active AND lower(custom_domain) = $1 AND domain_verified_at IS NOT NULL LIMIT 1",
    )
    .bind(host)
    .fetch_optional(db)
    .await
}

fn is_platform_host(state: &AppState, host: &str) -> bool {
    let frontend = host_of(&state.domain).unwrap_or_default();
    let backend = host_of(&state.store_domain).unwrap_or_default();
    host == frontend || host == backend || host == format!("www.{frontend}") || host == "localhost"
}

async fn resolve(state: &AppState, parts: &Parts) -> Result<Option<Store>, sqlx::Error> {
    let host = parts
        .headers
        .get(header::HOST)
        .and_then(|v| v.to_str().ok())
        .and_then(host_of);

    if let Some(host) = host {
        if !is_platform_host(state, &host) {
            if let Some(store) = by_domain(&state.db, &host).await? {
                return Ok(Some(store));
            }
        }
    }

    if let Some(slug) = parts
        .headers
        .get("x-store")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
    {
        if let Some(store) = by_slug(&state.db, &slug).await? {
            return Ok(Some(store));
        }
    }

    if let Some(origin) = parts
        .headers
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .and_then(host_of)
    {
        if !is_platform_host(state, &origin) {
            if let Some(store) = by_domain(&state.db, &origin).await? {
                return Ok(Some(store));
            }
        }
    }

    Ok(None)
}

impl FromRequestParts<AppState> for CurrentStore {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        resolve(state, parts)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "database error"))?
            .map(CurrentStore)
            .ok_or((StatusCode::NOT_FOUND, "store not found"))
    }
}

pub async fn storefront(CurrentStore(store): CurrentStore) -> Html<String> {
    Html(format!(
        "<!doctype html><html><body>{}</body></html>",
        store.slug
    ))
}

#[derive(Deserialize)]
pub struct TlsQuery {
    domain: String,
}

pub async fn tls_check(State(state): State<AppState>, Query(q): Query<TlsQuery>) -> StatusCode {
    let host = q.domain.trim().to_lowercase();
    match by_domain(&state.db, &host).await {
        Ok(Some(_)) => StatusCode::OK,
        Ok(None) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
