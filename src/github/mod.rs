use serde::{Deserialize, Serialize};
use crate::db::models::GitHubRepoInfo;
use crate::error::AppError;

// ── GitHub App Manifest Flow ──────────────────────────────
// This implements the same flow as Dokploy/Vercel:
// 1. User clicks "Create GitHub App" in Settings
// 2. We generate a manifest and redirect to GitHub
// 3. GitHub creates the app and redirects back with a code
// 4. We exchange the code for the app credentials
// 5. All credentials are stored in the database

/// Generate the GitHub App manifest for the creation flow.
/// See: https://docs.github.com/en/apps/sharing-github-apps/registering-a-github-app-from-a-manifest
pub fn generate_app_manifest(app_url: &str, app_name_suffix: &str) -> serde_json::Value {
    let callback_url = format!("{}/api/github/callback", app_url);
    let webhook_url = format!("{}/api/github/webhook", app_url);
    let setup_url = format!("{}/api/github/setup", app_url);

    // Sanitize app name: only alphanumeric, hyphens, max 34 chars
    let sanitized = app_name_suffix
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();
    let app_name = format!("RustCluster-{}", if sanitized.is_empty() { "app" } else { &sanitized });
    let app_name = if app_name.len() > 34 { &app_name[..34] } else { &app_name };

    serde_json::json!({
        "name": app_name,
        "url": app_url,
        "hook_attributes": {
            "url": webhook_url,
            "active": true
        },
        "redirect_url": callback_url,
        "setup_url": setup_url,
        "setup_on_update": true,
        "public": false,
        "default_permissions": {
            "contents": "read",
            "metadata": "read",
            "pull_requests": "read",
            "statuses": "write"
        },
        "default_events": [
            "push",
            "pull_request",
            "create",
            "repository"
        ]
    })
}

/// Exchange the temporary code from GitHub for full app credentials.
/// This is called after GitHub redirects back to our callback URL.
pub async fn exchange_manifest_code(code: &str) -> Result<GitHubAppCreationResponse, AppError> {
    let client = reqwest::Client::new();

    let url = format!("https://api.github.com/app-manifests/{}/conversions", code);

    let resp = client
        .post(&url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "RustCluster")
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to exchange code: {}", e)))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::Internal(format!(
            "GitHub returned {} when exchanging manifest code: {}",
            status, body
        )));
    }

    let data: GitHubAppCreationResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse GitHub response: {}", e)))?;

    Ok(data)
}

/// Response from GitHub when exchanging a manifest code.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GitHubAppCreationResponse {
    pub id: u64,
    pub slug: String,
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub pem: String,
    pub webhook_secret: Option<String>,
    pub html_url: String,
    pub owner: Option<GitHubOwner>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GitHubOwner {
    pub login: String,
}

// ── Installation Token Generation ─────────────────────────

/// Generate a JWT for authenticating as the GitHub App.
/// This JWT is used to get installation access tokens.
pub fn generate_app_jwt(app_id: &str, private_key_pem: &str) -> Result<String, AppError> {
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

    let now = chrono::Utc::now().timestamp() as usize;

    let claims = serde_json::json!({
        "iat": now - 60,           // issued at (60s in past for clock drift)
        "exp": now + (9 * 60),     // expires in 9 minutes (max 10)
        "iss": app_id,
    });

    let header = Header::new(Algorithm::RS256);
    let key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
        .map_err(|e| AppError::Internal(format!("Invalid private key: {}", e)))?;

    encode(&header, &claims, &key)
        .map_err(|e| AppError::Internal(format!("Failed to generate JWT: {}", e)))
}

/// Get an installation access token from GitHub.
/// This token is used to access repos that the app is installed on.
pub async fn get_installation_token(
    app_id: &str,
    private_key_pem: &str,
    installation_id: &str,
) -> Result<InstallationToken, AppError> {
    let jwt = generate_app_jwt(app_id, private_key_pem)?;

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/app/installations/{}/access_tokens",
        installation_id
    );

    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "RustCluster")
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get installation token: {}", e)))?;

    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::Internal(format!(
            "GitHub token request failed: {}",
            body
        )));
    }

    let token: InstallationToken = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse token response: {}", e)))?;

    Ok(token)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InstallationToken {
    pub token: String,
    pub expires_at: String,
}

