use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::db::models::{GitHubAppConfig, GitHubInstallation};
use crate::error::AppError;
use crate::github;
use crate::AppState;

// ── GitHub App Setup (from UI) ────────────────────────────

/// GET /api/github/setup/status — Check if GitHub App is configured
pub async fn get_setup_status(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let config = state.db.get_github_app_config().await?;
    let installations = state.db.list_github_installations().await.unwrap_or_default();

    Ok(Json(json!({
        "configured": config.is_some(),
        "app": config.as_ref().map(|c| json!({
            "app_id": c.app_id,
            "app_name": c.app_name,
            "html_url": c.html_url,
        })),
        "installations": installations.iter().map(|i| json!({
            "installation_id": i.installation_id,
            "account_login": i.account_login,
            "account_type": i.account_type,
            "repos_count": i.repos.len(),
        })).collect::<Vec<_>>(),
    })))
}

/// POST /api/github/setup/create — Generate manifest URL for creating a GitHub App
pub async fn create_github_app(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Check if already configured
    if state.db.get_github_app_config().await?.is_some() {
        return Err(AppError::Conflict(
            "GitHub App is already configured. Delete it first to create a new one.".to_string(),
        ));
    }

    // Generate a random suffix for the app name
    let suffix: String = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let manifest = github::generate_app_manifest(&state.config.app_url, &suffix);

    // The frontend will POST this manifest to GitHub
    Ok(Json(json!({
        "manifest": manifest,
        "github_url": format!("https://github.com/settings/apps/new?state={}", suffix),
        "state": suffix,
    })))
}

/// GET /api/github/callback — GitHub redirects here after app creation
#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
}

pub async fn github_callback(
    State(state): State<AppState>,
    Query(query): Query<CallbackQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Exchange the code for app credentials
    let app_data = github::exchange_manifest_code(&query.code).await?;

    // Store in database
    let config = GitHubAppConfig {
        app_id: app_data.id.to_string(),
        app_name: app_data.name.clone(),
        client_id: app_data.client_id,
        client_secret: app_data.client_secret,
        private_key: app_data.pem,
        webhook_secret: app_data.webhook_secret.unwrap_or_default(),
        html_url: app_data.html_url,
    };

    state.db.save_github_app_config(&config).await?;

    tracing::info!("✅ GitHub App '{}' created and saved (ID: {})", app_data.name, app_data.id);

    // Redirect back to the settings page
    Ok(Redirect::to("/#/settings/github"))
}

/// GET /api/github/setup — GitHub redirects here after app installation
#[derive(Deserialize)]
pub struct SetupQuery {
    installation_id: Option<String>,
    setup_action: Option<String>,
}

pub async fn github_setup(
    State(state): State<AppState>,
    Query(query): Query<SetupQuery>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(installation_id) = &query.installation_id {
        let config = state
            .db
            .get_github_app_config()
            .await?
            .ok_or_else(|| AppError::BadRequest("GitHub App not configured".to_string()))?;

        // Get installation token to fetch repos
        let token = github::get_installation_token(
            &config.app_id,
            &config.private_key,
            installation_id,
        )
        .await?;

        // Fetch accessible repos
        let repos = github::list_installation_repos(&token.token).await.unwrap_or_default();

        // Get the installation account info
        let installs = github::list_app_installations(&config.app_id, &config.private_key)
            .await
            .unwrap_or_default();

        let account = installs
            .iter()
            .find(|i| i.id.to_string() == *installation_id);

        let now = chrono::Utc::now().to_rfc3339();
        let installation = GitHubInstallation {
            installation_id: installation_id.clone(),
            account_login: account
                .map(|a| a.account.login.clone())
                .unwrap_or_else(|| "unknown".to_string()),
            account_type: account
                .map(|a| a.target_type.clone())
                .unwrap_or_else(|| "User".to_string()),
            access_token: token.token,
            token_expires_at: token.expires_at,
            repos,
            created_at: now.clone(),
            updated_at: now,
        };

        state.db.upsert_github_installation(&installation).await?;
        tracing::info!(
            "✅ GitHub installation saved for '{}'",
            installation.account_login
        );
    }

    Ok(Redirect::to("/#/settings/github"))
}

/// DELETE /api/github/setup — Remove GitHub App configuration
pub async fn delete_github_app(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    state.db.delete_github_app_config().await?;
    tracing::info!("🗑️ GitHub App configuration removed");
    Ok(Json(json!({ "message": "GitHub App configuration removed" })))
}

// ── Repo Operations ───────────────────────────────────────

