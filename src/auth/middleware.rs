use axum::{
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
};

use crate::auth::Claims;
use crate::error::AppError;
use crate::AppState;

/// Extract authenticated admin claims from the request.
/// Checks Authorization header first, then falls back to cookie.
impl FromRequestParts<AppState> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let secret = &state.config.jwt_secret;

        // Try Authorization: Bearer <token>
        if let Some(auth_header) = parts.headers.get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    return crate::auth::validate_token(token, secret)
                        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()));
                }
            }
        }

        // Try cookie
        if let Some(cookie_header) = parts.headers.get("cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in cookie_str.split(';') {
                    let cookie = cookie.trim();
                    if let Some(token) = cookie.strip_prefix("token=") {
                        return crate::auth::validate_token(token, secret)
                            .map_err(|_| AppError::Unauthorized("Invalid token".to_string()));
                    }
                }
            }
        }

        Err(AppError::Unauthorized("No authentication token provided".to_string()))
    }
}

/// Auth middleware that rejects unauthenticated requests.
pub async fn require_auth(
    state: axum::extract::State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let secret = &state.config.jwt_secret;

    // Try Authorization header
    let token = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    // Fallback to cookie
    let token = token.or_else(|| {
        request
            .headers()
            .get("cookie")
            .and_then(|h| h.to_str().ok())
            .and_then(|cookies| {
                cookies.split(';').find_map(|c| {
                    let c = c.trim();
                    c.strip_prefix("token=").map(|t| t.to_string())
                })
            })
    });

    match token {
        Some(t) => {
            crate::auth::validate_token(&t, secret)
                .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;
            Ok(next.run(request).await)
        }
        None => Err(AppError::Unauthorized("Authentication required".to_string())),
    }
}
