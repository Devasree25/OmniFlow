use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    models::{LoginRequest, LoginResponse, User, UserResponse},
    utils::{jwt::create_jwt, password::verify_password},
    AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Find user by email
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1 AND is_active = true"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify password
    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate JWT
    let token = create_jwt(&user, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        token,
        user: user.into(),
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<UserResponse>, StatusCode> {
    // Registration logic would go here
    // For demo purposes, registration is disabled
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_current_user(
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, StatusCode> {
    // This would extract user from JWT claims in request extensions
    // Placeholder for now
    Err(StatusCode::NOT_IMPLEMENTED)
}
