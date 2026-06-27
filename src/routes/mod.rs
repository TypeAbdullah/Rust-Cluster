pub mod auth;
pub mod deployments;
pub mod github;
pub mod projects;
pub mod system;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::auth::middleware::require_auth;
use crate::AppState;

/// Build the complete API router with all routes.
pub fn api_router(state: AppState) -> Router<AppState> {
    // Public routes (no auth needed)
    let public_routes = Router::new()
        .route("/auth/login", post(auth::login))
        .route("/system/health", get(system::health_check))
        // GitHub callback/setup routes (GitHub redirects here — no auth cookie)
        .route("/github/callback", get(github::github_callback))
        .route("/github/setup", get(github::github_setup))
        // GitHub webhook (called by GitHub, not by user)
        .route("/github/webhook", post(github::handle_webhook));

    // Protected routes (require auth)
    let protected_routes = Router::new()
        // Auth
        .route("/auth/me", get(auth::me))
        .route("/auth/logout", post(auth::logout))
        // Projects
        .route("/projects", get(projects::list_projects))
        .route("/projects", post(projects::create_project))
        .route("/projects/{id}", get(projects::get_project))
        .route("/projects/{id}", put(projects::update_project))
        .route("/projects/{id}", delete(projects::delete_project))
        .route("/projects/{id}/deploy", post(projects::deploy_project))
        .route("/projects/{id}/start", post(projects::start_project))
        .route("/projects/{id}/stop", post(projects::stop_project))
        .route("/projects/{id}/restart", post(projects::restart_project))
        .route("/projects/{id}/env", get(projects::get_env_vars))
        .route("/projects/{id}/env", put(projects::update_env_vars))
        // Deployments
        .route("/projects/{id}/deployments", get(deployments::list_deployments))
        .route("/deployments/{id}", get(deployments::get_deployment))
        .route("/deployments/{id}/logs", get(deployments::get_deployment_logs))
        // GitHub App management
        .route("/github/setup/status", get(github::get_setup_status))
        .route("/github/setup/create", post(github::create_github_app))
        .route("/github/setup/delete", delete(github::delete_github_app))
        .route("/github/repos", get(github::list_repos))
        .route("/github/sync", post(github::sync_installations))
        // System
        .route("/system/stats", get(system::get_stats))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    Router::new().merge(public_routes).merge(protected_routes)
}