// ── Repo Listing ──────────────────────────────────────────

/// List repositories accessible to an installation.
pub async fn list_installation_repos(access_token: &str) -> Result<Vec<GitHubRepoInfo>, AppError> {
    let client = reqwest::Client::new();

    let resp = client
        .get("https://api.github.com/installation/repositories?per_page=100")
        .header("Authorization", format!("token {}", access_token))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "RustCluster")
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list repos: {}", e)))?;

    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::Internal(format!("GitHub API error: {}", body)));
    }

    let data: RepoListResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse repos: {}", e)))?;

    let repos: Vec<GitHubRepoInfo> = data
        .repositories
        .into_iter()
        .map(|r| GitHubRepoInfo {
            id: r.id,
            name: r.name,
            full_name: r.full_name,
            clone_url: r.clone_url.unwrap_or_default(),
            html_url: r.html_url.unwrap_or_default(),
            default_branch: r.default_branch.unwrap_or_else(|| "main".to_string()),
            private: r.private,
        })
        .collect();

    Ok(repos)
}

#[derive(Debug, Deserialize)]
struct RepoListResponse {
    repositories: Vec<GitHubApiRepo>,
}

#[derive(Debug, Deserialize)]
struct GitHubApiRepo {
    id: u64,
    name: String,
    full_name: String,
    clone_url: Option<String>,
    html_url: Option<String>,
    default_branch: Option<String>,
    private: bool,
}

// ── List Installations ────────────────────────────────────

/// List all installations of the GitHub App.
pub async fn list_app_installations(
    app_id: &str,
    private_key_pem: &str,
) -> Result<Vec<AppInstallation>, AppError> {
    let jwt = generate_app_jwt(app_id, private_key_pem)?;

    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.github.com/app/installations")
        .header("Authorization", format!("Bearer {}", jwt))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "RustCluster")
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list installations: {}", e)))?;

    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::Internal(format!("GitHub API error: {}", body)));
    }

    let installs: Vec<AppInstallation> = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse installations: {}", e)))?;

    Ok(installs)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppInstallation {
    pub id: u64,
    pub account: AppInstallationAccount,
    pub target_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppInstallationAccount {
    pub login: String,
    pub avatar_url: Option<String>,
}

/// Clone a repo using an installation token for authentication.
pub async fn clone_with_token(
    token: &str,
    repo_url: &str,
    branch: &str,
    target_dir: &str,
) -> Result<String, AppError> {
    // Convert https://github.com/user/repo to https://x-access-token:TOKEN@github.com/user/repo
    let authed_url = repo_url.replace(
        "https://github.com/",
        &format!("https://x-access-token:{}@github.com/", token),
    );

    let output = tokio::process::Command::new("git")
        .args(["clone", "-b", branch, "--depth", "1", &authed_url, target_dir])
        .output()
        .await
        .map_err(|e| AppError::Internal(format!("git clone failed: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return Err(AppError::Internal(format!("git clone failed: {}", stderr)));
    }

    Ok(format!("{}{}", stdout, stderr))
}

/// Pull latest changes using an installation token.
pub async fn pull_with_token(
    token: &str,
    repo_dir: &str,
    branch: &str,
    repo_url: &str,
) -> Result<String, AppError> {
    let authed_url = repo_url.replace(
        "https://github.com/",
        &format!("https://x-access-token:{}@github.com/", token),
    );

    // Update remote origin URL with refreshed token
    let _ = tokio::process::Command::new("git")
        .args(["remote", "set-url", "origin", &authed_url])
        .current_dir(repo_dir)
        .output()
        .await;

    let output = tokio::process::Command::new("git")
        .args(["pull", "origin", branch])
        .current_dir(repo_dir)
        .output()
        .await
        .map_err(|e| AppError::Internal(format!("git pull failed: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return Err(AppError::Internal(format!("git pull failed: {}", stderr)));
    }

    Ok(format!("{}{}", stdout, stderr))
}
