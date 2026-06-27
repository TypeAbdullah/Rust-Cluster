use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;
use std::collections::HashMap;

use crate::db::models::{CreateProjectRequest, Deployment, Project, UpdateEnvVarsRequest, UpdateProjectRequest};
use crate::error::AppError;
use crate::AppState;

/// GET /api/projects
pub async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<Vec<Project>>, AppError> {
    let mut projects = state.db.list_projects().await?;

    // Update status based on actual running state
    for project in &mut projects {
        if state.process_manager.is_running(&project.id).await {
            project.status = "running".to_string();
        }
    }

    Ok(Json(projects))
}

/// POST /api/projects
pub async fn create_project(
    State(state): State<AppState>,
    Json(req): Json<CreateProjectRequest>,
) -> Result<Json<Project>, AppError> {
    if req.name.is_empty() {
        return Err(AppError::BadRequest("Project name is required".to_string()));
    }
    if req.start_command.is_empty() {
        return Err(AppError::BadRequest("Start command is required".to_string()));
    }

    let now = chrono::Utc::now().to_rfc3339();
    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name: req.name,
        description: req.description,
        repo_url: req.repo_url,
        branch: req.branch,
        build_command: req.build_command,
        start_command: req.start_command,
        runtime: req.runtime,
        status: "stopped".to_string(),
        env_vars: req.env_vars,
        auto_deploy: req.auto_deploy,
        created_at: now.clone(),
        updated_at: now,
    };

    state.db.create_project(&project).await?;
    tracing::info!("✅ Created project '{}'", project.name);

    Ok(Json(project))
}

/// GET /api/projects/:id
pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut project = state.db.get_project(&id).await?;

    // Update status based on actual running state
    if state.process_manager.is_running(&id).await {
        project.status = "running".to_string();
    }

    let process_info = state.process_manager.get_process_info(&id).await;

    Ok(Json(json!({
        "project": project,
        "process": process_info,
    })))
}

/// PUT /api/projects/:id
pub async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateProjectRequest>,
) -> Result<Json<Project>, AppError> {
    let project = state
        .db
        .update_project(
            &id,
            req.name.as_deref(),
            req.description.as_deref(),
            req.repo_url.as_deref(),
            req.branch.as_deref(),
            req.build_command.as_deref(),
            req.start_command.as_deref(),
            req.runtime.as_deref(),
            req.auto_deploy,
        )
        .await?;

    Ok(Json(project))
}

/// DELETE /api/projects/:id
pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Stop if running
    state.process_manager.stop_project(&id).await.ok();
    state.db.delete_project(&id).await?;

    Ok(Json(json!({ "message": "Project deleted" })))
}

/// POST /api/projects/:id/deploy
pub async fn deploy_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Deployment>, AppError> {
    let _project = state.db.get_project(&id).await?;
    let now = chrono::Utc::now().to_rfc3339();

    let deployment = Deployment {
        id: uuid::Uuid::new_v4().to_string(),
        project_id: id.clone(),
        commit_sha: String::new(),
        commit_message: String::new(),
        status: "pending".to_string(),
        logs: String::new(),
        trigger: "manual".to_string(),
        created_at: now,
        finished_at: String::new(),
    };

    state.db.create_deployment(&deployment).await?;

    // Stop currently running process
    state.process_manager.stop_project(&id).await.ok();

    // Run build in background
    let db = state.db.clone();
    let apps_dir = state.config.apps_dir.clone();
    let deploy_id = deployment.id.clone();
    let project_id = id.clone();
    let pm = state.process_manager.clone();

    tokio::spawn(async move {
        match crate::process::builder::build_and_deploy(&project_id, &deploy_id, db.clone(), &apps_dir).await {
            Ok(_) => {
                // Start the process after successful build
                if let Err(e) = pm.start_project(&project_id).await {
                    tracing::error!("Failed to start project after deploy: {}", e);
                    db.update_deployment(
                        &deploy_id,
                        "failed",
                        &format!("Build succeeded but failed to start: {}", e),
                        Some(&chrono::Utc::now().to_rfc3339()),
                    )
                    .await
                    .ok();
                }
            }
            Err(e) => {
                tracing::error!("Deployment failed: {}", e);
                db.update_deployment(
                    &deploy_id,
                    "failed",
                    &format!("Build failed: {}", e),
                    Some(&chrono::Utc::now().to_rfc3339()),
                )
                .await
                .ok();
            }
        }
    });

    Ok(Json(deployment))
}

/// POST /api/projects/:id/start
pub async fn start_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    state.process_manager.start_project(&id).await?;
    Ok(Json(json!({ "message": "Project started" })))
}

/// POST /api/projects/:id/stop
pub async fn stop_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    state.process_manager.stop_project(&id).await?;
    Ok(Json(json!({ "message": "Project stopped" })))
}

/// POST /api/projects/:id/restart
pub async fn restart_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    state.process_manager.restart_project(&id).await?;
    Ok(Json(json!({ "message": "Project restarted" })))
}

/// GET /api/projects/:id/env
pub async fn get_env_vars(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let project = state.db.get_project(&id).await?;

    // Mask values for security (show first 3 chars only)
    let masked: HashMap<String, String> = project
        .env_vars
        .iter()
        .map(|(k, v)| {
            let masked_val = if v.len() > 3 {
                format!("{}•••", &v[..3])
            } else {
                "•••".to_string()
            };
            (k.clone(), masked_val)
        })
        .collect();

    Ok(Json(json!({
        "env_vars": masked,
        "count": project.env_vars.len(),
    })))
}

/// PUT /api/projects/:id/env
pub async fn update_env_vars(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateEnvVarsRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    state.db.update_project_env_vars(&id, &req.env_vars).await?;
    Ok(Json(json!({
        "message": "Environment variables updated",
        "count": req.env_vars.len(),
    })))
}
