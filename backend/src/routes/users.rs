use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{models::{User, UserResponse}, AppState};

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    // Query users - RLS will automatically filter by tenant
    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE is_active = true ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();
    Ok(Json(response))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, StatusCode> {
    // Query single user - RLS ensures tenant isolation
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1 AND is_active = true"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user.into()))
}
