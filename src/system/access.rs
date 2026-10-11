use axum::http::StatusCode;
use uuid::Uuid;

use crate::AppState;

#[derive(Clone, Copy)]
pub enum Action {
    View,
    Create,
    Update,
    Delete,
}

impl Action {
    fn as_str(self) -> &'static str {
        match self {
            Action::View => "view",
            Action::Create => "create",
            Action::Update => "update",
            Action::Delete => "delete",
        }
    }
}

pub async fn authorize(
    state: &AppState,
    user_id: Uuid,
    store_id: Uuid,
    resource: &str,
    action: Action,
) -> Result<(), StatusCode> {
    let allowed: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM store_members m
            JOIN stores s ON s.id = m.store_id
            LEFT JOIN store_member_permissions p
                ON p.store_member_id = m.id AND p.resource = $3
            WHERE m.store_id = $1
              AND m.user_id = $2
              AND s.is_active
              AND (
                    m.is_owner
                 OR CASE $4
                        WHEN 'view' THEN COALESCE(p.can_view, FALSE)
                        WHEN 'create' THEN COALESCE(p.can_create, FALSE)
                        WHEN 'update' THEN COALESCE(p.can_update, FALSE)
                        WHEN 'delete' THEN COALESCE(p.can_delete, FALSE)
                        ELSE FALSE
                    END
              )
        )
        "#,
    )
    .bind(store_id)
    .bind(user_id)
    .bind(resource)
    .bind(action.as_str())
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if allowed {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
