use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;

use crate::{models::Claims, utils::jwt::decode_jwt, AppState};

/// Middleware to set tenant context in PostgreSQL session
/// This enforces Row-Level Security (RLS) policies
pub async fn tenant_context_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract JWT token from Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    if let Some(token) = auth_header {
        // Decode JWT and extract tenant_id
        match decode_jwt(token, &state.jwt_secret) {
            Ok(claims) => {
                // Set PostgreSQL session variable for RLS
                let tenant_id = claims.tenant_id;
                
                // Execute SET LOCAL to configure tenant context
                // This will be used by RLS policies
                let result = sqlx::query(&format!(
                    "SET LOCAL app.current_tenant_id = '{}'",
                    tenant_id
                ))
                .execute(&state.db)
                .await;

                if result.is_err() {
                    tracing::error!("Failed to set tenant context: {:?}", result);
                }

                // Store claims in request extensions for later use
                request.extensions_mut().insert(claims);
            }
            Err(e) => {
                tracing::warn!("Invalid JWT token: {:?}", e);
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }

    Ok(next.run(request).await)
}

/// Extract claims from request extensions
pub fn get_claims_from_request(request: &Request) -> Option<&Claims> {
    request.extensions().get::<Claims>()
}
