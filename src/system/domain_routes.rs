use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use uuid::Uuid;

use crate::AppState;
use crate::auth::Claims;
use crate::domain::{VerifyResult, verify_domain};
use crate::system::access::{Action, authorize};

pub async fn verify_domain_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<VerifyResult>, StatusCode> {
    authorize(&state, claims.sub, id, "settings", Action::Update).await?;

    verify_domain(&state, id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
