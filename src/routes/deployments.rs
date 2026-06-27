use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;

use crate::error::AppError;
use crate::AppState;

/// GET /api/projects/:id/deployments
pub async fn list_deployments(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Verify project exists
    state.db.get_project(&id).await?;
    let deployments = state.db.list_deployments(&id).await?;

    Ok(Json(json!({
        "deployments": deployments,
        "total": deployments.len(),
    })))
}

/// GET /api/deployments/:id
pub async fn get_deployment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let deployment = state.db.get_deployment(&id).await?;
    Ok(Json(json!({ "deployment": deployment })))
}

/// GET /api/deployments/:id/logs
pub async fn get_deployment_logs(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let deployment = state.db.get_deployment(&id).await?;
    Ok(Json(json!({
        "logs": deployment.logs,
        "status": deployment.status,
    })))
}
