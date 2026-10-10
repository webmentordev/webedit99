use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;

pub async fn get_asset() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "message": "Asset served!" })))
}
