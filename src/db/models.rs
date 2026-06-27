use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Projects ──────────────────────────────────────────────

/// A project (bot/app) managed by the platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub repo_url: String,
    pub branch: String,
    pub build_command: String,
    pub start_command: String,
    pub runtime: String,
    pub status: String,
    #[serde(default)]
    pub env_vars: HashMap<String, String>,
    pub auto_deploy: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create a new project.
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub repo_url: String,
    #[serde(default = "default_branch")]
    pub branch: String,
    #[serde(default)]
    pub build_command: String,
    pub start_command: String,
    #[serde(default = "default_runtime")]
    pub runtime: String,
    #[serde(default)]
    pub env_vars: HashMap<String, String>,
    #[serde(default)]
    pub auto_deploy: bool,
}

/// Request to update a project.
#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub repo_url: Option<String>,
    pub branch: Option<String>,
    pub build_command: Option<String>,
    pub start_command: Option<String>,
    pub runtime: Option<String>,
    pub auto_deploy: Option<bool>,
}

/// Request to update env vars.
#[derive(Debug, Deserialize)]
pub struct UpdateEnvVarsRequest {
    pub env_vars: HashMap<String, String>,
}

// ── Deployments ───────────────────────────────────────────

/// A deployment record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    pub id: String,
    pub project_id: String,
    pub commit_sha: String,
    pub commit_message: String,
    pub status: String,
    pub logs: String,
    pub trigger: String,
    pub created_at: String,
    pub finished_at: String,
}

// ── GitHub App ────────────────────────────────────────────

/// GitHub App credentials stored in the settings table.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitHubAppConfig {
    pub app_id: String,
    pub app_name: String,
    pub client_id: String,
    pub client_secret: String,
    pub private_key: String,     // PEM format
    pub webhook_secret: String,
    pub html_url: String,
}

/// A GitHub App installation (an account that installed the app).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubInstallation {
    pub installation_id: String,
    pub account_login: String,
    pub account_type: String,
    pub access_token: String,
    pub token_expires_at: String,
    pub repos: Vec<GitHubRepoInfo>,
    pub created_at: String,
    pub updated_at: String,
}

/// Minimal GitHub repo info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepoInfo {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub clone_url: String,
    pub html_url: String,
    pub default_branch: String,
    pub private: bool,
}

// ── Auth ──────────────────────────────────────────────────

/// Login request.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login response.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
    pub email: String,
}

// ── System ────────────────────────────────────────────────

/// System stats response.
#[derive(Debug, Serialize)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub memory_percent: f32,
    pub uptime_seconds: u64,
    pub total_projects: usize,
    pub running_projects: usize,
    pub total_deployments: usize,
}

fn default_branch() -> String {
    "main".to_string()
}

fn default_runtime() -> String {
    "node".to_string()
}
