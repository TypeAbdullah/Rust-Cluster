use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::auth::{self, Claims};
use crate::db::models::LoginRequest;
use crate::error::AppError;
use crate::AppState;

/// POST /api/auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate credentials against env vars
    let valid_email = &state.config.admin_email;
    let valid_password = &state.config.admin_password;

    if req.email != *valid_email || req.password != *valid_password {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Create JWT
    let token = auth::create_token(
        &state.config.admin_username,
        &state.config.admin_email,
        &state.config.jwt_secret,
    )?;

    // Set cookie
    let cookie = format!(
        "token={}; HttpOnly; Path=/; Max-Age=86400; SameSite=Lax",
        token
    );

    let body = json!({
        "token": token,
        "username": state.config.admin_username,
        "email": state.config.admin_email,
    });

    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(body),
    ))
}

/// GET /api/auth/me
pub async fn me(claims: Claims) -> Json<serde_json::Value> {
    Json(json!({
        "username": claims.sub,
        "email": claims.email,
    }))
}

/// POST /api/auth/logout
pub async fn logout() -> impl IntoResponse {
    let cookie = "token=; HttpOnly; Path=/; Max-Age=0; SameSite=Lax";
    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(json!({ "message": "Logged out" })),
    )
}