/// GET /api/github/repos — List all accessible repos from all installations
pub async fn list_repos(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let config = state
        .db
        .get_github_app_config()
        .await?
        .ok_or_else(|| AppError::BadRequest("GitHub App not configured. Go to Settings → GitHub.".to_string()))?;

    let installations = state.db.list_github_installations().await?;
    let mut all_repos = Vec::new();

    for install in &installations {
        // Refresh token if expired
        let token = if is_token_expired(&install.token_expires_at) {
            let new_token = github::get_installation_token(
                &config.app_id,
                &config.private_key,
                &install.installation_id,
            )
            .await?;

            // Update token in DB
            let mut updated = install.clone();
            updated.access_token = new_token.token.clone();
            updated.token_expires_at = new_token.expires_at;
            updated.updated_at = chrono::Utc::now().to_rfc3339();
            state.db.upsert_github_installation(&updated).await?;

            new_token.token
        } else {
            install.access_token.clone()
        };

        // Fetch fresh repo list
        match github::list_installation_repos(&token).await {
            Ok(repos) => {
                for repo in repos {
                    all_repos.push(json!({
                        "id": repo.id,
                        "name": repo.name,
                        "full_name": repo.full_name,
                        "clone_url": repo.clone_url,
                        "html_url": repo.html_url,
                        "default_branch": repo.default_branch,
                        "private": repo.private,
                        "owner": install.account_login,
                        "installation_id": install.installation_id,
                    }));
                }
            }
            Err(e) => {
                tracing::warn!("Failed to list repos for {}: {}", install.account_login, e);
            }
        }
    }

    Ok(Json(json!({
        "repos": all_repos,
        "total": all_repos.len(),
    })))
}

/// POST /api/github/sync — Refresh installations and repos
pub async fn sync_installations(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let config = state
        .db
        .get_github_app_config()
        .await?
        .ok_or_else(|| AppError::BadRequest("GitHub App not configured".to_string()))?;

    let installs = github::list_app_installations(&config.app_id, &config.private_key).await?;

    let mut synced = 0;
    for install in installs {
        let install_id = install.id.to_string();
        let token = github::get_installation_token(
            &config.app_id,
            &config.private_key,
            &install_id,
        )
        .await?;

        let repos = github::list_installation_repos(&token.token).await.unwrap_or_default();

        let now = chrono::Utc::now().to_rfc3339();
        let gh_install = GitHubInstallation {
            installation_id: install_id,
            account_login: install.account.login,
            account_type: install.target_type,
            access_token: token.token,
            token_expires_at: token.expires_at,
            repos,
            created_at: now.clone(),
            updated_at: now,
        };

        state.db.upsert_github_installation(&gh_install).await?;
        synced += 1;
    }

    Ok(Json(json!({
        "message": format!("Synced {} installations", synced),
        "synced": synced,
    })))
}

/// POST /api/github/webhook — GitHub webhook handler (push events for auto-deploy)
pub async fn handle_webhook(
    State(state): State<AppState>,
    body: String,
) -> Result<Json<serde_json::Value>, AppError> {
    // Parse the webhook payload
    let payload: serde_json::Value = serde_json::from_str(&body)
        .map_err(|_| AppError::BadRequest("Invalid JSON".to_string()))?;

    // Check if it's a push event
    let ref_field = payload.get("ref").and_then(|r| r.as_str()).unwrap_or("");
    let repo_url = payload
        .get("repository")
        .and_then(|r| r.get("clone_url"))
        .and_then(|u| u.as_str())
        .unwrap_or("");

    if ref_field.is_empty() || repo_url.is_empty() {
        return Ok(Json(json!({ "message": "Ignored: not a push event" })));
    }

    // Extract branch from ref (refs/heads/main → main)
    let branch = ref_field.strip_prefix("refs/heads/").unwrap_or(ref_field);

    // Find matching projects with auto_deploy enabled
    let projects = state.db.list_projects().await?;
    let mut deployed = 0;

    for project in projects {
        if project.auto_deploy
            && !project.repo_url.is_empty()
            && project.repo_url.contains(repo_url.trim_end_matches(".git"))
            && project.branch == branch
        {
            tracing::info!(
                "🚀 Auto-deploying '{}' (push to {})",
                project.name,
                branch
            );

            // Trigger deploy
            let now = chrono::Utc::now().to_rfc3339();
            let deployment = crate::db::models::Deployment {
                id: uuid::Uuid::new_v4().to_string(),
                project_id: project.id.clone(),
                commit_sha: payload
                    .get("after")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string(),
                commit_message: payload
                    .get("head_commit")
                    .and_then(|c| c.get("message"))
                    .and_then(|m| m.as_str())
                    .unwrap_or("")
                    .to_string(),
                status: "pending".to_string(),
                logs: String::new(),
                trigger: "github".to_string(),
                created_at: now,
                finished_at: String::new(),
            };

            state.db.create_deployment(&deployment).await?;

            // Run deploy in background
            let db = state.db.clone();
            let apps_dir = state.config.apps_dir.clone();
            let deploy_id = deployment.id.clone();
            let project_id = project.id.clone();
            let pm = state.process_manager.clone();

            tokio::spawn(async move {
                pm.stop_project(&project_id).await.ok();
                match crate::process::builder::build_and_deploy(&project_id, &deploy_id, db.clone(), &apps_dir).await {
                    Ok(_) => {
                        if let Err(e) = pm.start_project(&project_id).await {
                            tracing::error!("Failed to start after auto-deploy: {}", e);
                        }
                    }
                    Err(e) => tracing::error!("Auto-deploy failed: {}", e),
                }
            });

            deployed += 1;
        }
    }

    Ok(Json(json!({
        "message": format!("Triggered {} auto-deployments", deployed),
        "deployed": deployed,
    })))
}

fn is_token_expired(expires_at: &str) -> bool {
    if expires_at.is_empty() {
        return true;
    }
    match chrono::DateTime::parse_from_rfc3339(expires_at) {
        Ok(exp) => chrono::Utc::now() >= exp,
        Err(_) => true,
    }
}
