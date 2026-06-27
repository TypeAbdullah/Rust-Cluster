use axum::{extract::State, Json};
use serde_json::json;
use sysinfo::System;

use crate::error::AppError;
use crate::AppState;

/// GET /api/system/health
pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

/// GET /api/system/stats
pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_percent = if total_memory > 0 {
        (used_memory as f64 / total_memory as f64 * 100.0) as f32
    } else {
        0.0
    };

    let projects = state.db.list_projects().await?;
    let total_projects = projects.len();
    let running_projects = state.process_manager.running_count().await;
    let total_deployments = state.db.count_deployments().await?;

    Ok(Json(json!({
        "cpu_usage": sys.global_cpu_usage(),
        "memory_used": used_memory,
        "memory_total": total_memory,
        "memory_percent": memory_percent,
        "total_projects": total_projects,
        "running_projects": running_projects,
        "total_deployments": total_deployments,
        "os": System::long_os_version().unwrap_or_default(),
        "hostname": System::host_name().unwrap_or_default(),
    })))
}
